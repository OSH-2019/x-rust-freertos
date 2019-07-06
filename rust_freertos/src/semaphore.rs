use crate::port::*;
use crate::queue::*;
use crate::queue_h::*;
use crate::task_control::*;
use crate::*;
use std::cell::UnsafeCell;

pub struct Semaphore(UnsafeCell<QueueDefinition<Option<TaskHandle>>>);
unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

impl Semaphore {
    /// # Descrpition
    /// Create a new mutex type semaphore instance.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.c 504-515
    ///
    /// # Arguments:
    /// Nothing
    ///
    /// # Return:
    /// The created mutex.
    pub fn new_mutex() -> Self {
        Semaphore(UnsafeCell::new(QueueDefinition::new(1, QueueType::Mutex)))
    }

    /// # Description
    /// Get the mutex holder.
    ///
    /// * Implemented by:Lei Siqi
    ///
    /// # Arguments:
    /// Nothing
    ///
    /// # Return:
    /// `Option<task_control::TaskHandle>` - the holder of the mutex
    #[cfg(all(
        feature = "configUSE_MUTEXES",
        feature = "INCLUDE_xSemaphoreGetMutexHolder"
    ))]
    pub fn get_mutex_holder(&self) -> Option<task_control::TaskHandle> {
        let mut mutex_holder: Option<task_control::TaskHandle>;
        taskENTER_CRITICAL!();
        {
            unsafe {
                let inner = self.0.get();
                mutex_holder = (*inner).queue_generic_receive(0, true).unwrap();
            }
        }
        taskEXIT_CRITICAL!();
        mutex_holder
    }

    /// # Description
    /// Release a semaphore.
    ///
    /// * Implemented by:Ning Yuting & Lei Siqi
    /// * C implementation:semphr.h 489 
    ///
    /// # Arguments:
    /// Nothing
    /// 
    /// # Return:
    /// Ok(T) if the semaphore was released, otherwise QueueError::QueueEmpty.
    pub fn semaphore_up(&self) -> Result<Option<TaskHandle>, QueueError> {
        unsafe {
            trace!("Semaphore up runs!");
            let inner = self.0.get();
            trace!("Semaphore up get finished!");
            (*inner).queue_generic_receive(semGIVE_BLOCK_TIME, false)
        }
    }

    /// # Description
    /// Obtain a semaphore.
    ///
    /// * Implemented by:Ning Yuting & Lei Siqi
    /// * C implementation:semphr.h 331
    ///
    /// # Arguments:
    /// `xBlockTime` - The time in ticks to wait for the semaphore to become available.
    /// A block time of zero can be used to poll the semaphore.
    /// A block time of portMAX_DELAY can be used to block indefinitely.
    ///
    /// # Return:
    /// Ok() if the semaphore was obtained, otherwise errQUEUE_FULL.
    pub fn semaphore_down(&self, xBlockTime: TickType) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(None, xBlockTime, queueSEND_TO_BACK)
        }
    }

    /// # Description
    /// Create a binary semaphore.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:semphr.h 135-144
    ///
    /// # Arguments:
    /// Nothing
    ///
    /// # Return:
    /// The created binary semaphore.
    pub fn create_binary() -> Self {
        Semaphore(UnsafeCell::new(QueueDefinition::new(
            1,
            QueueType::BinarySemaphore,
        )))
    }

    /// # Description
    /// Create a counting semaphore.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:semphr.h 1039-1041
    ///
    /// # Arguments:
    /// `max_count` - The maximum count value that can be reached. When the semaphore reaches 
    /// this value it can no longer be 'given'.
    ///
    /// # Return
    /// The created counting semaphore.
    pub fn create_counting(max_count: UBaseType /*,initial_count:UBaseType*/) -> Self {
        let mut counting_semphr = Semaphore(UnsafeCell::new(QueueDefinition::new(
            max_count,
            QueueType::CountingSemaphore,
        )));
        unsafe {
            let inner = counting_semphr.0.get();
            (*inner).initialise_count(0);
        }
        //traceCREATE_COUNTING_SEMAPHORE!();
        counting_semphr
    }

    /// # Description
    /// Created a recursive mutex.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:semphr.h 886-888
    ///
    /// # Argument
    /// Nothing
    ///
    /// # Return
    /// The created recursive mutex.
    pub fn create_recursive_mutex() -> Self {
        Semaphore(UnsafeCell::new(QueueDefinition::new(
            1,
            QueueType::RecursiveMutex,
        )))
    }

    /// # Description
    /// Release a recursive mutex.
    ///
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.c 570-622
    ///
    /// # Arguments:
    /// Nothing
    /// 
    /// # Return
    /// `bool` - true if the recursive mutex was released.
    pub fn up_recursive(&self) -> bool {
        unsafe {
            let inner = self.0.get();
            if (*inner).transed_task_handle_for_mutex().unwrap().clone()
                == get_current_task_handle!()
            {
                traceGIVE_MUTEX_RECURSIVE!(*inner);
                (*inner).QueueUnion_decrease();
                if (*inner).is_QueueUnion_zero() {
                    (*inner).queue_generic_receive(semGIVE_BLOCK_TIME, false);
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
                return true;
            } else {
                traceGIVE_MUTEX_RECURSIVE_FAILED!(*inner);
                return false;
            }
        }
    }

    /// # Description
    /// Obtain a recursive mutex.
    ///
    /// * Implemented by:Ning Yuting & Lei Siqi
    /// * C implementation:queue.c 625-664
    ///
    /// # Arguments:
    /// `ticks_to_wait` - The time in ticks to wait for the semaphore to become available.
    /// A block time of zero can be used to poll the semaphore.
    ///
    /// # Return:
    /// `bool` - true if the recursive mutex was obtained.
    pub fn down_recursive(&self, ticks_to_wait: TickType) -> bool {
        let mut xReturn: bool = false;
        unsafe {
            let inner = self.0.get();
            traceTAKE_MUTEX_RECURSIVE!(*inner);
            trace!("Ready to get recursive mutex holder");
            let mutex_holder = (*inner).transed_task_handle_for_mutex();
            trace!("Get recursive mutex holder successfully");
            if mutex_holder.is_some()
            {
                if mutex_holder.unwrap().clone() == get_current_task_handle!() {
                    trace!("Not First Time get this mutex");
                    (*inner).QueueUnion_increase();
                    xReturn = false;
                }
            } 
            // else {
                trace!("First Time get this mutex");
                match (*inner).queue_generic_send(None, ticks_to_wait, queueSEND_TO_BACK) {
                    Ok(x) => {
                        (*inner).QueueUnion_increase();
                        xReturn = true;
                    }
                    Err(x) => {
                        traceTAKE_MUTEX_RECURSIVE_FAILED!(*inner);
                        xReturn = false;
                    }
                }
            // }
        }
        return xReturn;
    }

    /// # Description
    /// Get the recursive count of a recursive mutex.
    ///
    /// * Implemented by:Lei Siqi
    ///
    /// # Arguments:
    /// Nothing
    ///
    /// # Return:
    /// `UBaseType` - the recursive count of the recursive mutex.
    pub fn get_recursive_count(&self) -> UBaseType {
        unsafe {
            let inner = self.0.get();
            (*inner).get_recursive_count()
        }
    }
}
