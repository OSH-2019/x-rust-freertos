use crate::queue::*;

#[derive(Default)]
pub struct Mutex<T>(Queue<T>)

impl <T>Mutex<T> {
    /// # Description
    /// 
    /// # Argument
    /// 
    /// # Return
    #[cfg(feature = "configUSE_MUTEXES")]
    fn initialise_mutex(&self) {        
        self.pcHead = 0;
        self.pcTail = 0;
        self.ucQueueType = QueueType::Mutex;

        self.QueueUnion = 0;

        traceCREATE_MUTEX!(&self);

        unimplemented!();
    }

    /// # Description
    /// 
    /// # Argument
    /// 
    /// # Return
    #[cfg(all(feature = "configUSE_MUTEXES", feature = "configSUPPORT_DYNAMIC_ALLOCATION"))]
    fn queue_creat_mutex(arg: Type) -> RetType {
        unimplemented!();
    }

    s
}