## Paxos: (Trap): The Bug in Paxos Made Simple

This is not a bug but people tend to interpret it in the wrong way.

#### The issue:

```
1. P1 sends 'prepare 1' to AB
2.  Both AB respond P1 with a promise to not to accept any request numbered smaller than 1.\
    Now the status is: A(-:-,1) B(-:-,1) C(-:-,-)
3.  P1 receives the responses, then gets stuck and runs very slowly
4.  P2 sends 'prepare 100' to AB
5.  Both AB respond P2 with a promise to not to accept any request numbered smaller than 100.
    Now the status is: A(-:-,100) B(-:-,100) C(-:-,-)
6.  P2 receives the responses, chooses a value b and sends 'accept 100:b' to BC
7.  BC receive and accept the accept request, the status is: A(-:-,100) B(100:b,100) C(100:b,-).
    Note that proposal 100:b has been chosen.
8.  P1 resumes, chooses value a and sends 'accept 1:a' to BC
9.  B doesn't accept it, but C accepts it because C has never promise anything.
    Status is: A(-:-,100) B(100:b,100) C(1:a,-). The chosen proposal is abandon, Paxos fails.
```

#### Explanation:

Missed something in step 7.
When C processes `accept 100:b` it sets its state to `C(100:b,100)`.
**By accepting a value the node is also promising to not accept earlier values.**


Sadly:

> What's more I looked through several proprietary and open-source paxos
> implementations and they **all had the bug submitted by the OP**!


**References**:

- [Marc Brooker's blog](https://brooker.co.za/blog/2021/11/16/paxos.html)
- [On stackoverflow](https://stackoverflow.com/questions/29880949/contradiction-in-lamports-paxos-made-simple-paper)
