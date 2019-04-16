use crate::port::{BaseType, TickType};

pub const pdTrue: BaseType = 1;
pub const pdFalse: BaseType = 0;

pub const pdPass: BaseType = pdTrue;
pub const pdFail: BaseType = pdFalse;

#[macro_export]
macro_rules! pdMS_TO_TICKS {
    ($xTimeInMs:ident) => {
        (($xTimeInMs as port::TickType * configTICK_RATE_HZ!()) / 1000 as port::TickType) as port::TickType
    };
}
