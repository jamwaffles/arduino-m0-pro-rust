# Arduino M0 Pro Rust

Uses a HAL based on the [Metro M0 board](https://www.adafruit.com/product/3505). Some pins might be different or inaccessible.

## Setup

```bash
# First time setup
rustup override set nightly
rustup target add thumbv6m-none-eabi
apt install -y openocd
echo $(pwd) >> ~/.gdbinit

# In one shell
./ocd.sh

# In another shell
cargo run --release --example blink

# When you see the (gdb) prompt, hit `c` to continue
```

> Note: You may need to fix Linux USB permissions