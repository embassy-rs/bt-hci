[![crates.io][crates-badge]][crates-url] [![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/bt-hci-driver
[crates-url]: https://crates.io/crates/bt-hci-driver
[docs-badge]: https://docs.rs/bt-hci-driver/badge.svg
[docs-url]: https://docs.rs/bt-hci-driver

# bt-hci-driver

This crate contains the driver trait necessary for adding [`bt-hci`](https://crates.io/crates/bt-hci) support
for a new hardware platform that uses a HCI-compatible controller.

If you want to *use* `bt-hci` with already made drivers, you should depend on the main `bt-hci` crate, not on this crate.

If you are writing a driver, you  should depend only on this crate, not on the main `bt-hci` crate.
This will allow your driver to continue working for newer `bt-hci` major versions, without needing an update,
if the driver trait has not had breaking changes.

## License

bt-hci is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
