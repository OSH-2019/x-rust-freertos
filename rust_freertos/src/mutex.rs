use crate::queue::*;
use crate::task_control::*;
use crate::queue_h::*;
use crate::*;

#[derive(Default)]
#[cfg(feature = "configUSE_MUTEXES")]
pub struct Mutex(Queue<Option<TaskHandle>>);

#[cfg(feature = "configUSE_MUTEXES")]
impl Mutex {
    /// # Description
    /// 
    /// # Argument
    /// 
    /// # Return
    fn new() -> Self {
        Mutex::mutex_create()
    }



    /// # Description
    /// 
    /// # Argument
    /// 
    /// # Return
    #[cfg(feature = "configUSE_MUTEXES")]
    fn initialise_mutex(&self) {        
        //self.0.ucQueueType = QueueType::Mutex;
        //self.0.QueueUnion = 0; //original uxRecursiveCallCount

        traceCREATE_MUTEX!(self);

        self.0.send_to_back(None, 0);
    }

    /// # Description
    /// 
    /// # Argument
    /// 
    /// # Return
    #[cfg(all(feature = "configUSE_MUTEXES", feature = "configSUPPORT_DYNAMIC_ALLOCATION"))]
    fn mutex_create() -> Self {
        let mut mutex: Mutex = Mutex(Queue::new_type(1, QueueType::Mutex));
        mutex.initialise_mutex();
        mutex
    }

    /// # Description
    /// Note:  This is a good way of determining if the
	///	calling task is the mutex holder, but not a good way of determining the
	///	identity of the mutex holder, as the holder may change between the
	///	following critical section exiting and the function returning
    /// # Argument
    /// 
    /// # Return
    #[cfg(all(feature = "configUSE_MUTEXES", feature = "INCLUDE_xSemaphoreGetMutexHolder"))]
    fn get_mutex_holder(&mut self) -> Option<TaskHandle> {
        let mut mutex_holder: Option<TaskHandle>;
        taskENTER_CRITICAL!();
        {
            mutex_holder  = self.0.peek(0).unwrap(); //tickstowait's value remain discussed
        }
        taskEXIT_CRITICAL!();
        mutex_holder
    }

    fn get_mutex(&mut self, Item:TaskHandle) -> (Result<(), QueueError>) {
        //FIXME:make sure the mutex is not occupied!
        match self.0.peek(xTicksToWait: TickType) {
            Some() => expr,
            None() => Err(expr),
        }
    }

    fn release_mutex(&mut self) -> (Result<(), QueueError>) {
        //insure the one release the mutex equal to the mutex holder
        let mut mutex_holder : TaskHandle = self.get_mutex_holder().unwrap();
        //FIXME:==
        if mutex_holder == get_current_task_handle!() {
            self.0.overwrite(None)    
        }
        else {
            unimplemented!();
        }
    }
}