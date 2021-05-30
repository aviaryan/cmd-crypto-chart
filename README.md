# CMD Crypto Chart

Commandline program to show cryptocurrency charts on demand.

### GIF

![GIF preview](https://user-images.githubusercontent.com/4047597/120102591-63801f80-c169-11eb-90c2-644a49195d2e.gif)

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
