# Pinocchio Fundraiser (practice)

Empty stubs + teaching comments. **You type the code.**

1. Open **[PRACTICE.md](./PRACTICE.md)** — when / how / why for every pattern  
2. Follow the build order bottom-up  
3. `cargo check` after each layer  

```bash
cargo check
# Mollusk loads the ELF from target/deploy — rebuild after program changes
cargo build-sbf --features bpf-entrypoint
cargo test -p pinocchio-fundraiser
```
