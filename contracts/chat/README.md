Chat Smart Contract
==================

A [smart contract] is written in [Rust]. It implements immutable chat that's stored in blockchain.

The [smart contract] provided here enable users to chat. Common chat very similar to Slack `general` channel where members can send messages, and it guarantees that it will never be changed. Contract can be used like text channel and chat at the same time. Any NEAR account can be added to whitelist during contract initialization. Only whitelisted users can execute `call` contract methods.

This version of contract doesn't support update and delete `call` for whitelist and messages. And the main thing is app powered by the NEAR protocol, so identity tools are built in.


### Key benefits
1. dApp is easy to use
2. messages are immutable
3. chat looks like text channel for anyone (chat is visible through blockchain)


[Rust]: https://www.rust-lang.org/
[smart contract]: https://docs.near.org/docs/develop/contracts/overview
