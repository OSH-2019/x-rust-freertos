use std::collections::VecDeque;
use crate::list::*;
use crate::port::*;
//use crate::trace::*;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use crate::*;
use crate::queue_h::*;
//use volatile::Volatile;
//

pub const queueUNLOCKED:i8 = -1;
pub const queueLOCKED_UNMODIFIED:i8 = 0;
pub const queueSEMAPHORE_QUEUE_ITEM_LENGTH:UBaseType = 0;
pub const queueMUTEX_GIVE_BLOCK_TIME:TickType = 0;

pub enum QueueUnion {
    pcReadFrom(UBaseType),
    uxRecuriveCallCount(UBaseType),
}

pub struct QueueDefinition<T>{
    pcQueue: VecDeque<T>,
    
    pcHead: UBaseType,
    pcTail: UBaseType,
    pcWriteTo: UBaseType,

    u: QueueUnion,

    xTasksWaitingToSend:Vec<Rc<RefCell<ListItem>>>,
    xTasksWaitingToReceive:Vec<Rc<RefCell<ListItem>>> ,

    uxMessagesWaiting: UBaseType,
    uxLength: UBaseType,
    uxItemSize: UBaseType,  //这玩意还有必要吗
    
    cRxLock: i8,
    cTxLock: i8,
    
    #[cfg(all(configSUPPORT_STATIC_ALLOCATION, configSUPPORT_DYNAMIC_ALLOCATION))]
    ucStaticallyAllocated: u8,
    
    #[cfg(configUSE_QUEUE_SETS)]
    pxQueueSetContainer:Option<Box<QueueDefinition>>,
    
    #[cfg(configUSE_TRACE_FACILITY)] 
    uxQueueNumber: UBaseType,
    #[cfg(configUSE_TRACE_FACILITY)]
    ucQueueType: u8,

}

type xQueue<T> = QueueDefinition<T>;
type Queue<T> = QueueDefinition<T>;



impl <T>QueueDefinition<T>{
    
    /// # Description
    /// * 
    /// * Implemented by:Lei Siqi
    /// # Argument
    ///
    /// # Return
    ///
    #[cfg(configSUPPORT_DYNAMIC_ALLOCATION)]
    fn queue_generic_create<T>( uxQueueLength:UBaseType, uxItem:T, ucQueueType:u8) -> Queue<T> {
        let queue:Queue;

        queue.pcQueue =  VecDeque::with_capacity(uxQueueLength);

        #[cfg(configSUPPORT_STATIC_ALLOCATION)]
        queue.ucStaticallyAllocated = false;
        
        queue.initialise_new_queue(uxQueueLength,ucQueueType);
        queue
    }

    /// # Description
    /// *
    /// * Implemented by:Lei Siqi
    /// # Argument
    ///
    /// # Return
    ///
    fn initialise_new_queue(&self, uxQueueLength: UBaseType, ucQueueType: u8)  {
        self. pcHead = 0;
        self.uxLength = uxQueueLength;
        self.queue_generic_reset(true);
        
        {
        #![cfg(configUSE_TRACE_FACILITY)]
        self.ucQueueType = ucQueueType;
        }
        
        {
        #![cfg(configUSE_QUEUE_SETS)]
        self.pxQueueSetContainer  = None;
        }

        traceQUEUE_CREATE!(self);
    }

