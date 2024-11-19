## Deployment

1. Generate keypair:

```bash
solana-keygen new --outfile ./program.json --force
```

2. Build:

```bash
cargo build-bpf
```

3. Deploy:

```bash
solana program deploy --program-id ./program.json ./target/deploy/spp_program.so
```
