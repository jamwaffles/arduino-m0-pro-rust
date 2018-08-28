//! Install deps, start OpenOCD, use examples to run code
//!
//! ```
//! rustup override set nightly
//! rustup target add thumbv6m-none-eabi
//! apt install -y openocd
//! ./ocd.sh
//! # In another shell
//! cargo run --release --example blink
//! ```

#![no_std]
