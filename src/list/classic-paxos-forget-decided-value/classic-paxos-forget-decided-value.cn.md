## Paxos: (Trap): The Bug in Paxos Made Simple

这不是一个 bug, 但人们往往会认为它是一个 bug 。

#### 问题描述：

```
1. P1 向 AB 发送 'prepare 1'
2. AB 都回复 P1，承诺不接受任何编号小于 1 的请求。
   现在的状态是：A(-:-,1) B(-:-,1) C(-:-,-)
3. P1 收到回复后，进程卡主，运行非常缓慢
4. P2 向 AB 发送 'prepare 100'
5. AB 都回复 P2，承诺不接受任何编号小于 100 的请求。
   现在的状态是：A(-:-,100) B(-:-,100) C(-:-,-)
6. P2 收到回复后，选择一个值 b，并向 BC 发送 'accept 100:b'
7. BC 接收并接受了接受请求，状态为：A(-:-,100) B(100:b,100) C(100:b,-)。
   注意，提案 100:b 已被选择。
8. P1 恢复，选择一个值 a，并向 BC 发送 'accept 1:a'
9. B 不接受它，但 C 接受它，因为 C 从未承诺过任何事情。
   C 只保存最后接受的提案，所以 1:a 覆盖了 100:b。
   状态为：A(-:-,100) B(100:b,100) C(1:a,-)。
10. P3 向 AC 发送 'prepare 101'。A 没有已接受的提案，C 报告 1:a。
    P3 选择值 a，并向 AC 发送 'accept 101:a'，AC 都接受了它。
    状态为：A(101:a,101) B(100:b,100) C(101:a,101)。
    现在 101:a 被选定，而 100:b 在第 7 步已被选定。Paxos 失败。
```

#### 解释：

上面的执行假设 acceptor 只保存最后接受的提案。
Paxos Made Simple 则要求 acceptor 记住它接受过的编号最大的提案。这样 C 在第 9 步仍保留 100:b，P3 在第 10 步会选择 b。

每次 accept 都覆盖已接受提案的实现，需要另一条规则，而第 7 步漏掉了它：
当 C 处理 `accept 100:b` 时，它将其状态设置为 `C(100:b,100)`。
**通过接受一个值，节点也承诺不接受较早的值。**

两条规则保持同一个不变式：acceptor 报告的已接受提案，编号只增不减。


遗憾的是：

> 此外，我查看了几个专有和开源的 Paxos 实现，它们**都有这个错误**！


**参考资料**：

- [Marc Brooker 的博客](https://brooker.co.za/blog/2021/11/16/paxos.html)
- [在 stackoverflow 上](https://stackoverflow.com/questions/29880949/contradiction-in-lamports-paxos-made-simple-paper)
