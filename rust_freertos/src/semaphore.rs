use std::cell::UnsafeCell;
use crate::queue::*;
use crate::port::*;
use crate::queue_h::*;
use crate::task_control::*;
use crate::*;

pub struct Semaphore(UnsafeCell<QueueDefinition<Option<TaskHandle>>>);
unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

impl Semaphore {
    
    pub fn new_mutex( ) -> Self {
        let mut mutex = Semaphore(UnsafeCell::new(QueueDefinition::new(1,QueueType::Mutex)));
        mutex.initialise_mutex();
        mutex
    }

    pub fn initialise_mutex(&self) {
        //traceCREATE_MUTEX!(self);
        unsafe{
            let inner = self.0.get();
            (*inner).queue_generic_send(None,0 as TickType, queueSEND_TO_BACK);
        }
    }

    #[cfg(all(feature = "configUSE_MUTEXES", feature = "INCLUDE_xSemaphoreGetMutexHolder"))]
    pub fn get_mutex_holder(&self) -> Option<task_control::TaskHandle> {
        let mut mutex_holder: Option<task_control::TaskHandle>;
        taskENTER_CRITICAL!();
        {
            unsafe {
                let inner = self.0.get();
                mutex_holder = (*inner).queue_generic_receive(0,true).unwrap();
            }
        }
        taskEXIT_CRITICAL!();
        mutex_holder
    }

    pub fn counting_semaphore_up(&self) -> Result<Option<TaskHandle>, QueueError> {
        unsafe {
            trace!("Counting Semaphore up runs!");
            let inner = self.0.get();
            trace!("Counting Semaphore up get finished!");
            (*inner).queue_generic_receive(semGIVE_BLOCK_TIME, false)
            //trace!("Semaphore take finish!");
        }
    }

    pub fn semaphore_take(&self,xBlockTime: TickType) -> Result<Option<TaskHandle>,QueueError> {
        unsafe {
            trace!("Semaphore take runs!");
            let inner = self.0.get();
            trace!("Semaphore take get finished!");
            (*inner).queue_generic_receive(xBlockTime, false)
            //trace!("Semaphore take finish!");
        }
    }

    pub fn counting_semaphore_down(&self, xBlockTime: TickType) -> Result<(), QueueError> {
        let current_task = get_current_task_handle!();
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(Some(current_task), xBlockTime, queueSEND_TO_BACK)
        }
    }


    pub fn semaphore_give(&self) -> Result<(),QueueError> {
        unsafe {
            let inner = self.0.get();
            (*inner).queue_generic_send(None,semGIVE_BLOCK_TIME,queueSEND_TO_BACK)
        }
    }

    pub fn create_binary () -> Self {
        Semaphore(UnsafeCell::new(QueueDefinition::new(1,QueueType::BinarySemaphore)))
    }
    
    pub fn create_counting(max_count: UBaseType/*,initial_count:UBaseType*/) -> Self {
        let mut counting_semphr = Semaphore(UnsafeCell::new(QueueDefinition::new(max_count,QueueType::CountingSemaphore)));
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
        Semaphore(UnsafeCell::new(QueueDefinition::new(1,QueueType::RecursiveMutex)))
    }

    pub fn give_recursive(&self) -> bool {
        unsafe {
            let inner = self.0.get();
            if  (*inner).transed_task_handle_for_mutex().unwrap().clone() == get_current_task_handle!() {
                traceGIVE_MUTEX_RECURSIVE!(*inner);
                (*inner).QueueUnion_decrease();
                if (*inner).is_QueueUnion_zero() {
                    (*inner).queue_generic_send(None,queueMUTEX_GIVE_BLOCK_TIME, queueSEND_TO_BACK);
                }
                else {
                    mtCOVERAGE_TEST_MARKER!();
                }
                return true;
            }
            else {
                traceGIVE_MUTEX_RECURSIVE_FAILED!(*inner);
                return false;
            }
        }
    }

    pub fn take_recursive(&self, ticks_to_wait:TickType) -> bool {
        let mut xReturn:bool = false;
        unsafe {
            let inner = self.0.get();
            traceTAKE_MUTEX_RECURSIVE!(*inner);
            if  (*inner).transed_task_handle_for_mutex().unwrap().clone() == get_current_task_handle!() {
                (*inner).QueueUnion_increase();
                xReturn = false;
            }
            else {
                match (*inner).queue_generic_receive(ticks_to_wait, false) {
                    Ok(x) => {
                        (*inner).QueueUnion_increase();
                        xReturn = true;
                    }
                    Err(x) => {
                        traceTAKE_MUTEX_RECURSIVE_FAILED!(*inner);
                        xReturn = false;
                    }
                }
            }
        }
        return xReturn;
    }

}
