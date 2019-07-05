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
    pub fn new_mutex() -> Self {
        let mut mutex = Semaphore(UnsafeCell::new(QueueDefinition::new(1, QueueType::Mutex)));
        //        mutex.initialise_mutex();
        mutex
    }

    pub fn initialise_mutex(&self) {
        //traceCREATE_MUTEX!(self);
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(None, 0 as TickType, queueSEND_TO_BACK);
        }
    }

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

    pub fn semaphore_up(&self) -> Result<Option<TaskHandle>, QueueError> {
        unsafe {
            trace!("Semaphore take runs!");
            let inner = self.0.get();
            trace!("Semaphore take get finished!");
            (*inner).queue_generic_receive(semGIVE_BLOCK_TIME, false)
            //trace!("Semaphore take finish!");
        }
    }

    //if you successfully call this function, you own the semaphore
    pub fn semaphore_down(&self, xBlockTime: TickType) -> Result<(), QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(None, xBlockTime, queueSEND_TO_BACK)
        }
    }

    pub fn create_binary() -> Self {
        Semaphore(UnsafeCell::new(QueueDefinition::new(
            1,
            QueueType::BinarySemaphore,
        )))
    }

    pub fn create_counting(max_count: UBaseType /*,initial_count:UBaseType*/) -> Self {
        let mut counting_semphr = Semaphore(UnsafeCell::new(QueueDefinition::new(
            max_count,
            QueueType::CountingSemaphore,
        )));
        unsafe {
            let inner = counting_semphr.0.get();
            (*inner).initialise_count(0);
            //            (*inner).initialise_count(initial_count);
        }
        //traceCREATE_COUNTING_SEMAPHORE!();
        counting_semphr
    }

    //#[cfg(feature = "configUSE_RECURSIVE_MUTEXES")]
    pub fn create_recursive_mutex() -> Self {
        Semaphore(UnsafeCell::new(QueueDefinition::new(
            1,
            QueueType::RecursiveMutex,
        )))
    }

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

    pub fn get_recursive_count(&self) -> UBaseType {
        unsafe {
            let inner = self.0.get();
            (*inner).get_recursive_count()
        }
    }
}
