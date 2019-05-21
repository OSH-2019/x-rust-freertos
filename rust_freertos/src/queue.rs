use std::collections::VecDeque;
use crate::list::*;
use crate::port::*;
//use crate::trace::*;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use crate::*;
use crate::queue_h::*;
use crate::projdefs::*;
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

#[derive(Default)]
pub struct QueueDefinition<T> 
    where T: Default {
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
    
    #[cfg(all(feature = "configSUPPORT_STATIC_ALLOCATION",feature = " configSUPPORT_DYNAMIC_ALLOCATION"))]
    ucStaticallyAllocated: u8,
    
    #[cfg(feature = "configUSE_QUEUE_SETS")]
    pxQueueSetContainer:Option<Box<QueueDefinition>>,
    
    #[cfg(feature = "configUSE_TRACE_FACILITY")] 
    uxQueueNumber: UBaseType,
    #[cfg(feature = "configUSE_TRACE_FACILITY")]
    ucQueueType: u8,

}

type xQueue<T> = QueueDefinition<T>;
type Queue<T> = QueueDefinition<T>;

impl Default for QueueUnion{
    fn default() -> Self {QueueUnion::pcReadFrom(0)}
}

impl <T>QueueDefinition<T>
    where T: Default{
    
    /// # Description
    /// * 
    /// * Implemented by:Lei Siqi
    /// # Argument
    ///
    /// # Return
    ///
    #[cfg(feature = "configSUPPORT_DYNAMIC_ALLOCATION")]
    fn queue_generic_create ( uxQueueLength:UBaseType, uxItem:T, ucQueueType:u8) -> Queue<T> {
        let queue:Queue<T>=Default::default();

        queue.pcQueue =  VecDeque::with_capacity(uxQueueLength as usize);

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
        self.pcHead=0;
        self.uxLength=uxQueueLength;
        self.queue_generic_reset(true);
        
        {
        #![cfg(feature = "configUSE_TRACE_FACILITY")]
        self.ucQueueType = ucQueueType;
        }
        
        {
        #![cfg(feature = "configUSE_QUEUE_SETS")]
        self.pxQueueSetContainer  = None;
        }

        traceQUEUE_CREATE!(self);
    }

    /// # Description
    /// * reset the queue
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.c 279-329
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
    /// * C implementation:queue.c 921-1069
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

                    #[cfg(feature = "configUSE_QUEUE_SETS")]
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
                    
                    {
                        #![cfg(not(feature = "configUSE_QUEUE_SETS"))]
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
    /// * C implementation:queue.c 264-276
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
    /// * C implementation:queue.c 1794-1911
    /// # Argument
    /// * `&self` - queue
    /// # Return
    /// * Nothing
    fn unlock_queue (&self){
        
        taskENTER_CRITICAL!();
        {
            let cTxLock:i8 = self.cTxLock;
            while cTxLock > queueLOCKED_UNMODIFIED{

                #[cfg(feature = "configUSE_QUEUE_SETS")]
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
                {
                    #![cfg(not(feature = "configUSE_QUEUE_SETS"))] 
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

    /// # Description
    /// * Post an item to the back of a queue.
    /// * Implemented by:Ning Yuting
    /// * C implementation: queue.h 355
    /// # Argument
    /// * `&self` - queue on which the item is to be posted.
    /// * `pvItemToQueue` - the item that is to be placed on the queue.
    /// * `xTicksToWait` - The maximum amount of time the task should block waiting for space to become available on the queue, should it already be full.
    /// # Return
    /// * true if the item was successfully posted, otherwise errQUEUE_FULL.
    fn queue_send_to_front(&self,pvItemToQueue:T,xTicksToWait:TickType){
        self.queue_generic_send(pvItemToQueue,xTicksToWait,queueSEND_TO_FRONT)
    }
    
    /// # Description
    /// * Post an item to the back of a queue.
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 437
    /// # Argument
    /// * same to queue_send_to_front
    /// # Return
    /// * same to queue_send_to_front
    fn queue_send_to_back(&self,pvItemToQueue:T,xTicksToWait:TickType){
        self.queue_generic_send(pvItemToQueue,xTicksToWait,queueSEND_TO_BACK)
    }
    
    /// # Description
    /// * equivalent to queue_send_to_back()
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 521
    /// # Argument
    /// * same to queue_send_to_back()
    /// # Return
    /// * same to queue_send_to_back()
    fn queue_send(&self,pvItemToQueue:T,xTicksToWait:TickType){
        self.queue_generic_send(pvItemToQueue,xTicksToWait,queueSEND_TO_BACK)
    }

    /// # Description
    /// * Only for use with queues that have a length of one - so the queue is either empty or full.
    /// * Post an item on a queue.  If the queue is already full then overwrite the value held in the queue. 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 604
    /// # Argument
    /// * `self` - queue
    /// * `pvItemToQueue` - the item that is to be place on the queue.
    /// # Return
     /// * pdPASS is the only value that can be returned because queue_overwrite will write to the
     /// queue even when the queue is already full.
    fn queue_overwrite(&self,pvItemToQueue:T){
        self.queue_generic_send(pvItemToQueue,0,queueOVERWRITE)
    }
    
    /// # Description
    /// * Post an item to the front ofa queue.  It is safe to use this macro from within an interrupt service routine.
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 1129
    /// # Argument
    /// * `self` - queue
    /// * `pvItemToQueue - the item taht is to be placed on the queue.
    /// # Return
    /// * `BaseType` -pdTRUE if the data was successfully sent to the queue, otherwise errQUEUE_FULL.
    /// * `bool` - pxHigherPriorityTaskWoken is changed to be a return value. it is true if sending to the
    /// queue caused a task to unblock,otherwise it is false.
    fn queue_send_to_front_from_isr(&self,pvItemToQueue:T)->(BaseType, bool){
        self.queue_generic_send_from_isr(pvItemToQueue,queueSEND_TO_FRONT)
    }

    /// # Description
    /// * Post an item to the back of a queue. Others is same to queue_send_to_front_from_isr
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 1200
    /// # Argument
    ///
    /// # Return
    ///
    fn queue_send_to_back_from_isr(&self,pvItemToQueue:T)->(BaseType, bool){
        self.queue_generic_send_from_isr(pvItemToQueue,queueSEND_TO_BACK)
    }

    /// # Description
    /// * A version of xQueueOverwrite() that can be used in an interrupt service routine (ISR).
    /// * Implemented by:Ning Yuting
    ///  * C implementation:queue.h 1287
    ///  # Argument
    ///
    ///  # Return
    ///
    fn queue_overwrite_from_isr(&self,pvItemToQueue:T)->(BaseType, bool){
        self.queue_generic_send_from_isr(pvItemToQueue,queueOVERWRITE)
    }
}
