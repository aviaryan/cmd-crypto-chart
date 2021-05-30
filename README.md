# CMD Crypto Chart

Commandline program to show cryptocurrency charts on demand.

### GIF

![GIF Preview](https://user-images.githubusercontent.com/4047597/120103274-7d6f3180-c16c-11eb-8aaf-eb9e5b1a611e.gif)

### Development

```sh
cargo build
# for compiling and running all at once
cargo run
# for formatting code
cargo fmt
```

### Using

```sh
cmd_crypto_chart [coinName]
# examples
cmd_crypto_chart bitcoin
cmd_crypto_chart ethereum
```

### Distribution

```sh
cargo build --release
./target/release/cmd_crypto_chart
```
