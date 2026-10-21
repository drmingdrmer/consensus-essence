## Paxos: (Trap): The Bug in Paxos Made Simple

This is not a bug but people tend to interpret it in the wrong way.

#### The issue:

```
1.  P1 sends 'prepare 1' to AB
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
    C keeps only its last accepted proposal, so 1:a replaces 100:b.
    Status is: A(-:-,100) B(100:b,100) C(1:a,-).
10. P3 sends 'prepare 101' to AC. A reports no accepted proposal, and C reports 1:a.
    P3 chooses value a and sends 'accept 101:a' to AC. Both accept it.
    Status is: A(101:a,101) B(100:b,100) C(101:a,101).
    Now 101:a is chosen, but 100:b was chosen in step 7. Paxos fails.
```

#### Explanation:

The execution assumes that an acceptor keeps only its last accepted proposal.
Paxos Made Simple instead requires an acceptor to remember the highest-numbered
proposal it has accepted. Then C would keep 100:b in step 9, and P3 would
choose b in step 10.

An implementation that overwrites its accepted proposal on every accept needs
another rule, which is missed in step 7.
When C processes `accept 100:b` it sets its state to `C(100:b,100)`.
**By accepting a value the node is also promising to not accept earlier values.**

Both rules keep the same invariant: the accepted proposal that an acceptor
reports never moves to a lower number.


Sadly:

> What's more I looked through several proprietary and open-source paxos
> implementations and they **all had the bug submitted by the OP**!


**References**:

- [Marc Brooker's blog](https://brooker.co.za/blog/2021/11/16/paxos.html)
- [On stackoverflow](https://stackoverflow.com/questions/29880949/contradiction-in-lamports-paxos-made-simple-paper)
