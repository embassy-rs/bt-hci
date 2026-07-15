//! HCI transport layers [📖](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host-controller-interface.html)
//!
//! Transport implementations live in separate crates:
//! [`bt-hci-serial`](https://crates.io/crates/bt-hci-serial),
//! [`bt-hci-linux`](https://crates.io/crates/bt-hci-linux) and
//! [`bt-hci-usb`](https://crates.io/crates/bt-hci-usb).

pub use bt_hci_transport::{Transport, WithIndicator};

pub mod blocking {
    //! Blocking transport trait.
    pub use bt_hci_transport::blocking::Transport;
}
