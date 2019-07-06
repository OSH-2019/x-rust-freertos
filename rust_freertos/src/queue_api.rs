// queue_api.rs, queue APIs
// This file is created by Ning Yuting.
// To solve the issue of mutability of queue.

use crate::port::*;
use crate::queue::*;
use crate::queue_h::*;
use std::cell::UnsafeCell;

/// * Description:
///
/// Implemente interior mutability for queue so that queue can be shared among threads as immutable
/// inference.
/// It is safe to use lots of unsafe codes here because we implemente synchronous blocking for
/// queue.
///
/// * Implemented by:Ning Yuting
pub struct Queue<T>(UnsafeCell<QueueDefinition<T>>)
where
    T: Default + Clone;

// send, sync is used for sharing queue among threads
unsafe impl<T: Default + Clone> Send for Queue<T> {}
unsafe impl<T: Default + Clone> Sync for Queue<T> {}

impl<T> Queue<T>
where
    T: Default + Clone,
{
    /*some APIs in queue.h */

    /// # Description:
    /// Create a new queue.
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 186
    ///
    /// # Arguments:
    /// * `length` - The maximum number of items that the queue can contain.
    ///
    /// # Return:
    /// The created queue.
    pub fn new(length: UBaseType) -> Self {
        Queue(UnsafeCell::new(QueueDefinition::new(
            length,
            QueueType::Base,
        )))
    }

    /// # Description
    /// Post an item to the front of a queue.
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation: queue.h 521
    ///
    /// # Argument
    /// * `pvItemToQueue` - the item that is to be placed on the queue.
    /// * `xTicksToWait` - The maximum amount of time the task should block waiting for space to 
    /// become available on the queue, should it already be full.
    ///
    /// # Return
    /// Ok() if the item was successfully posted, otherwise errQUEUE_FULL.
    pub fn send(&self, pvItemToQueue: T, xTicksToWait: TickType) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(pvItemToQueue, xTicksToWait, queueSEND_TO_BACK)
        }
    }

    /// # Description
    /// Post an item to the front of a queue.
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 355
    ///
    /// # Argument
    /// * `pvItemToQueue` - the item that is to be placed on the queue.
    /// * `xTicksToWait` - The maximum amount of time the task should block waiting for space to
    /// become available on the queue, should it already be full.
    /// 
    /// # Return
    /// Ok() if the item was successfully posted, otherwise errQUEUE_FULL.
    pub fn send_to_front(
        &self,
        pvItemToQueue: T,
        xTicksToWait: TickType,
    ) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(pvItemToQueue, xTicksToWait, queueSEND_TO_FRONT)
        }
    }

    /// # Description
    /// Post an item to the back of a queue.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 437
    ///
    /// # Argument
    /// * `pvItemToQueue` - the item that is to be placed on the queue.
    /// * `xTicksToWait` - The maximum amount of time the task should block waiting for space to 
    /// become available on the queue, should it already be full.
    /// 
    /// # Return
    /// Ok() if the item was successfully posted, otherwise errQUEUE_FULL.
    pub fn send_to_back(&self, pvItemToQueue: T, xTicksToWait: TickType) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(pvItemToQueue, xTicksToWait, queueSEND_TO_BACK)
        }
    }

    /// # Description
    /// Only for use with queues that have a length of one - so the queue is either empty or full.
    /// Post an item on a queue.If the queue is already full then overwrite the value held in the queue.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 604
    ///
    /// # Argument
    /// * `pvItemToQueue` - the item that is to be place on the queue.
    ///
    /// # Return
    /// Ok() is the only value that can be returned because queue_overwrite will write to the
    /// queue even when the queue is already full.
    pub fn overwrite(&self, pvItemToQueue: T) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(pvItemToQueue, 0, queueOVERWRITE)
        }
    }

    /// # Description
    /// Post an item to the front of a queue. It is safe to use this function from within an interrupt service routine.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 1129
    ///
    /// # Argument
    /// * `pvItemToQueue - the item that is to be placed on the queue.
    ///
    /// # Return
    /// * `Result` -Ok() if the data was successfully sent to the queue, otherwise errQUEUE_FULL.
    /// * `bool` - pxHigherPriorityTaskWoken is changed to be a return value. it is true if sending to the
    /// queue caused a task to unblock,otherwise it is false.
    pub fn send_to_front_from_isr(&self, pvItemToQueue: T) -> (Result<(), QueueError>, bool) {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send_from_isr(pvItemToQueue, queueSEND_TO_FRONT)
        }
    }

    /// # Description
    /// Post an item to the back of a queue. It is safe to use this function from within an interrupt service routi    ne.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 1200
    ///
    /// # Argument
    /// * `pvItemToQueue - the item that is to be placed on the queue.
    /// # Return
    /// * `Result` -Ok() if the data was successfully sent to the queue, otherwise errQUEUE_FULL.
    /// * `bool` - pxHigherPriorityTaskWoken is changed to be a return value. it is true if sending to the
    /// queue caused a task to unblock,otherwise it is false.
    pub fn send_to_back_from_isr(&self, pvItemToQueue: T) -> (Result<(), QueueError>, bool) {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send_from_isr(pvItemToQueue, queueSEND_TO_BACK)
        }
    }

    /// # Description
    /// A version of xQueueOverwrite() that can be used in an interrupt service routine (ISR).
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 1287
    ///
    /// # Argument
    /// * `pvItemToQueue - the item that is to be placed on the queue.
    /// # Return
    /// * `Result` -Ok().
    /// * `bool` - pxHigherPriorityTaskWoken is changed to be a return value. it is true if sending to the
    /// queue caused a task to unblock,otherwise it is false.
    pub fn overwrite_from_isr(&self, pvItemToQueue: T) -> (Result<(), QueueError>, bool) {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send_from_isr(pvItemToQueue, queueOVERWRITE)
        }
    }

    /// # Description
    /// Receive an item from a queue.
    /// The item is received by copy and is returned by Ok(T);
    /// Successfully received items are removed from the queue.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 913
    ///
    /// # Argument
    /// * `xTicksToWait` - The maximum amount of time the task should block 
    /// waiting for an item to receive should the queue be empty at the time
    /// of the call.It will return immediately if xTicksToWait is zero and the queue is empty.
    /// 
    /// # Return
    /// Ok(T) if an item was successfully received from the queue, otherwise QueueError::QueueEmpty.
    pub fn receive(&self, xTicksToWait: TickType) -> Result<T, QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_receive(xTicksToWait, false)
        }
    }

    /// # Description
    /// Receive an item from a queue without removing the item from the queue.
    /// The item is received by copy and is returned by Ok(T);
    /// Successfully received items remain on the queue.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 787
    ///
    /// # Argument
    /// * `xTicksToWait` - The maximum amount of time the task should block 
    /// waiting for an item to receive should the queue be empty at the time of the call.
    /// 
    /// # Return
    /// Ok(T) if an item was successfully received from the queue, otherwise
    /// QueueError::QueueEmpty.
    pub fn peek(&self, xTicksToWait: TickType) -> Result<T, QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_receive(xTicksToWait, true)
        }
    }
}
