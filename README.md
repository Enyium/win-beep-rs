Causes a beeping sound on Windows with a given frequency and duration ([used API](https://learn.microsoft.com/en-us/windows/win32/api/utilapiset/nf-utilapiset-beep)). Blocks the thread while beeping. System sounds must not be muted for the beep to be audible.

# Usage

```rust
win_beep::beep_with_hz_and_millis(1000, 100);
```

# License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
