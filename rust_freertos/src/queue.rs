use std::collections::VecDeque;
use crate::list::*;
use crate::port::*;
//use crate::trace::*;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use crate::*;
use crate::queue_h::*;
use crate::projdefs::*;
use crate::task_queue::*;
use crate::task_global::*;
//use volatile::Volatile;
//
pub const queueQUEUE_IS_MUTEX:UBaseType = 0;
pub const queueUNLOCKED:i8 = -1;
pub const queueLOCKED_UNMODIFIED:i8 = 0;
pub const queueSEMAPHORE_QUEUE_ITEM_LENGTH:UBaseType = 0;
pub const queueMUTEX_GIVE_BLOCK_TIME:TickType = 0;
/*
pub enum QueueUnion {
    pcReadFrom(UBaseType),
    uxRecuriveCallCount(UBaseType),
}
*/
#[derive(Default)]
pub struct QueueDefinition<T> 
    where T: Default {
    pcQueue: VecDeque<T>,
    
    pcHead: UBaseType,
    pcTail: UBaseType,
    pcWriteTo: UBaseType,

    /*pcReadFrom & uxRecuriveCallCount*/
    QueueUnion:UBaseType,

    xTasksWaitingToSend:UBaseType,
    xTasksWaitingToReceive:UBaseType,

    uxMessagesWaiting: UBaseType,
    uxLength: UBaseType,
    //uxItemSize: UBaseType,  //这玩意还有必要吗
    
    cRxLock: i8,
    cTxLock: i8,
    
    #[cfg(all(feature = "configSUPPORT_STATIC_ALLOCATION",feature = "configSUPPORT_DYNAMIC_ALLOCATION"))]
    ucStaticallyAllocated: u8,
    
    #[cfg(feature = "configUSE_QUEUE_SETS")]
    pxQueueSetContainer:Option<Box<QueueDefinition>>,
    
    #[cfg(feature = "configUSE_TRACE_FACILITY")] 
    uxQueueNumber: UBaseType,
    //#[cfg(feature = "configUSE_TRACE_FACILITY")]
    ucQueueType: QueueType,

}

