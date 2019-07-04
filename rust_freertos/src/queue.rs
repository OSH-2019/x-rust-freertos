use std::collections::VecDeque;
//use crate::list::*;
use crate::port::*;
//use crate::trace::*;
//use std::rc::Rc;
//use std::cell::{RefCell, Ref, RefMut};
use crate::list::*;
use crate::queue_h::*;
use crate::*;
//use crate::projdefs::*;
use crate::task_queue::*;
//use volatile::Volatile;
//
pub const queueQUEUE_IS_MUTEX: UBaseType = 0;
pub const queueUNLOCKED: i8 = -1;
pub const queueLOCKED_UNMODIFIED: i8 = 0;
pub const queueSEMAPHORE_QUEUE_ITEM_LENGTH: UBaseType = 0;
pub const queueMUTEX_GIVE_BLOCK_TIME: TickType = 0;
/*
pub enum QueueUnion {
    pcReadFrom(UBaseType),
    uxRecuriveCallCount(UBaseType),
}
*/
#[derive(Default)]
pub struct QueueDefinition<T>
where
    T: Default + Clone,
{
    pcQueue: VecDeque<T>,

    pcHead: UBaseType,
    pcTail: UBaseType,
    pcWriteTo: UBaseType,

    /*pcReadFrom & uxRecuriveCallCount*/
    QueueUnion: UBaseType,

    xTasksWaitingToSend: ListLink,
    xTasksWaitingToReceive: ListLink,

    uxMessagesWaiting: UBaseType,
    uxLength: UBaseType,
    //uxItemSize: UBaseType,  //这玩意还有必要吗
    cRxLock: i8,
    cTxLock: i8,

    #[cfg(all(
        feature = "configSUPPORT_STATIC_ALLOCATION",
        feature = "configSUPPORT_DYNAMIC_ALLOCATION"
    ))]
    ucStaticallyAllocated: u8,

    #[cfg(feature = "configUSE_QUEUE_SETS")]
    pxQueueSetContainer: Option<Box<QueueDefinition>>,

    #[cfg(feature = "configUSE_TRACE_FACILITY")]
    uxQueueNumber: UBaseType,
    //#[cfg(feature = "configUSE_TRACE_FACILITY")]
    ucQueueType: QueueType,
}

