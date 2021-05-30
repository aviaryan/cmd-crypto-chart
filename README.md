# CMD Crypto Chart

Commandline program to show cryptocurrency charts on demand. Project created to learn Rust.

### GIF

![GIF Preview](https://user-images.githubusercontent.com/4047597/120104224-ff615980-c170-11eb-87bb-42e1f43be5e6.gif)

### Development

```sh
# for compiling and running all at once
cargo run
# for formatting code
cargo fmt
```

### Using

```sh
cmd_crypto_chart [coin=bitcoin]
# examples
cmd_crypto_chart
cmd_crypto_chart bitcoin
cmd_crypto_chart ethereum
```

### Distribution

```sh
cargo build --release
./target/release/cmd_crypto_chart
```
