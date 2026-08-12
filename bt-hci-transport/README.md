[![crates.io][crates-badge]][crates-url] [![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/bt-hci-transport
[crates-url]: https://crates.io/crates/bt-hci-transport
[docs-badge]: https://docs.rs/bt-hci-transport/badge.svg
[docs-url]: https://docs.rs/bt-hci-transport

# bt-hci-transport

This crate contains the transport trait necessary for adding [`bt-hci`](https://crates.io/crates/bt-hci) support
for a new hardware platform that uses a HCI-compatible controller.

If you want to *use* `bt-hci` with already made transports, you should depend on the main `bt-hci` crate, not on this crate.

If you are writing a transport, you should depend only on this crate, not on the main `bt-hci` crate.
This will allow your transport to continue working for newer `bt-hci` major versions, without needing an update,
if the transport trait has not had breaking changes.

## License

bt-hci is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
