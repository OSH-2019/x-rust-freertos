// projdefs.rs - Basic (maybe useless) constant definitions.
use crate::port::BaseType;

pub const pdTRUE: BaseType = 1;
pub const pdFALSE: BaseType = 0;

pub const pdPASS: BaseType = pdTRUE;
pub const pdFAIL: BaseType = pdFALSE;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FreeRtosError {
    OutOfMemory,
    Timeout,
    TaskNotFound,
    ProcessorHasShutDown,
    DeadLocked,
    PortError
}

#[macro_export]
macro_rules! pdMS_TO_TICKS {
    ($xTimeInMs:expr) => {
        (($xTimeInMs as crate::port::TickType * configTICK_RATE_HZ!()) / 1000 as crate::port::TickType) as crate::port::TickType
    };
}
