<img src="https://pbs.twimg.com/profile_banners/1533492603076808705/1659046799/1500x500"></img>

# Pantheon Core

This protocol is currently a WIP. Breaking changes are likely to occur

Pantheon is a protocol on Solana for vaults that automate common trading patterns, onchain. The only vault currently present is the AutoDCA vault, more will follow eventually. Nothing has been professionally audited, please use at your own risk.

## Smart Contracts

- `autodca` - Automated dollar cost averaging vaults, powered by [Jupiter Exchange](https://www.jup.ag/infra) for best-in-class price execution

## Offchain Programs

- `accounts-api` - An API for indexing smart contract data into a MySQL database
- `schdeuler` - A binary run on a crontab cycle that reads data from the MySQL database and schedules the necessary DCA swaps when they're due
- `executor` - An API that the scheduler communicates with when it needs to execute DCA swaps using [Jupiter Exchange](https://www.jup.ag/infra)
- `autodca-client` - A CLI tool to invoke the autodca program instructions without having to manually write any code or clients

## Building and running tests

```bash
git clone https://github.com/RohanKapurDEV/pantheon-core && cd pantheon-core/
npm install && anchor build

# Running tests
anchor test
```

## License

Pantheon Core is licensed under Apache 2.0.
