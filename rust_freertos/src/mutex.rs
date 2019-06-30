use crate::queue::*;
use crate::task_control::*;
use crate::queue_h::*;
use crate::*;
use crate::port::*;
//use crate::task_queue::*;

#[derive(Default)]
#[cfg(feature = "configUSE_MUTEXES")]
pub struct Semaphore(Queue<Option<TaskHandle>>);

pub type Mutex = Semaphore;
pub type BinarySemaphore = Semaphore;

#[cfg(feature = "configUSE_MUTEXES")]
impl Semaphore {
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

        self.0.queue_generic_send(None, 0 as TickType, queueSEND_TO_BACK);
    }

    /// # Description
    /// 
    /// # Argument
    /// 
    /// # Return
    #[cfg(all(feature = "configUSE_MUTEXES", feature = "configSUPPORT_DYNAMIC_ALLOCATION"))]
    fn mutex_create() -> Self {
        let mut mutex: Mutex = Semaphore(Queue::new_type(1, QueueType::Mutex));
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

    fn get_mutex(&mut self, xBlockTime: TickType) -> Result<Option<TaskHandle>, QueueError> {
        self.semaphore_take(xBlockTime)
    }

    fn semaphore_take(&mut self,xBlockTime: TickType) -> Result<Option<TaskHandle>, QueueError> {
        self.0.queue_generic_receive(xBlockTime, false)
    }

    fn release_mutex(&mut self) -> (Result<(), QueueError>) {
        //insure the one release the mutex equal to the mutex holder
        self.semaphore_give()
    }

    fn semaphore_give(&mut self) -> Result<(), QueueError> {
        self.0.queue_generic_send(None, semGIVE_BLOCK_TIME, queueSEND_TO_BACK)
    }

    /// # Description
    /// This type of semaphore can be used for pure synchronisation between tasks or
    /// between an interrupt and a task.  The semaphore need not be given back once
    /// obtained, so one task/interrupt can continuously 'give' the semaphore while
    /// another continuously 'takes' the semaphore.  For this reason this type of
    /// semaphore does not use a priority inheritance mechanism.  For an alternative
    /// that does use priority inheritance see xSemaphoreCreateMutex().
    /// old version
    /// fn semaphore_create_binary() -> Semaphore {
    ///    let mut binary_semaphore: BinarySemaphore = Semaphore(Queue::new_type(1, QueueType::BinarySemaphre));
    ///    binary_semaphore.semaphore_give();
    ///    binary_semaphore
    ///}
    /// new version
    fn semaphore_create_binary() -> Semaphore {
        Semaphore(Queue::new_type(1, QueueType::BinarySemaphre)
    }
    /*
    fn name(arg: Type) -> RetType {
        unimplemented!();
    }
    */
}