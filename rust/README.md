# Example API Redeem on Rust

This is just an example of how to use the Prakerja Redeem API on Rust; for further use, please modify it as needed.

## Usage
Please add CLIENT_CODE, SECRET_KEY & REDEEM_CODE as environment variable like below or export it in your .bashrc file

```bash
CLIENT_CODE=<your-client-code> SECRET_KEY=<your-secret-key> REDEEM_CODE=<your-redeem-code> cargo run
```

If you want to change host and endpoint please refer to the `src/main.rs` file

## Testing

To test signature generator lib please run:

```bash
cargo test --lib
```

