#![no_std]
mod timer;

use mork_common::mork_kernel_log;
use mork_hal::trap::{InterruptCause, TrapCause};
use mork_kernel_state::KernelSafeAccessData;
use crate::timer::handle_timer_interrupt;

pub fn handle_interrupt(kernel_state: &mut KernelSafeAccessData, cause: TrapCause) {
    match cause {
        TrapCause::Interrupt(inner_cause) => {
            // mork_kernel_log!(debug, "inner_cause: {:?}", inner_cause);
            match inner_cause {
                InterruptCause::SupervisorTimer => {
                    handle_timer_interrupt(kernel_state);
                }
                _ => {
                    mork_kernel_log!(error, "unsupported interrupt {:#?}", inner_cause);
                    todo!("mask interrupt");
                }
            }
        }
        _ => {
            panic!("unhandled trap {:?}", cause);
        }
    }
}