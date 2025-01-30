1. run cargo run --bin extract_packages this generates a json with all packages and its dependencies
2. run cargo run --bin create_git_command this generates the git checkout command
3. clone
- git clone --filter=blob:none --sparse https://github.com/anza-xyz/agave.git agave-solana-svm
- cd agave-solana-svm
4. run git checkout
copy git command in sparse_checkout_command.sh

5. build
run cargo build --lib --package solana-core

Note: This is a work in progress. In order to build the package successfully I removed dependencies from the Cargo.toml members. 
Next step would be to check if (some of) these dependencies are required for working with the package or if they can be removed.
For example for solana-svm I commented out:
# "accounts-bench",
# "accounts-cluster-bench",
# "banking-bench",
# "bench-streamer",
    # "bench-tps",
    # "bench-vote",
# "cargo-registry",
# "cli",
# "client-test",
# "core",
# "dos",
# "install",
    # "keygen",
# "ledger-tool",
    # "local-cluster",
    # "log-analyzer",
# "memory-management",
    # "merkle-root-bench",
# "net-shaper",
# "platform-tools-sdk/cargo-build-sbf",
    # "platform-tools-sdk/cargo-test-sbf",
    # "platform-tools-sdk/gen-headers",
# "poh-bench",
    # "programs/address-lookup-table-tests",
    # "programs/bpf-loader-tests",
    # "programs/compute-budget-bench",
    # "programs/ed25519-tests",
    # "programs/stake-tests",
    # "programs/zk-token-proof-tests",
    # "rbpf-cli",
    # "rpc-test",
# "stake-accounts",
    # "test-validator",
    # "thread-manager",
    # "tokens",
    # "tps-client",
    # "transaction-dos",
    # "upload-perf",
    # "validator",
    # "vortexor",
    # "watchtower",