Router Smart Contract
==================

A [smart contract] is written in [Rust]. It implements accounts cache that's stored in blockchain. In context of chats contracts it'll store their account ids. Only owner can execute `call` contract methods.

This version of contract support add and delete chat methods. Preferably, next release should implement factory contract for chats using callbacks and cross-contract calls.


[Rust]: https://www.rust-lang.org/
[smart contract]: https://docs.near.org/docs/develop/contracts/overview
