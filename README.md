# Pantheon Core

Pantheon is an experimental protocol on Solana for vaults that automate common trading patterns. The only vault currently present is the AutoDCA vault, more will follow eventually.

## Smart Contracts

- `autodca` - Automated dollar cost averaging vaults, powered by [Jupiter Exchange](https://www.jup.ag) for best-in-class price execution

## Offchain Programs

- `autodca-accounts` - An API for indexing the necessary data for the iterator (crank) to process DCA schedules in due time
- `iterator-crank` - Serverless functions that both iterate over dca schedules periodically and carry out the transactions as necessary
- `autodca-client` - A CLI tool to invoke the autodca program instructions without having to manually write any code or clients

## On trustlessness in code

As much as we all love applauding DeFi and Web3 for being decentralized, most smart contracts are not actually trustless and the AutoDCA program is no different. Here are some things to keep in mind when interacting with this code:

- There does exist a `CrankAuthority` account that every `DcaMetadata` account must associate with. The `current_authority` of the `CrankAuthority` is the only entity allowed to execute DCA schedules. One can make the argument that this means the `CrankAuthority` is a privileged entity. Not exactly a big deal but probably good to keep in mind.

- Jupiter order routing is too complex to shove into the logic of any single contract instruction. Because of this, the `trigger_dca_payment` instruction that the `CrankAuthority` runs only extracts tokens from the contract's vault, into a token account owned by the crank, after which it is free to run the jupiter route on the funds **outside** of the contracts execution environment, and then return the result of the swap into the vault. Once again, not a big deal at all (since you can see the code that executes all this) but it's easy to see how one could argue that there exists a blackbox where people can be rugged if they were to rely on a malicious `CrankAuthority`. This assertion is 100% correct and should always be kept in mind. Ideally, you run your own crank using the tools provided in this repo. The second best option is to use the hosted crank that I run. There is no third best option, unfortunately.

## Building and running tests