type xQueue<T> = QueueDefinition<T>;
type Queue<T> = QueueDefinition<T>;
/*
impl Default for QueueUnion{
    fn default() -> Self {QueueUnion::pcReadFrom(0)}
}
*/
impl <T>QueueDefinition<T>
    where T: Default{
    
    /// # Description
    /// * 
    /// * Implemented by:Lei Siqi
    /// * * Modifiled by: Ning Yuting
    /// # Argument
    ///
    /// # Return
    ///
    #[cfg(feature = "configSUPPORT_DYNAMIC_ALLOCATION")]
    fn queue_generic_create ( uxQueueLength:UBaseType, ucQueueType:QueueType) -> Self {
        let mut queue:Queue<T>=Default::default();
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
    fn initialise_new_queue(&mut self, uxQueueLength: UBaseType, ucQueueType: QueueType)  {
        self.pcHead=0;
        self.uxLength=uxQueueLength;
        self.queue_generic_reset(true);
        
        //{
        // #![cfg(feature = "configUSE_TRACE_FACILITY")]
        self.ucQueueType = ucQueueType;
        //}
        
        {
        #![cfg(feature = "configUSE_QUEUE_SETS")]
        self.pxQueueSetContainer  = None;
        }

        traceQUEUE_CREATE!(&self);
    }

    /// # Description
    /// * reset the queue
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.c 279-329
    /// # Argument
    /// * `xNewQueue` - whether the queue is a new queue
    /// # Return
    /// * bool
    fn queue_generic_reset(&mut self, xNewQueue: bool) -> Result<(), QueueError>{
        //xNewQueue源码中为BaseType，改为bool
        //返回值原为BaseType，改为result
        taskENTER_CRITICAL!();
        {
            //初始化队列相关成员变量
            self.pcTail = self.pcHead + self.uxLength;
            self.uxMessagesWaiting = 0 as UBaseType;
            self.pcWriteTo = self.pcHead;
            self.QueueUnion = self.pcHead + self.uxLength - (1 as UBaseType);
            self.cRxLock = queueUNLOCKED;
            self.cTxLock = queueUNLOCKED;
            self.pcQueue.clear();//初始化空队列
            if xNewQueue == false {
                if list_is_empty!(get_list!(self.xTasksWaitingToSend)) == false {
                    if task_remove_from_event_list(self.xTasksWaitingToSend) != false{
                        queueYIELD_IF_USING_PREEMPTION!();
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
                self.xTasksWaitingToSend = add_list();
                self.xTasksWaitingToReceive = add_list();
            }
        }
        taskEXIT_CRITICAL!();
        Ok(())
    }

    /// # Description
    ///
    /// * Implemented by:Lei Siqi
    /// * Modifiled by: Ning Yuting
    /// # Argument
    ///
    /// # Return
    ///
    fn queue_generic_send(&mut self, pvItemToQueue: T, xTicksToWait: TickType, xCopyPosition: BaseType) -> (Result<(), QueueError>){
        let xEntryTimeSet: bool = false;
        let xYieldRequired: bool = true;
        let xTimeOut: TimeOut;

        assert!(!((xCopyPosition==queueOVERWRITE)&&self.uxLength==1));

        #[cfg(all(feature = "xTaskGetSchedulerState", feature = "configUSE_TIMERS"))]
        assert!(!((kernel::task_get_scheduler_state() == SchedulerState::Suspended) && (xTicksToWait != 0)));

        loop {
            taskENTER_CRITICAL!();
            {
                if self.uxMessagesWaiting < self.uxLength || xCopyPosition == queueOVERWRITE {
                    traceQUEUE_SEND!(&self);
                    xYieldRequired = self.copy_data_to_queue(pvItemToQueue,xCopyPosition);

                    #[cfg(feature = "configUSE_QUEUE_SETS")]
                    match self.pxQueueSetContainer {
                        Some => {
                            if notify_queue_set_container(&self, &xCopyPosition) != false {
                                queueYIELD_IF_USING_PREEMPTION!();
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        },
                        None => {
                            if list_is_empty!(get_list!(self.xTasksWaitingToReceive)) == false {
                                if task_remove_from_event_list(&self.xTasksWaitingToReceive) {
                                    queueYIELD_IF_USING_PREEMPTION!();
                                }
                                else {
                                    mtCOVERAGE_TEST_MARKER!();
                                }
                            }
                        }
                    }

                    {
                        #![cfg(not(feature = "configUSE_QUEUE_SETS"))]
                        if list_is_empty!(get_list!(self.xTasksWaitingToReceive)) == false {
                            if task_remove_from_event_list(&self.xTasksWaitingToReceive) != false {
                                queueYIELD_IF_USING_PREEMPTION!();
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        else if xYieldRequired != true {
                            queueYIELD_IF_USING_PREEMPTION!();
                        }
                        else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    taskEXIT_CRITICAL!();
                    return Ok(()); //return pdPASS
                }
                else {
                    if xTicksToWait == 0 as TickType {
                        taskEXIT_CRITICAL!();
                        traceQUEUE_SEND_FAILED!(&self);
                        return Err(QueueError::QueueFull);
                    }
                    else if xEntryTimeSet == false {
                        task_set_time_out_state(xTimeOut);
                        xEntryTimeSet = true;
                    }
                    else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
            }
            taskEXIT_CRITICAL!();

            kernel::task_suspend_all();
            self.lock_queue();

            if task_check_for_time_out(xTimeOut, xTicksToWait) == false {
                if self.is_queue_full() != false {
                    traceBLOCKING_ON_QUEUE_SEND!(self);
                    task_place_on_event_list(&self.xTasksWaitingToSend, xTicksToWait);

                    self.unlock_queue();

                    if kernel::task_resume_all() == false {
                        portYIELD_WITHIN_API!();
                    }
                }
                else {
                    self.unlock_queue();
                    kernel::task_resume_all();
                }                
            }
            else {
                self.unlock_queue();
                kernel::task_resume_all();

                traceQUEUE_SEND_FAILED!(self);
                return Err(QueueError::QueueFull);
            }
        }
    }
    
    /// # Description
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.c 921-1069
    /// # Argument
    ///
    /// # Return
    /// * (BaseType,bool)
    fn queue_generic_send_from_isr(&mut self, pvItemToQueue: T, xCopyPosition: BaseType) ->(Result<(), QueueError>, bool){
        //原先参数const pxHigherPriorityTaskWoken: BaseType作为返回值的第二个元素，bool型
        //返回值改为struct

        let mut xReturn:Result<(), QueueError> = Ok(());
        let pxHigherPriorityTaskWoken:bool = false;//默认为false,下面一些情况改为true

        portASSERT_IF_INTERRUPT_PRIORITY_INVALID!();
        let uxSavedInterruptStatus: UBaseType = portSET_INTERRUPT_MASK_FROM_ISR!();
        {
            if self.uxMessagesWaiting < self.uxLength || xCopyPosition == queueOVERWRITE {

                let cTxLock: i8 = self.cTxLock;
                traceQUEUE_SEND_FROM_ISR!(&self);
                self.copy_data_to_queue(pvItemToQueue, xCopyPosition);

                if cTxLock == queueUNLOCKED{

                    #[cfg(feature = "configUSE_QUEUE_SETS")]
                    match self.pxQueueSetContainer{
                        Some =>{
                            if notify_queue_set_container(self, xCopyPosition ) != false{
                                pxHigherPriorityTaskWoken = true
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        None => {
                            if list_is_empty!(get_list!(self.xTasksWaitingToReceive)) == false{
                                if task_remove_from_event_list( &self.xTasksWaitingToReceive ) != false{
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
                        if list_is_empty!(get_list!(self.xTasksWaitingToReceive)) == false{
                            if task_remove_from_event_list( &self.xTasksWaitingToReceive) != false{
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
                xReturn = Ok(());
            }
            else {
                traceQUEUE_SEND_FROM_ISR_FAILED!(&self);
                xReturn = Err(QueueError::QueueFull);
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
    fn lock_queue (&mut self){
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
    fn unlock_queue (&mut self){
        
        taskENTER_CRITICAL!();
        {
            let cTxLock:i8 = self.cTxLock;
            while cTxLock > queueLOCKED_UNMODIFIED{

                #[cfg(feature = "configUSE_QUEUE_SETS")]
                match self.pxQueueSetContainer{
                    Some =>{
                        if notify_queue_set_container(self, queueSEND_TO_BACK) != false{
                            task_missed_yield();
                        }
                        else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    None =>{
                        if list_is_empty!(get_list!(self.xTasksWaitingToReceive)) == false{
                            if task_remove_from_event_list( &self.xTasksWaitingToReceive) != false{
                                task_missed_yield();
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
                    if list_is_empty!(get_list!(self.xTasksWaitingToReceive)) == false{
                        if task_remove_from_event_list( &self.xTasksWaitingToReceive) != false{
                            task_missed_yield();
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
                if list_is_empty!(get_list!(self.xTasksWaitingToReceive)) == false{
                    if task_remove_from_event_list(&self.xTasksWaitingToReceive) != false{
                        task_missed_yield();
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
    /// * 原第二个参数pvBuffer是读取到的数据，作为返回值的第二个. 
    /// * Implemented by:Ning Yuting
    /// * C implementation: queue.c 1237
    /// # Argument
    /// * 
    /// # Return
    /// * 
    fn queue_generic_receive(&mut self,xTicksToWait:TickType,xJustPeeking:bool)->Result<T, QueueError>{
        let mut xEntryTimeSet:BaseType = pdFALSE;
        let mut xTimeOut:TimeOut;
        let mut buffer:Option<T>;
        #[cfg(all(feature = "xTaskGetSchedulerState", feature = "configUSE_TIMERS"))]
        assert!(!((kernel::task_get_scheduler_state() == SchedulerState::Suspended) && (xTicksToWait != 0)));
        loop {
            taskENTER_CRITICAL!();
            {
                let uxMessagesWaiting:UBaseType = self.uxMessagesWaiting;
                
                /* Is there data in the queue now?  To be running the calling task
		    must be the highest priority task wanting to access the queue. */
                if uxMessagesWaiting > 0 as UBaseType{
                    let pcOriginalReadPosition:UBaseType = self.QueueUnion;
                    buffer = self.copy_data_from_queue();//
                    if xJustPeeking == false {
                        traceQUEUE_RECEIVE!(&self);    
                        /* actually removing data, not just peeking. */
                        self.uxMessagesWaiting = uxMessagesWaiting - 1;
                        {
                            #![cfg(feature = "configUSE_MUTEXES")]
                            if self.pcHead == queueQUEUE_IS_MUTEX{
                                /*actually uxQueueType == pcHead */
                                self.pcTail = pvTaskIncrementMutexHeldCount();
                                /*actually pxMutexHolder == pcTail*/
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }

                        if list_is_empty!(get_list!(self.xTasksWaitingToSend)) == false {
                            if task_remove_from_event_list(&self.xTasksWaitingToSend) != false {
                                queueYIELD_IF_USING_PREEMPTION!();
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    else {
                        traceQUEUE_PEEK!(&self);
                        /* The data is not being removed, so reset the read
			    pointer. */
                        self.QueueUnion = pcOriginalReadPosition;
                        if list_is_empty!(get_list!(self.xTasksWaitingToReceive)) != false {
                            if task_remove_from_event_list(&self.xTasksWaitingToReceive) != false{
                                queueYIELD_IF_USING_PREEMPTION!();
                            }
                            else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    taskEXIT_CRITICAL!();
                    return Ok(buffer.unwrap());
                }
                else {
                    if xTicksToWait == 0 as TickType {
                        /* The queue was empty and no block time is specified (or
			    the block time has expired) so leave now. */
                        taskEXIT_CRITICAL!();
                        traceQUEUE_RECEIVE_FAILED!(&self);
                        return Err(QueueError::QueueEmpty);
                    }
                    else if xEntryTimeSet == pdFALSE {
                        /* The queue was empty and a block time was specified so
			    configure the timeout structure. */
                        task_set_time_out_state(xTimeOut);
                    }
                    else {
                        /* Entry time was already set. */
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
            }
            taskEXIT_CRITICAL!();

            kernel::task_suspend_all();
            self.lock_queue();
            
            /* Update the timeout state to see if it has expired yet. */
            if task_check_for_time_out(xTimeOut,xTicksToWait) == false {
                if self.is_queue_empty() != false{
                    traceBLOCKING_ON_QUEUE_RECEIVE!(&self);
                    {
                        #![cfg(feature = "configUSE_MUTEXES")]
                        if self.pcHead == queueQUEUE_IS_MUTEX {
                            /* actually uxQueueType == pcHead */
                            taskENTER_CRITICAL!();
                            {
                                task_priority_inherit(&self.pxMutexHolder);
                            }
                            taskEXIT_CRITICAL!();
                        }
                        else{
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    task_place_on_event_list(&self.xTasksWaitingToReceive,xTicksToWait);
                    self.unlock_queue();
                    if kernel::task_resume_all() == false {
                        portYIELD_WITHIN_API!();
                    }
                    else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
                else {
                    self.unlock_queue();
                    kernel::task_resume_all();
                }
            }
            else {
                self.unlock_queue();
                kernel::task_resume_all();
                if self.is_queue_empty() != false{
                    traceQUEUE_RECEIVE_FAILED!(&self);
                    return Err(QueueError::QueueEmpty);
                }
                else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }           
        }
    }

    /// 原先是将队列中pcReadFrom处的内容拷贝到第二个参数pvBuffer中，现改为返回值
    fn copy_data_from_queue(&self) ->Option<T> {
        if self.ucQueueType == QueueType::Base || self.ucQueueType == QueueType::Set {
            self.QueueUnion += 1; //QueueUnion represents pcReadFrom in the original code
            if self.QueueUnion >= self.pcTail {
                self.QueueUnion = self.pcHead;
            }
            else {
                mtCOVERAGE_TEST_MARKER!();
            }
            Some(*(self.pcQueue.get(self.QueueUnion as usize)).unwrap())
        }
        else{
            None
        }
    }


    fn copy_data_to_queue(&mut self, pvItemToQueue:T,xPosition:BaseType) -> bool{
        /* This function is called from a critical section. */
        let mut xReturn:bool = false;
        let uxMessagesWaiting:UBaseType = self.uxMessagesWaiting;
        
        /* 未完成信号量部分处理*/
        /*
        {
            #![cfg(configUSE_MUTEXES)]
            if self.uxQueueType == queueQUEUE_IS_MUTEX {
                xReturn = task_priority_disinherit(self.pxMutexHolder);
                self.pxMutexHolder = None;
            }
            else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }*/

        if xPosition == queueSEND_TO_BACK{
            self.pcQueue.insert(self.pcWriteTo as usize,pvItemToQueue);
            self.pcWriteTo = self.pcWriteTo + 1;
            
            if self.pcWriteTo >=  self.pcTail {
                self.pcWriteTo = self.pcHead;
            }
            else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
        else {
            self.pcQueue.insert(self.QueueUnion as usize,pvItemToQueue);
            self.QueueUnion = self.QueueUnion - 1;
            if self.QueueUnion < self.pcHead {
                self.QueueUnion = self.pcTail - 1;
            }
            else {
                mtCOVERAGE_TEST_MARKER!();
            }

            if xPosition == queueOVERWRITE {
                if uxMessagesWaiting > 0 as UBaseType {
                    /* An item is not being added but overwritten, so subtract
                       one from the recorded number of items in the queue so when
                       one is added again below the number of recorded items remains
                       correct. */
                    uxMessagesWaiting = uxMessagesWaiting - 1;
                }
                else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }
            else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
        self.uxMessagesWaiting = uxMessagesWaiting + 1;
        xReturn
    }

    /// # Description
    /// * Implemented by:Ning Yuting
    /// * C implementation: queue.c 1914
    fn is_queue_empty(&self) -> bool{
        let mut xReturn:bool = false;
        taskENTER_CRITICAL!();
        {
            if self.uxMessagesWaiting == 0 as UBaseType{
                xReturn = true;
            }
        }
        taskEXIT_CRITICAL!();
        xReturn
    }

    /// # Description
    ///
    /// * Implemented by:Lei Siqi
    /// # Argument
    ///
    /// # Return
    ///
    fn is_queue_full(&self) -> bool {
        let mut xReturn: bool = false;
        taskENTER_CRITICAL!();
        {
            if self.uxMessagesWaiting == self.uxLength {
                xReturn = true;
            }
        }
        taskEXIT_CRITICAL!();
        xReturn
    }

    /*some api in queue.h*/

    /// # Description:
    /// * Creates a new queue instance, and returns a handle by which the new queue can be referenced.
    /// * Implemented by: Ning Yuting.
    /// * C implementation:queue.h 186
    ///
    /// # Arguments
    ///
    ///
    /// # Return
    ///
    pub fn new(uxQueueLength:UBaseType) -> Self {
        Queue::queue_generic_create(uxQueueLength,QueueType::Base)
    }

    /// # Description
    /// * Post an item to the back of a queue.
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation: queue.h 355
    /// 
    /// # Argument
    /// * `&self` - queue on which the item is to be posted.
    /// * `pvItemToQueue` - the item that is to be placed on the queue.
    /// * `xTicksToWait` - The maximum amount of time the task should block waiting for space to become available on the queue, should it already be full.
    /// 
    /// # Return
    /// * true if the item was successfully posted, otherwise errQUEUE_FULL.
    pub fn send_to_front(&mut self,pvItemToQueue:T,xTicksToWait:TickType)-> (Result<(), QueueError>){
        self.queue_generic_send(pvItemToQueue,xTicksToWait,queueSEND_TO_FRONT)
    }
    
    /// # Description
    /// * Post an item to the back of a queue.
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 437
    /// 
    /// # Argument
    /// * same to queue_send_to_front
    /// 
    /// # Return
    /// * same to queue_send_to_front
    pub fn send_to_back(&mut self,pvItemToQueue:T,xTicksToWait:TickType) -> (Result<(), QueueError>){
        self.queue_generic_send(pvItemToQueue,xTicksToWait,queueSEND_TO_BACK)
    }
    
    /// # Description
    /// * equivalent to queue_send_to_back()
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 521
    /// 
    /// # Argument
    /// * same to queue_send_to_back()
    /// 
    /// # Return
    /// * same to queue_send_to_back()
    pub fn send(&mut self,pvItemToQueue:T,xTicksToWait:TickType) -> (Result<(), QueueError>){
        self.queue_generic_send(pvItemToQueue,xTicksToWait,queueSEND_TO_BACK)
    }

    /// # Description
    /// * Only for use with queues that have a length of one - so the queue is either empty or full.
    /// * Post an item on a queue.  If the queue is already full then overwrite the value held in the queue. 
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 604
    /// 
    /// # Argument
    /// * `self` - queue
    /// * `pvItemToQueue` - the item that is to be place on the queue.
    /// 
    /// # Return
    /// * pdPASS is the only value that can be returned because queue_overwrite will write to the
    /// queue even when the queue is already full.
    pub fn overwrite(&mut self,pvItemToQueue:T) -> (Result<(), QueueError>){
        self.queue_generic_send(pvItemToQueue,0,queueOVERWRITE)
    }
    
    /// # Description
    /// * Post an item to the front ofa queue.  It is safe to use this macro from within an interrupt service routine.
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 1129
    /// 
    /// # Argument
    /// * `self` - queue
    /// * `pvItemToQueue - the item taht is to be placed on the queue.
    /// 
    /// # Return
    /// * `Result` -pdTRUE if the data was successfully sent to the queue, otherwise errQUEUE_FULL.
    /// * `bool` - pxHigherPriorityTaskWoken is changed to be a return value. it is true if sending to the
    /// queue caused a task to unblock,otherwise it is false.
    pub fn send_to_front_from_isr(&mut self,pvItemToQueue:T)->(Result<(), QueueError>, bool){
        self.queue_generic_send_from_isr(pvItemToQueue,queueSEND_TO_FRONT)
    }

    /// # Description
    /// * Post an item to the back of a queue. Others is same to queue_send_to_front_from_isr
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 1200
    /// 
    /// # Argument
    ///
    /// # Return
    ///
    pub fn send_to_back_from_isr(&mut self,pvItemToQueue:T) ->(Result<(), QueueError>, bool){
        self.queue_generic_send_from_isr(pvItemToQueue,queueSEND_TO_BACK)
    }

    /// # Description
    /// * A version of xQueueOverwrite() that can be used in an interrupt service routine (ISR).
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 1287
    /// 
    /// # Argument
    ///
    ///  # Return
    ///
    pub fn overwrite_from_isr(&mut self,pvItemToQueue:T)->(Result<(), QueueError>, bool){
        self.queue_generic_send_from_isr(pvItemToQueue,queueOVERWRITE)
    }
    
    /// # Description
    /// * This is a macro that calls the xQueueGenericReceive() function.
    /// * Receive an item from a queue.  The item is received by copy so a buffer of
    /// * adequate size must be provided.  The number of bytes copied into the buffer
    /// * was defined when the queue was created.
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 913
    /// 
    /// # Argument
    ///
    /// # Return
    ///
    pub fn receive(&mut self,xTicksToWait:TickType) -> Result<T, QueueError> {
        self.queue_generic_receive(xTicksToWait,false)
    }

    /// # Description
    /// * This is a macro that calls the xQueueGenericReceive() function.
    /// * Receive an item from a queue without removing the item from the queue.
    /// * The item is received by copy so a buffer of adequate size must be
    /// * provided.  The number of bytes copied into the buffer was defined when
    /// * the queue was created.
    /// 
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.h 787
    /// 
    /// # Argument
    ///
    /// # Return
    ///
    pub fn peek(&mut self,xTicksToWait:TickType) -> Result<T, QueueError>{
        self.queue_generic_receive(xTicksToWait,true)
    }

    #[cfg(feature = "configUSE_TRACE_FACILITY")]
    pub fn get_queue_number(&self){
        self.uxQueueNumber
    }


    /// # Description
    /// 
    /// # Argument
    /// 
    /// # Return
    #[cfg(feature = "configUSE_QUEUE_SETS")]
    fn notify_queue_set_container(&self, xCopyPosition: BaseType) {
        unimplemented!();
    }

}

#[macro_export]
macro_rules! queueYIELD_IF_USING_PREEMPTION{
    () => {
        #[cfg(feature = "configUSE_PREEMPTION")]
        portYIELD_WITHIN_API!();
    };
}
