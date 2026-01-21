#![doc = include_str!("../README.md")]
#![no_std]
use cyw43::{aligned_bytes, Aligned, A4};

/// CYW43xx SoC WiFi firmware blob
///
/// # Important
/// Please note the licensing information from the [README.md](../README.md#license), as well as the licensing
/// information from <https://github.com/georgerobotics/cyw43-driver/tree/main/firmware>.
#[cfg(feature = "wifi")]
pub const CYW43_43439A0: &Aligned<A4, [u8]> = aligned_bytes!("../dist/cyw43-firmware/43439A0.bin");

/// CYW43xx SoC WiFi firmware country location matrix blob
///
/// # Important
/// Please note the licensing information from the [README.md](../README.md#license), as well as the licensing
/// information from <https://github.com/georgerobotics/cyw43-driver/tree/main/firmware>.
#[cfg(feature = "wifi")]
pub const CYW43_43439A0_CLM: &Aligned<A4, [u8]> =
    aligned_bytes!("../dist/cyw43-firmware/43439A0_clm.bin");

/// CYW43xx SoC bluetooth firmware blob
///
/// # Important
/// Please note the licensing information from the [README.md](../README.md#license), as well as the licensing
/// information from <https://github.com/georgerobotics/cyw43-driver/tree/main/firmware>.
#[cfg(feature = "bluetooth")]
pub const CYW43_43439A0_BTFW: &Aligned<A4, [u8]> =
    aligned_bytes!("../dist/cyw43-firmware/43439A0_btfw.bin");
