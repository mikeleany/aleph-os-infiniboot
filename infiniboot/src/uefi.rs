/*************************************************************************\
 *                                                                       *
 *  This Source Code Form is subject to the terms of the Mozilla Public  *
 *  License, v. 2.0. If a copy of the MPL was not distributed with this  *
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.             *
 *                                                                       *
\*************************************************************************/
//! UEFI-specific code.
#![cfg(target_os = "uefi")]
use log::info;
use rlibc as _;
use uefi::prelude::*;
use uefi::logger::Logger;

/// Infiniboot's entry point.
///
/// # Safety
/// This function must only be called by the UEFI, thus preventing it from being called twice.
#[no_mangle]
unsafe fn efi_main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    static mut LOGGER: Option<Logger> = None;

    // SAFETY:
    // `Logger::new`: `LOGGER` will be disabled before exiting boot services
    // use of mutable static: `LOGGER` is only used mutably in `efi_main` and its safety contract
    // guarantees that it is only called once
    log::set_logger(unsafe {
        LOGGER.insert(Logger::new(system_table.stdout()))
    }).expect("logger shouldn't be set before now");
    log::set_max_level(log::LevelFilter::Trace);

    info!("Infiniboot v{}", "0.1.0");

    todo!("load the kernel");
    todo!("SAFETY: ensure `LOGGER` and any other code that uses boot services is disabled");
    todo!("exit boot services");
}

#[path = "panic.rs"]
mod panic;
