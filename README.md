# Pantheon Core

Pantheon is an experimental protocol on Solana for vaults that automate common trading patterns. The only vault currently present is the AutoDCA vault, more will follow eventually.

## Programs

- `autodca` - Automated dollar cost averaging vaults, powered by [Jupiter Exchange](https://www.jup.ag) for best-in-class price execution
- `autodca-accounts` - An API for indexing the necessary data for the iterator (crank) to process DCA schedules in due time
- `iterator-crank` - TBD
- `autodca-client` - A CLI tool to invoke the autodca program instructions without having to manually write any code or clients

## Building and running tests
