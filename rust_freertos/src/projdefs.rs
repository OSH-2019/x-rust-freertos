// projdefs.rs - Basic (maybe useless) constant definitions.
use crate::port::BaseType;

pub const pdTrue: BaseType = 1;
pub const pdFalse: BaseType = 0;

pub const pdPass: BaseType = pdTrue;
pub const pdFail: BaseType = pdFalse;

#[macro_export]
macro_rules! pdMS_TO_TICKS {
    ($xTimeInMs:expr) => {
        (($xTimeInMs as crate::port::TickType * configTICK_RATE_HZ!()) / 1000 as crate::port::TickType) as crate::port::TickType
    };
}