//type xQueue<T> = QueueDefinition<T>;
//pub type Queue<T> = QueueDefinition<T>;
/*
impl Default for QueueUnion{
    fn default() -> Self {QueueUnion::pcReadFrom(0)}
}
*/
impl<T> QueueDefinition<T>
where
    T: Default + Clone,
{
    /// # Description
    /// *
    /// * Implemented by:Lei Siqi
    /// * * Modifiled by: Ning Yuting
    /// # Argument
    ///
    /// # Return
    ///
    #[cfg(feature = "configSUPPORT_DYNAMIC_ALLOCATION")]
    pub fn queue_generic_create(uxQueueLength: UBaseType, ucQueueType: QueueType) -> Self {
        let mut queue: QueueDefinition<T> = Default::default();
        queue.pcQueue = VecDeque::with_capacity(uxQueueLength as usize);
        queue.initialise_new_queue(uxQueueLength, ucQueueType);
        queue
    }

    /// # Description
    /// *
    /// * Implemented by:Lei Siqi
    /// # Argument
    ///
    /// # Return
    ///
    pub fn initialise_new_queue(&mut self, uxQueueLength: UBaseType, ucQueueType: QueueType) {
        self.pcHead = 0;
        self.uxLength = uxQueueLength;
        self.queue_generic_reset(true);

        //{
        // #![cfg(feature = "configUSE_TRACE_FACILITY")]
        self.ucQueueType = ucQueueType;
        //}

        {
            #![cfg(feature = "configUSE_QUEUE_SETS")]
            self.pxQueueSetContainer = None;
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
    pub fn queue_generic_reset(&mut self, xNewQueue: bool) -> Result<(), QueueError> {
        //xNewQueue源码中为BaseType，改为bool
        //返回值原为BaseType，改为result
        taskENTER_CRITICAL!();
        {
            //初始化队列相关成员变量
            self.pcTail = self.pcHead + self.uxLength;
            self.uxMessagesWaiting = 0 as UBaseType;
            self.pcWriteTo = self.pcHead;
            self.QueueUnion = self.pcHead + self.uxLength - (1 as UBaseType); //QueueUnion represents pcReadFrom
            self.cRxLock = queueUNLOCKED;
            self.cTxLock = queueUNLOCKED;
            self.pcQueue.clear(); //初始化空队列
            if xNewQueue == false {
                if list::list_is_empty(&self.xTasksWaitingToSend) == false {
                    if task_queue::task_remove_from_event_list(&self.xTasksWaitingToSend) != false {
                        queueYIELD_IF_USING_PREEMPTION!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                self.xTasksWaitingToSend = Default::default();
                self.xTasksWaitingToReceive = Default::default();
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
    pub fn queue_generic_send(
        &mut self,
        pvItemToQueue: T,
        xTicksToWait: TickType,
        xCopyPosition: BaseType,
    ) -> Result<(), QueueError> {
        let mut xEntryTimeSet: bool = false;
        //let mut xYieldRequired: bool = true;
        /*use default to solve the error:unitialized xTimeOut*/
        let mut xTimeOut: time_out = Default::default();
        let mut xTicksToWait = xTicksToWait;

        assert!(!((xCopyPosition == queueOVERWRITE) && self.uxLength == 1));

        #[cfg(all(feature = "xTaskGetSchedulerState", feature = "configUSE_TIMERS"))]
        assert!(
            !((kernel::task_get_scheduler_state() == SchedulerState::Suspended)
                && (xTicksToWait != 0))
        );
        trace!("Enter function queue_generic_send! TicksToWait: {}, uxMessageWaiting: {}, xCopyPosition: {}", xTicksToWait ,self.uxMessagesWaiting, xCopyPosition);
        /* This function relaxes the coding standard somewhat to allow return
        statements within the function itself.  This is done in the interest
        of execution time efficiency. */
        loop {
            taskENTER_CRITICAL!();
            {
                /* Is there room on the queue now?  The running task must be the
                highest priority task wanting to access the queue.  If the head item
                in the queue is to be overwritten then it does not matter if the
                queue is full. */
                if self.uxMessagesWaiting < self.uxLength || xCopyPosition == queueOVERWRITE {
                    traceQUEUE_SEND!(&self);
                    /*xYieldRequired = */
                    self.copy_data_to_queue(pvItemToQueue, xCopyPosition);
                    trace!("Queue can be sent");

                    /* The queue is a member of a queue set, and posting
                    to the queue set caused a higher priority task to
                    unblock. A context switch is required. */
                    #[cfg(feature = "configUSE_QUEUE_SETS")]
                    match self.pxQueueSetContainer {
                        Some => {
                            if notify_queue_set_container(&self, &xCopyPosition) != false {
                                queueYIELD_IF_USING_PREEMPTION!();
                            } else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        None => {
                            if list::list_is_empty(&self.xTasksWaitingToReceive) == false {
                                if task_queue::task_remove_from_event_list(
                                    &self.xTasksWaitingToReceive,
                                ) {
                                    queueYIELD_IF_USING_PREEMPTION!();
                                } else {
                                    mtCOVERAGE_TEST_MARKER!();
                                }
                            }
                        }
                    }

                    {
                        /* If there was a task waiting for data to arrive on the
                        queue then unblock it now. */
                        #![cfg(not(feature = "configUSE_QUEUE_SETS"))]
                        if !list::list_is_empty(&self.xTasksWaitingToReceive) {
                            if task_queue::task_remove_from_event_list(&self.xTasksWaitingToReceive)
                            {
                                /* The unblocked task has a priority higher than
                                our own so yield immediately.  Yes it is ok to do
                                this from within the critical section - the kernel
                                takes care of that. */
                                queueYIELD_IF_USING_PREEMPTION!();
                            } else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        /*when send = semaphore_take, it does not have to yield*/
                        /*else if xYieldRequired {
                            /* This path is a special case that will only get
                               executed if the task was holding multiple mutexes and
                               the mutexes were given back in an order that is
                               different to that in which they were taken. */
                            queueYIELD_IF_USING_PREEMPTION!();
                        }*/
                        else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    taskEXIT_CRITICAL!();
                    return Ok(()); //return pdPASS
                } else {
                    {
                        #![cfg(feature = "configUSE_MUTEXES")]
                        if self.ucQueueType == QueueType::Mutex || self.ucQueueType == QueueType::RecursiveMutex {
                            taskENTER_CRITICAL!();
                            {
                                let task_handle = self.transed_task_handle_for_mutex();
                                task_queue::task_priority_inherit(task_handle);
                            }
                            taskEXIT_CRITICAL!();
                        }
                        else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    if xTicksToWait == 0 as TickType {
                        /* The queue was full and no block time is specified (or
                        the block time has expired) so leave now. */
                        taskEXIT_CRITICAL!();
                        /* Return to the original privilege level before exiting
                        the function. */
                        traceQUEUE_SEND_FAILED!(&self);
                        trace!("Queue Send: QueueFull");
                        return Err(QueueError::QueueFull);
                    } else if !xEntryTimeSet {
                        /* The queue was full and a block time was specified so
                        configure the timeout structure. */
                        task_queue::task_set_time_out_state(&mut xTimeOut);
                        xEntryTimeSet = true;
                    } else {
                        /* Entry time was already set. */
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
            }
            taskEXIT_CRITICAL!();

            /* Interrupts and other tasks can send to and receive from the queue
            now the critical section has been exited. */
            kernel::task_suspend_all();
            self.lock_queue();

            /* Update the timeout state to see if it has expired yet. */
            if !task_queue::task_check_for_timeout(&mut xTimeOut, &mut xTicksToWait) {
                if self.is_queue_full() {
                    traceBLOCKING_ON_QUEUE_SEND!(&self);
                    trace!("queue_generic_send place on event list");
                    task_queue::task_place_on_event_list(&self.xTasksWaitingToSend, xTicksToWait);

                    /* Unlocking the queue means queue events can effect the
                    event list.  It is possible	that interrupts occurring now
                    remove this task from the event	list again - but as the
                    scheduler is suspended the task will go onto the pending
                    ready last instead of the actual ready list. */
                    self.unlock_queue();

                    /* Resuming the scheduler will move tasks from the pending
                    ready list into the ready list - so it is feasible that this
                    task is already in a ready list before it yields - in which
                    case the yield will not cause a context switch unless there
                    is also a higher priority task in the pending ready list. */
                    if !kernel::task_resume_all() {
                        portYIELD_WITHIN_API!();
                    }
                } else {
                    /* Try again. */
                    self.unlock_queue();
                    kernel::task_resume_all();
                }
            } else {
                /* The timeout has expired. */
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
    pub fn queue_generic_send_from_isr(
        &mut self,
        pvItemToQueue: T,
        xCopyPosition: BaseType,
    ) -> (Result<(), QueueError>, bool) {
        //原先参数const pxHigherPriorityTaskWoken: BaseType作为返回值的第二个元素，bool型
        //返回值改为struct

        let mut xReturn: Result<(), QueueError> = Ok(());
        let mut pxHigherPriorityTaskWoken: bool = false; //默认为false,下面一些情况改为true

        portASSERT_IF_INTERRUPT_PRIORITY_INVALID!();
        let uxSavedInterruptStatus: UBaseType = portSET_INTERRUPT_MASK_FROM_ISR!() as UBaseType;
        {
            if self.uxMessagesWaiting < self.uxLength || xCopyPosition == queueOVERWRITE {
                let cTxLock: i8 = self.cTxLock;
                traceQUEUE_SEND_FROM_ISR!(&self);
                self.copy_data_to_queue(pvItemToQueue, xCopyPosition);

                if cTxLock == queueUNLOCKED {
                    #[cfg(feature = "configUSE_QUEUE_SETS")]
                    match self.pxQueueSetContainer {
                        Some => {
                            if notify_queue_set_container(self, xCopyPosition) != false {
                                pxHigherPriorityTaskWoken = true
                            } else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        None => {
                            if list::list_is_empty(&self.xTasksWaitingToReceive) == false {
                                if task_queue::task_remove_from_event_list(
                                    &self.xTasksWaitingToReceive,
                                ) != false
                                {
                                    pxHigherPriorityTaskWoken = true;
                                } else {
                                    mtCOVERAGE_TEST_MARKER!();
                                }
                            } else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                    }

                    {
                        #![cfg(not(feature = "configUSE_QUEUE_SETS"))]
                        if list::list_is_empty(&self.xTasksWaitingToReceive) == false {
                            if task_queue::task_remove_from_event_list(&self.xTasksWaitingToReceive)
                                != false
                            {
                                pxHigherPriorityTaskWoken = true;
                            } else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        } else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                } else {
                    self.cTxLock = (cTxLock + 1) as i8;
                }
                xReturn = Ok(());
            } else {
                traceQUEUE_SEND_FROM_ISR_FAILED!(&self);
                xReturn = Err(QueueError::QueueFull);
            }
        }
        portCLEAR_INTERRUPT_MASK_FROM_ISR!(uxSavedInterruptStatus);
        (xReturn, pxHigherPriorityTaskWoken)
    }

    /// # Description
    /// * lock the queue
    /// * Implemented by:Ning Yuting
    /// * C implementation:queue.c 264-276
    /// # Argument
    /// * `&self` - queue
    /// # Return
    /// * Nothing
    pub fn lock_queue(&mut self) {
        //源码中为宏，改为Queue的方法
        taskENTER_CRITICAL!();
        {
            if self.cRxLock == queueUNLOCKED {
                self.cRxLock = queueLOCKED_UNMODIFIED;
            }
            if self.cTxLock == queueUNLOCKED {
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
    fn unlock_queue(&mut self) {
        taskENTER_CRITICAL!();
        {
            let mut cTxLock: i8 = self.cTxLock;
            while cTxLock > queueLOCKED_UNMODIFIED {
                #[cfg(feature = "configUSE_QUEUE_SETS")]
                match self.pxQueueSetContainer {
                    Some => {
                        if notify_queue_set_container(self, queueSEND_TO_BACK) != false {
                            task_queue::task_missed_yield();
                        } else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    None => {
                        if list::list_is_empty(&self.xTasksWaitingToReceive) == false {
                            if task_queue::task_remove_from_event_list(&self.xTasksWaitingToReceive)
                                != false
                            {
                                task_queue::task_missed_yield();
                            } else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        } else {
                            break;
                        }
                    }
                }
                {
                    #![cfg(not(feature = "configUSE_QUEUE_SETS"))]
                    if list::list_is_empty(&self.xTasksWaitingToReceive) == false {
                        if task_queue::task_remove_from_event_list(&self.xTasksWaitingToReceive)
                            != false
                        {
                            task_queue::task_missed_yield();
                        } else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    } else {
                        break;
                    }
                }

                cTxLock = cTxLock - 1;
            }
            self.cTxLock = queueUNLOCKED;
        }
        taskEXIT_CRITICAL!();

        taskENTER_CRITICAL!();
        {
            let mut cRxLock: i8 = self.cRxLock;
            while cRxLock > queueLOCKED_UNMODIFIED {
                if list::list_is_empty(&self.xTasksWaitingToReceive) == false {
                    if task_queue::task_remove_from_event_list(&self.xTasksWaitingToReceive)
                        != false
                    {
                        task_queue::task_missed_yield();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }

                    cRxLock = cRxLock - 1;
                } else {
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
    pub fn queue_generic_receive(
        &mut self,
        mut xTicksToWait: TickType,
        xJustPeeking: bool,
    ) -> Result<T, QueueError> {
        let mut xEntryTimeSet: bool = false;
        let mut xTimeOut: time_out = Default::default();
        /*when receive = give, it has to call the function task_priority_disinherit. It may require
         * yield.*/
        let mut xYieldRequired: bool = false;
        let mut buffer: Option<T>;
        #[cfg(all(feature = "xTaskGetSchedulerState", feature = "configUSE_TIMERS"))]
        assert!(
            !((kernel::task_get_scheduler_state() == SchedulerState::Suspended)
                && (xTicksToWait != 0))
        );
        loop {
            trace!(
                "Enter function queue_generic_receive, TicksToWait:{}, Peeking: {}!",
                xTicksToWait,
                xJustPeeking
            );
            taskENTER_CRITICAL!();
            {
                let uxMessagesWaiting: UBaseType = self.uxMessagesWaiting;
                trace!(
                    "queue_generic_receive: uxMessageWaiting: {}",
                    uxMessagesWaiting
                );
                /* Is there data in the queue now?  To be running the calling task
                must be the highest priority task wanting to access the queue. */
                if uxMessagesWaiting > 0 as UBaseType {
                    let pcOriginalReadPosition: UBaseType = self.QueueUnion; //QueueUnion represents pcReadFrom
                    buffer = self.copy_data_from_queue(); //
                    if xJustPeeking == false {
                        traceQUEUE_RECEIVE!(&self);
                        /* actually removing data, not just peeking. */
                        self.uxMessagesWaiting = uxMessagesWaiting - 1;

                        {
                            #![cfg(feature = "configUSE_MUTEXES")]
                            /*if uxQueueType == queueQUEUE_IS_MUTEX*/
                            if self.ucQueueType == QueueType::Mutex
                                || self.ucQueueType == QueueType::RecursiveMutex
                            {
                                /////
                                let task_handle = self.transed_task_handle_for_mutex();
                                xYieldRequired = task_queue::task_priority_disinherit(task_handle);
                                self.pcQueue.pop_front();
                            //let transed_none = transed_task_handle_to_T(None);
                            //self.pcQueue.insert(0,transed_none);
                            //let mutex_holder = transed_task_handle_to_T(task_increment_mutex_held_count());
                            //self.pcQueue.pop_front();
                            //self.pcQueue.insert(0,mutex_holder);
                            //self.pxMutexHolder = task_increment_mutex_held_count();
                            } else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        }
                        trace!("queue_generic_receive -- line 498");
                        if list::list_is_empty(&self.xTasksWaitingToSend) == false {
                            if task_queue::task_remove_from_event_list(&self.xTasksWaitingToSend)
                                != false
                            {
                                queueYIELD_IF_USING_PREEMPTION!();
                            } else {
                                trace!("queue_generic_receive -- line 504");
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        } else {
                            trace!("queue_generic_receive -- line 508");
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    } else {
                        traceQUEUE_PEEK!(&self);
                        /* The data is not being removed, so reset the read
                        pointer. */
                        self.QueueUnion = pcOriginalReadPosition; //QueueUnnion represents pcReadFrom
                        if list::list_is_empty(&self.xTasksWaitingToReceive) != false {
                            if task_queue::task_remove_from_event_list(&self.xTasksWaitingToReceive)
                                != false
                            {
                                queueYIELD_IF_USING_PREEMPTION!();
                            } else {
                                mtCOVERAGE_TEST_MARKER!();
                            }
                        } else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                    taskEXIT_CRITICAL!();
                    trace!("queue_generic_receive -- line 529");
                    return Ok(buffer.unwrap_or_else(|| panic!("buffer is empty!")));
                } else {
                    if xTicksToWait == 0 as TickType {
                        /* The queue was empty and no block time is specified (or
                        the block time has expired) so leave now. */
                        taskEXIT_CRITICAL!();
                        traceQUEUE_RECEIVE_FAILED!(&self);
                        return Err(QueueError::QueueEmpty);
                    } else if xEntryTimeSet == false {
                        /* The queue was empty and a block time was specified so
                        configure the timeout structure. */
                        task_queue::task_set_time_out_state(&mut xTimeOut);
                        xEntryTimeSet = true;
                    } else {
                        /* Entry time was already set. */
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
            }
            taskEXIT_CRITICAL!();
            trace!("queue_generic_receive -- line 553");
            kernel::task_suspend_all();
            self.lock_queue();
            trace!("queue_generic_receive -- line 556");
            /* Update the timeout state to see if it has expired yet. */
            if task_queue::task_check_for_timeout(&mut xTimeOut, &mut xTicksToWait) == false {
                if self.is_queue_empty() != false {
                    traceBLOCKING_ON_QUEUE_RECEIVE!(&self);
                    /*{
                        #![cfg(feature = "configUSE_MUTEXES")]
                        if self.ucQueueType == QueueType::Mutex
                            || self.ucQueueType == QueueType::RecursiveMutex
                        {
                            /* actually uxQueueType == pcHead */
                            taskENTER_CRITICAL!();
                            {
                                let task_handle = self.transed_task_handle_for_mutex();
                                task_queue::task_priority_inherit(task_handle);
                            }
                            taskEXIT_CRITICAL!();
                        } else {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }*/
                    task_queue::task_place_on_event_list(
                        &self.xTasksWaitingToReceive,
                        xTicksToWait,
                    );
                    self.unlock_queue();
                    if kernel::task_resume_all() == false {
                        portYIELD_WITHIN_API!();
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                } else {
                    self.unlock_queue();
                    kernel::task_resume_all();
                }
                trace!("queue_generic_receive -- line 589");
            } else {
                self.unlock_queue();
                kernel::task_resume_all();
                if self.is_queue_empty() != false {
                    traceQUEUE_RECEIVE_FAILED!(&self);
                    return Err(QueueError::QueueEmpty);
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }
        }
    }

    /// 原先是将队列中pcReadFrom处的内容拷贝到第二个参数pvBuffer中，现改为返回值
    pub fn copy_data_from_queue(&mut self) -> Option<T> {
        //        if self.ucQueueType == QueueType::Base || self.ucQueueType == QueueType::Set {
        self.QueueUnion += 1; //QueueUnion represents pcReadFrom in the original code
        if self.QueueUnion >= self.pcTail {
            self.QueueUnion = self.pcHead;
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
        let ret_val = self.pcQueue.get(self.QueueUnion as usize).cloned();
        Some(ret_val.unwrap())
        //        }
        //        else{
        //            None
        //        }
    }

    pub fn copy_data_to_queue(&mut self, pvItemToQueue: T, xPosition: BaseType) /*-> bool*/
    {
        /* This function is called from a critical section. */
        //let mut xReturn:bool = false;
        let mut uxMessagesWaiting: UBaseType = self.uxMessagesWaiting;

        {
            //TODO:understand the usage this part
            #![cfg(feature = "configUSE_MUTEXES")]
            if self.ucQueueType == QueueType::Mutex || self.ucQueueType == QueueType::RecursiveMutex
            {
                let mutex_holder = transed_task_handle_to_T(task_increment_mutex_held_count());
                //self.pcQueue.pop_front();
                self.pcQueue.insert(0, mutex_holder);
            //let task_handle = self.transed_task_handle_for_mutex();
            //xReturn = task_queue::task_priority_disinherit(task_handle);
            //self.pcQueue.pop_front();
            //self.pcQueue.insert(0,None);
            //self.pxMutexHolder = None;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }

        if xPosition == queueSEND_TO_BACK {
            self.pcQueue.insert(self.pcWriteTo as usize, pvItemToQueue);
            self.pcWriteTo = self.pcWriteTo + 1;

            if self.pcWriteTo >= self.pcTail {
                self.pcWriteTo = self.pcHead;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        } else {
            self.pcQueue.insert(self.QueueUnion as usize, pvItemToQueue); //QueueUnion represents pcReadFrom
            self.QueueUnion = self.QueueUnion - 1;
            if self.QueueUnion < self.pcHead {
                self.QueueUnion = self.pcTail - 1;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }

            if xPosition == queueOVERWRITE {
                if uxMessagesWaiting > 0 as UBaseType {
                    /* An item is not being added but overwritten, so subtract
                    one from the recorded number of items in the queue so when
                    one is added again below the number of recorded items remains
                    correct. */
                    uxMessagesWaiting = uxMessagesWaiting - 1;
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
        self.uxMessagesWaiting = uxMessagesWaiting + 1;
        //xReturn
    }

    /// # Description
    /// * Implemented by:Ning Yuting
    /// * C implementation: queue.c 1914
    pub fn is_queue_empty(&self) -> bool {
        let mut xReturn: bool = false;
        taskENTER_CRITICAL!();
        {
            if self.uxMessagesWaiting == 0 as UBaseType {
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
    pub fn is_queue_full(&self) -> bool {
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

    pub fn initialise_count(&mut self, initial_count: UBaseType) {
        self.uxMessagesWaiting = initial_count;
    }

    pub fn QueueUnion_decrease(&mut self) {
        self.QueueUnion = self.QueueUnion - 1;
    }

    pub fn QueueUnion_increase(&mut self) {
        self.QueueUnion = self.QueueUnion + 1;
    }

    pub fn is_QueueUnion_zero(&self) -> bool {
        if self.QueueUnion == 0 as UBaseType {
            return true;
        } else {
            return false;
        }
    }

    /* `new` has two arguments now:length, QueueType.
     * Remember to add QueueType when using it.
     */
    pub fn new(uxQueueLength: UBaseType, QueueType: QueueType) -> Self {
        QueueDefinition::queue_generic_create(uxQueueLength, QueueType)
    }

    #[cfg(feature = "configUSE_TRACE_FACILITY")]
    pub fn get_queue_number(&self) -> UBaseType {
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

    pub fn transed_task_handle_for_mutex(&self) -> Option<task_control::TaskHandle> {
        /* use unsafe to get transed_task_handle for mutex
         * inplemented by: Ning Yuting
         */
        let untransed_task_handle = self.pcQueue.get(0).cloned().unwrap();
        let untransed_task_handle = Box::new(untransed_task_handle);
        let mut task_handle: Option<task_control::TaskHandle>;
        unsafe {
            let transed_task_handle = std::mem::transmute::<
                Box<T>,
                Box<Option<task_control::TaskHandle>>,
            >(untransed_task_handle);
            task_handle = *transed_task_handle
        }
        task_handle
    }
}

fn transed_task_handle_to_T<T>(task_handle: Option<task_control::TaskHandle>) -> T {
    /* use unsafe to transmute Option<TaskHandle> to T type*/
    let mut T_type: T;
    let task_handle = Box::new(task_handle);
    unsafe {
        let transed_T =
            std::mem::transmute::<Box<Option<task_control::TaskHandle>>, Box<T>>(task_handle);
        T_type = *transed_T;
    }
    T_type
}

#[macro_export]
macro_rules! queueYIELD_IF_USING_PREEMPTION {
    () => {
        #[cfg(feature = "configUSE_PREEMPTION")]
        portYIELD_WITHIN_API!();
    };
}
