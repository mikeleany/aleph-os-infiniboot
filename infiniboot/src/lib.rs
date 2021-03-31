/*************************************************************************\
 *                                                                       *
 *  This Source Code Form is subject to the terms of the Mozilla Public  *
 *  License, v. 2.0. If a copy of the MPL was not distributed with this  *
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.             *
 *                                                                       *
\*************************************************************************/
//! Infiniboot is a bootloader specifically designed for loading the Aleph Operating System.
#![no_std]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unwrap_used)]
#![deny(safe_packed_borrows)]
#![deny(unsafe_op_in_unsafe_fn)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/mikeleany/aleph-os/main/images/aleph-os.png"
)]
