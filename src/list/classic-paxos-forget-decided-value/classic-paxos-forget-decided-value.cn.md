## Paxos：（陷阱）Paxos Made Simple 中的错误

这不是一个错误，但人们往往会以错误的方式解释它。

#### 问题：

```
1. P1 向 AB 发送 'prepare 1'
2. AB 都回复 P1，承诺不接受任何编号小于 1 的请求。
   现在的状态是：A(-:-,1) B(-:-,1) C(-:-,-)
3. P1 收到回复后，陷入困境，运行非常缓慢
4. P2 向 AB 发送 'prepare 100'
5. AB 都回复 P2，承诺不接受任何编号小于 100 的请求。
   现在的状态是：A(-:-,100) B(-:-,100) C(-:-,-)
6. P2 收到回复后，选择一个值 b，并向 BC 发送 'accept 100:b'
7. BC 接收并接受了接受请求，状态为：A(-:-,100) B(100:b,100) C(100:b,-)。
   注意，提案 100:b 已被选择。
8. P1 恢复，选择一个值 a，并向 BC 发送 'accept 1:a'
9. B 不接受它，但 C 接受它，因为 C 从未承诺过任何事情。
   状态为：A(-:-,100) B(100:b,100) C(1:a,-)。选择的提案被放弃，Paxos 失败。
```

#### 解释：

在第 7 步中漏掉了一些内容。
当 C 处理 `accept 100:b` 时，它将其状态设置为 `C(100:b,100)`。
**通过接受一个值，节点也承诺不接受较早的值。**


遗憾的是：

> 此外，我查看了几个专有和开源的 Paxos 实现，它们**都有 OP 提交的这个错误**！


**参考资料**：

- [Marc Brooker 的博客](https://brooker.co.za/blog/2021/11/16/paxos.html)
- [在 stackoverflow 上](https://stackoverflow.com/questions/29880949/contradiction-in-lamports-paxos-made-simple-paper)