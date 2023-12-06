# AntiVPN-API-Rust
This is a Rust implementation of the IsItBad API, currently hosted on https://funkemunky.cc.

Git: https://github.com/funkemunky/AntiVPN-API-Rust/ 

## Usage
You can provide either an IPv4 or IPv6 to check.

### Add crate to your project
```bash
cargo add isitbad_api
```

### Checking an IP for a Proxy
```rust
use isitbad_api::get_ip_info;

fn main() {
    let result = match get_ip_info("192.168.1.1".to_string()) {
        Ok(ip_info) => ip_info,
        Err(failed_response) => {
            panic!("Failed to get IP information: {}", failed_response.reason);
        }
    };

    println!("Is {} a VPN: {}", result.ip, result.proxy);
}
```

**Cli Output:**
```bash
dawson@dawsons-desktop-fedora:~/Dev/Rust/RustExampleAPI$ cargo run
   Compiling rust_example_api v0.1.0 (/home/dawson/Dev/Rust/RustExampleAPI)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/rust_example_api`
Is 192.168.1.1 a VPN: true
```
