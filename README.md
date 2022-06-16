# Pantheon Core

This protocol is currently a WIP. Breaking changes are likely to occur

Pantheon is a protocol on Solana for vaults that automate common trading patterns, onchain. The only vault currently present is the AutoDCA vault, more will follow eventually. Nothing has been professionally audited, please use at your own risk.

## Smart Contracts

- `autodca` - Automated dollar cost averaging vaults, powered by [Jupiter Exchange](https://www.jup.ag) for best-in-class price execution

## Offchain Programs

- `lambda-sst` - Serverless functions that allow users to index DCA schedules into a MySQL database and automagically execute them when they're due
- `autodca-client` - A CLI tool to invoke the autodca program instructions without having to manually write any code or clients


## Building and running tests

```bash
git clone https://github.com/RohanKapurDEV/pantheon-core && cd pantheon-core/
npm install && anchor build

# Running tests
anchor test
```

## On trustlessness in code

As much as we all love applauding DeFi and Web3 for being decentralized, most smart contracts are not actually trustless, and the AutoDCA program is no different. Here are some things to keep in mind when interacting with this code:

- There does exist a `CrankAuthority` account that every `DcaMetadata` account must associate with. The `current_authority` of the `CrankAuthority` is the only entity allowed to execute DCA schedules for any given `DcaMetadata`. One can make the argument that this means the `CrankAuthority` is a privileged entity. Not exactly a big deal but probably good to keep in mind.

- Jupiter v2 order routing is too complex to shove into the logic of any single contract instruction. Because of this, the `trigger_dca_payment` instruction that the `CrankAuthority` runs only extracts tokens from the contract's vault, into a token account owned by the current authority of the crank. After which the crank authority is free to run the jupiter route on the funds **outside** of the contracts execution context, and then return the result of the swap into the vault. Once again, not a big deal at all (since you can read the code that executes this entire series of actions, and this entire sequence of withdraw -> swap -> deposit is meant to be 1 atomic transaction that is verifiable onchain) but it's easy to see how one could argue that there exists a moment where people can be rugged if they were to rely on a malicious `CrankAuthority`. This assertion is 100% correct and should always be kept in mind. If you are particularly paranoid, you should run your own crank (+ `CrankAuthority`) and manage positions manually using the tools provided in this repo. The second best option is to use the hosted infra and tooling from Pantheon, accessible via our frontend website. There is no third best option, unfortunately.