    /// # Description
    /// * reset the queue
    /// * Implemented by:Ning Yuting
    /// # Argument
    /// * `xNewQueue` - whether the queue is a new queue
    /// # Return
    /// * bool
    fn queue_generic_reset(&self, xNewQueue: bool) -> bool{
        //xNewQueue源码中为BaseType，改为bool
        //返回值原为BaseType，改为bool
        taskENTER_CRITICAL!();
        {
            //初始化队列相关成员变量
            self.pcTail = self.pcHead + self.uxLength;
            self.uxMessagesWaiting = 0 as UBaseType;
            self.pcWriteTo = self.pcHead;
            self.u = QueueUnion::pcReadFrom(self.pcHead + self.uxLength - (1 as UBaseType));
            self.cRxLock = queueUNLOCKED;
            self.cTxLock = queueUNLOCKED;
            self.pcQueue.clear();//初始化空队列
            if xNewQueue == false {
                if list_is_empty!(self.xTasksWaitingToSend) == false {
                    if xTaskRemoveFromEventList( &(self.xTasksWaitingToSend)) != false{
                        queueYIELD_IF_USING_PREEMPTION();
                    }
                    else{
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
                else{
                    mtCOVERAGE_TEST_MARKER!();
                }
            }
            else{
                list_initialise!(self.xTasksWaitingToSend);
                list_initialise!(self.xTasksWaitingToReceive);
            }
        }
        taskEXIT_CRITICAL!();
        true
    }

    /// # Description
    ///
    /// * Implemented by:Lei Siqi
    /// # Argument
    ///
    /// # Return
    ///
    fn queue_generic_send(&self, pvItemToQueue: T, xTicksToWait: TickType, xCopyPosition: BaseType) {
        let xEntryTimeSet: bool = false;
        let xYieldRequired: BaseType;
        let xTimeOut: TimeOut;
    }
    
    /// # Description
    /// 
    /// * Implemented by:Ning Yuting
    /// # Argument
    ///
    /// # Return
    /// * (BaseType,bool)
    fn queue_generic_send_from_isr(&self, pvItemToQueue: T, xCopyPosition: BaseType) ->(BaseType, bool){
        //原先参数const pxHigherPriorityTaskWoken: BaseType作为返回值的第二个元素，bool型
        //返回值改为struct

        let xReturn: BaseType;
        let pxHigherPriorityTaskWoken:bool = false;//默认为false,下面一些情况改为true

        portASSERT_IF_INTERRUPT_PRIORITY_INVALID!();
        let uxSavedInterruptStatus: UBaseType = portSET_INTERRUPT_MASK_FROM_ISR!();
        {
            if self.uxMessagesWaiting < self.uxLength || xCopyPosition == queueOVERWRITE {

                let cTxLock: i8 = self.cTxLock;
                traceQUEUE_SEND_FROM_ISR! (self);
                prvCopyDataToQueue(self, pvItemToQueue, xCopyPosition);

                if cTxLock == queueUNLOCKED{

                    #[cfg(configUSE_QUEUE_SETS)]
                    match self.pxQueueSetContainer{
                        Some =>{
                            if prvNotifyQueueSetContainer(self, xCopyPosition ) != false{
                                pxHigherPriorityTaskWoken = true
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        None => {
                            if list_is_empty!(self.xTasksWaitingToReceive ) == false{
                                if xTaskRemoveFromEventList( &self.xTasksWaitingToReceive ) != false{
                                    pxHigherPriorityTaskWoken = true;
                                }
                                else {
                                    mtCOVERAGE_TEST_MARKER!();
                                }
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                    }

                    if cfg!(not(configUSE_QUEUE_SETS)) {
                        if list_is_empty!(self.xTasksWaitingToReceive) == false{
                            if xTaskRemoveFromEventList( &self.xTasksWaitingToReceive) != false{
                                pxHigherPriorityTaskWoken = true;
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                }
                else {
                    self.cTxLock = (cTxLock + 1) as i8;
                }
                xReturn = pdPASS;
            }
            else {
                traceQUEUE_SEND_FROM_ISR_FAILED!(self);
                xReturn = errQUEUE_FULL;
            }
        }
        portCLEAR_INTERRUPT_MASK_FROM_ISR!( uxSavedInterruptStatus );
        (xReturn,pxHigherPriorityTaskWoken)
    }

    /// # Description
    /// * lock the queue
    /// * Implemented by:Ning Yuting
    /// # Argument
    /// * `&self` - queue
    /// # Return
    /// * Nothing
    fn lock_queue (&self){
        //源码中为宏，改为Queue的方法
        taskENTER_CRITICAL!();
        {
            if self.cRxLock == queueUNLOCKED{
                self.cRxLock = queueLOCKED_UNMODIFIED;
            }
            if self.cTxLock == queueUNLOCKED{
                self.cTxLock = queueLOCKED_UNMODIFIED;
            }
        }
        taskEXIT_CRITICAL!();
        }
    
    /// # Description
    /// * unlock the queue
    /// * Implemented by:Ning Yuting
    /// # Argument
    /// * `&self` - queue
    /// # Return
    /// * Nothing
    fn unlock_queue (&self){
        
        taskENTER_CRITICAL!();
        {
            let cTxLock:i8 = self.cTxLock;
            while cTxLock > queueLOCKED_UNMODIFIED{

                #[cfg(configUSE_QUEUE_SETS)]
                match self.pxQueueSetContainer{
                    Some =>{
                        if prvNotifyQueueSetContainer(self, queueSEND_TO_BACK) != false{
                            vTaskMissedYield();
                        }
                        else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    None =>{
                        if list_is_empty!(self.xTasksWaitingToReceive) == false{
                            if xTaskRemoveFromEventList( &self.xTasksWaitingToReceive) != false{
                                vTaskMissedYield();
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        else {
                            break;
                        }
                    }
                }

                if cfg!(not(configUSE_QUEUE_SETS)) {
                    if list_is_empty!(self.xTasksWaitingToReceive) == false{
                        if xTaskRemoveFromEventList( &self.xTasksWaitingToReceive) != false{
                            vTaskMissedYield();
                        }
                        else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    else {
                        break;
                    }
                }

                --cTxLock;
            }
            self.cTxLock == queueUNLOCKED;
        }
        taskEXIT_CRITICAL!();

        taskENTER_CRITICAL!();
        {
            let cRxLock:i8 = self.cRxLock;
            while cRxLock > queueLOCKED_UNMODIFIED{
                if list_is_empty!(self.xTasksWaitingToReceive) == false{
                    if xTaskRemoveFromEventList(&self.xTasksWaitingToReceive) != false{
                        vTaskMissedYield();
                    }
                    else {
                        mtCOVERAGE_TEST_MARKER!();
                    }

                    --cRxLock;
                }
                else {
                    break;
                }
            }
            self.cRxLock = queueUNLOCKED;
        }
        taskEXIT_CRITICAL!();
    }

}

