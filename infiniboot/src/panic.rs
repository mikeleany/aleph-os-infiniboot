/*************************************************************************\
 *                                                                       *
 *  This Source Code Form is subject to the terms of the Mozilla Public  *
 *  License, v. 2.0. If a copy of the MPL was not distributed with this  *
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.             *
 *                                                                       *
\*************************************************************************/
//! A panic handler for infiniboot
use core::panic::PanicInfo;
use log::error;

/// Replacement for the `std` panic handler, since it can't be used with
/// `no_std` binaries.
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {
        //TODO: halt the processor
    }
}
