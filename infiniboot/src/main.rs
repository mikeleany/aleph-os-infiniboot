/*************************************************************************\
 *                                                                       *
 *  This Source Code Form is subject to the terms of the Mozilla Public  *
 *  License, v. 2.0. If a copy of the MPL was not distributed with this  *
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.             *
 *                                                                       *
\*************************************************************************/
//! The bootloader for the Aleph Operating System
#![cfg_attr(target_os = "uefi", no_std, no_main)]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unwrap_used)]
#![deny(safe_packed_borrows)]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(abi_efiapi)]
#![feature(once_cell)]
#![feature(option_insert)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/mikeleany/aleph-os/main/images/aleph-os.png"
)]

#[cfg(target_os = "uefi")]
mod uefi;
