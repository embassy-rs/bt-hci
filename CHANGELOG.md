# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.0 - 2025-04-07

* feat: add a way to get the acl header of an acl data by @lulf in https://github.com/embassy-rs/bt-hci/pull/37
* Add missing fields to `LeExtAdvReport` by @korbin in https://github.com/embassy-rs/bt-hci/pull/39
* Added support for serialization of param using serde by @blueluna in https://github.com/embassy-rs/bt-hci/pull/36
* Add 1 to the size of WriteHci-serialized byte slices by @korbin in https://github.com/embassy-rs/bt-hci/pull/38
* Make `secondary_adv_phy` optional for LeExtAdvReport by @korbin in https://github.com/embassy-rs/bt-hci/pull/40
* Add `LeSetExtAdvParamsV2` from BLE5.4 by @korbin in https://github.com/embassy-rs/bt-hci/pull/41

## 0.2.1 - 2025-02-20

- Allow either v0.3.x or v0.4.x versions of `embassy-time`.
- Fix parsing of boolean values in controller-to-host packets.
- Fix parsing of Events when the data slice extends past the end of the event params.

## 0.2.0 - 2025-01-02

- Update `embassy-time` from v0.3 to v0.4.
