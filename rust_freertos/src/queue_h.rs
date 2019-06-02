use crate::port::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum QueueError {
    QueueSendTimeout,
    QueueReceiveTimeout,
    MutexTimeout,
    QueueFull,
    QueueEmpty
}

pub const queueSEND_TO_BACK: BaseType = 0 ;
pub const queueSEND_TO_FRONT:BaseType = 1;
pub const queueOVERWRITE:BaseType = 2;

pub const queueQUEUE_TYPE_BASE:u8 = 0;
pub const queueQUEUE_TYPE_SET:u8 = 0;
pub const queueQUEUE_TYPE_MUTEX:u8 = 1;
pub const queueQUEUE_TYPE_COUNTING_SEMAPHORE:u8 = 2;
pub const queueQUEUE_TYPE_BINARY_SEMAPHORE:u8 = 3;
pub const queueQUEUE_TYPE_RECURSIVE_MUTEX:u8 = 4;

