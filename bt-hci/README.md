[![crates.io][crates-badge]][crates-url] [![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/bt-hci
[crates-url]: https://crates.io/crates/bt-hci
[docs-badge]: https://docs.rs/bt-hci/badge.svg
[docs-url]: https://docs.rs/bt-hci

# bt-hci

Rust types for the Bluetooth HCI (Host Controller Interface) specification, and traits for implementing the `Controller` part of the interface.

See [Trouble](https://github.com/embassy-rs/trouble) for an example of using this crate.

## Bluetooth UUIDs

The bluetooth specification includes [reference information](https://bitbucket.org/bluetooth-SIG/public/src/main/) for pre-defined UUIDs that can be used to communicate specific services, characteristics, properties, etc of a device.  These are also made available as constants from this crate through the [uuid module](./src/uuid/) for users of this crate.

For crate maintainers, to update these constants run the [update_uuids](./update_uuids/) binary, which will redownload the bluetooth-sig yaml spec and rebuild the uuids module based on the latest version.

## License

bt-hci is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
