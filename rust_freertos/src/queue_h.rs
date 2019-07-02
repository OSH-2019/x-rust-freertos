use crate::port::*;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum QueueError {
    QueueSendTimeout,
    QueueReceiveTimeout,
    MutexTimeout,
    QueueFull,
    QueueEmpty
}

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
          QueueError::QueueSendTimeout => write!(f, "QueueSendTimeOut"),
          QueueError::QueueReceiveTimeout => write!(f, "QueueReceiveTimeOut"),
          QueueError::MutexTimeout => write!(f, "MutexSendTimeOut"),
          QueueError::QueueFull => write!(f, "QueueFull"),
          QueueError::QueueEmpty => write!(f, "QueueEmpty"),
        }
    }
}

pub const queueSEND_TO_BACK: BaseType = 0 ;
pub const queueSEND_TO_FRONT:BaseType = 1;
pub const queueOVERWRITE:BaseType = 2;

pub const semGIVE_BLOCK_TIME: TickType = 0;

#[derive(PartialEq)]
pub enum QueueType {
    Base,
    Set,
    Mutex,
    CountingSemaphore,
    BinarySemaphore,
    RecursiveMutex
}
impl Default for QueueType{
   fn default() -> Self {QueueType::Base}
 }

