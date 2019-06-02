use crate::port::*;
use crate::list::*;
use crate::kernel::*;
use crate::task_global::*;
use crate::projdefs::FreeRtosError;
use crate::*;
use std::boxed::FnBox;
use std::sync::{Arc, RwLock};

//* task states
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum task_state {
    running   = 0,
    ready     = 1,
    blocked   = 2,
    suspended = 3,
    deleted   = 4
}

pub enum updated_top_priority{
    Updated,
    Notupdated
}

#[derive(Debug)]
pub struct task_control_block{
    //* basic information
    state_list_item: Arc<RwLock<ListItem>>,
    evnet_list_item: Arc<RwLock<ListItem>>,
    task_priority  : UBaseType,
    task_stacksize : UBaseType,
    task_name      : String,
    // `stack_pos` is StackType because raw pointer can't be sent between threads safely.
    stack_pos      : StackType,

    //* end of stack
    // #[cfg(portStack_GROWTH)]{}
    // end_of_stack: *mut StackType,

    //* nesting
    #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
    critical_nesting: UBaseType,

    //* reverse priority
    #[cfg(feature = "configUSE_MUTEXES")]
    base_priority  : UBaseType,
    #[cfg(feature = "configUSE_MUTEXES")]
    mutexes_held   : UBaseType,

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    runtime_counter: TickType,

    //* notify information
    #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
    notified_value: u32,
    #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
    notify_state  : u8,
    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    delay_aborted : u8,
}

pub type TCB = task_control_block;
pub type Task = task_control_block;
impl task_control_block {
    pub fn new() -> Self {
        task_control_block {
            state_list_item: ListItem::new(0),
            evnet_list_item: ListItem::new(0),
            task_priority  : 1,
            task_stacksize : configMINIMAL_STACK_SIZE!(),
            task_name      : String::from("Unnamed"),
            stack_pos      : 0,

            //* nesting
            #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
            critical_nesting: 0,

            //* reverse priority
            #[cfg(feature = "configUSE_MUTEXES")]
            base_priority  : 0,
            #[cfg(feature = "configUSE_MUTEXES")]
            mutexes_held   : 0,

            #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
            runtime_counter: 0,

            //* notify information
            #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
            notified_value: 0,
            #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
            notify_state  : 0,
            #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
            delay_aborted : 0,
        }
    }

    pub fn name (mut self, name:&str) -> Self {
        self.task_name = name.to_owned().to_string();
        self
    }

    pub fn stacksize (mut self, stacksize: UBaseType) -> Self {
        self.task_stacksize = stacksize;
        self
    }

    pub fn priority (mut self, priority: UBaseType) -> Self {
        if priority >= configMAX_PRIORITIES!() {
            warn!("Specified priority larger than system maximum priority, will be reduced.");
            info!("MAX_PRIORITY is {}, but got {}", configMAX_PRIORITIES!() - 1, priority);
            self.task_priority = configMAX_PRIORITIES!() - 1;
        } else {
            self.task_priority = priority;
        }
        self
    }

    pub fn initialise<F>(mut self, func: F) -> Result<TaskHandle, FreeRtosError>
        where F: FnOnce() -> ()
    {
        let size_of_stacktype = std::mem::size_of::<StackType>();
        let stacksize_as_bytes = size_of_stacktype * self.task_stacksize as usize;
        trace!("Initialising Task: {}, stack size: {} bytes", self.task_name, stacksize_as_bytes);

        // Return `Err` if malloc fails.
        let px_stack = port::port_malloc(stacksize_as_bytes)?;

        // A trick here. By changing raw pointer `px_stack` to StackType,
        // avoid using unsafe `*mut` as a struct field.
        // We don't lost any information here because raw pointers are actually addresses,
        // which can be stored as plain numbers.
        self.stack_pos = px_stack as StackType;
        trace!("stack_pos for task {} is {}", self.task_name, self.stack_pos);

        let mut top_of_stack = self.stack_pos + self.task_stacksize as StackType - 1;
        top_of_stack = top_of_stack & portBYTE_ALIGNMENT_MASK as StackType;

        let param_ptr = Box::new(Box::new(func) as Box<FnBox()>); // Pass task function as a parameter.
        let param_ptr = &*param_ptr as *const _ as *mut _; // Convert to raw pointer.

        /* We use a wrapper function to call the task closure,
         * this is how freertos.rs approaches this problem, and is explained here:
         * https://stackoverflow.com/questions/32270030/how-do-i-convert-a-rust-closure-to-a-c-style-callback
         */
        port::port_initialise_stack(top_of_stack as *mut _,
                                    Some(run_wrapper),
                                    param_ptr
                                    )?;

        /* Do a bunch of conditional initialisations. */
        #[cfg(feature = "configUSE_MUTEXES")]
        {
            self.mutexes_held = 0;
            self.base_priority = self.task_priority;
        }

        /* These list items were already initialised when `self` was created.
        list_initialise_item! (self.state_list_item);
        list_initialise_item! (self.evnet_list_item);
        */

        // Create task handle.
        let handle = TaskHandle(Arc::new(RwLock::new(self)));
        // TODO: Change type of list_items.
        let state_list_item = handle.get_state_list_item();
        let event_list_item = handle.get_event_list_item();
        set_list_item_owner!(state_list_item, &handle.0);
        set_list_item_owner!(event_list_item, &handle.0);
        let item_value = (configMAX_PRIORITIES!() - handle.get_priority()) as TickType;
        set_list_item_value!(state_list_item, item_value);

        #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
        {
            self.critical_nesting = 0;
        }

        #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
        {
            self.runtime_counter = 0;
        }

        #[cfg(feature = "config_USE_TASK_NOTIFICATIONS")]
        {
            self.notify_state = taskNOT_WAITING_NOTIFICATION;
            self.notified_value = 0;
        }

        handle.add_new_task_to_ready_list()?;

        Ok(handle)
    }

    pub fn get_state_list_item(&self) -> Arc<RwLock<ListItem>> {
        Arc::clone(&self.state_list_item)
    }

    pub fn get_event_list_item(&self) -> Arc<RwLock<ListItem>> {
        Arc::clone(&self.evnet_list_item)
    }

    pub fn get_priority(&self) -> UBaseType {
        self.task_priority
    }

    pub fn get_name(&self) -> String {
        self.task_name.clone()
    }

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn get_run_time(&self) -> TickType {
        self.runtime_counter
    }

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn set_run_time(&mut self, next_val: TickType) -> TickType {
        let prev_val: u32 = self.runtime_counter;
        self.runtime_counter = next_val;
        prev_val
    }

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn get_delay_aborted (&self) -> u8 {self.delay_aborted}

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn set_delay_aborted (&self, next_val: u8) -> u8 {
        let prev_val: u8 = self.delay_aborted;
        self.delay_aborted = next_val;
        prev_val
    }
}

/* Task call wrapper function. */
extern "C" fn run_wrapper(func_to_run: CVoidPointer)
{
    info!("Run_wrapper: The function is at position: {:X}", func_to_run as u64);
    unsafe {
        let func_to_run = Box::from_raw(func_to_run as *mut Box<FnBox() + 'static>);
        func_to_run();
        // TODO: Delete this wrapper task.
    }
}

// * Record the Highest ready priority
// * Usage:
// * Input: num
// * Output: None
#[macro_export]
macro_rules! record_ready_priority {
    ($priority:expr) => ({
        if $priority > get_top_ready_priority!()
        {set_top_ready_priority!($priority);}
    })
}

/*
pub fn initialize_task_list () {
    for priority in (0..configMAX_PRIORITIES-1)	{
        list_initialise! ( READY_TASK_LIST [priority] );
    }

    list_initialise!( DELAY_TASK_LIST1 );
    list_initialise!( DELAY_TASK_LIST2 );
    list_initialise!( PENDING_READY_LIST );

    {
        #![cfg(INCLUDE_vTaskDelete)]
        list_initialise!( TASK_WATCHING_TERMINATION );
    }

    {
        #![cfg(INCLUDE_vTaskSuspend)]
        list_initialise!( SUSPEND_TASK_LIST );
    }

    /* Start with pxDelayedTaskList using list1 and the pxOverflowDelayedTaskList
       using list2. */
    DELAY_TASK_LIST = &DELAY_TASK_LIST1;
    OVERFLOW_DELAY_TASK_LIST = &DELAY_TASK_LIST2;
}
*/

/* Since multiple `TaskHandle`s may refer to and own a same TCB at a time,
 * we wrapped TCB within a `tuple struct` using `Arc<RwLock<_>>`
 */
#[derive(Clone)]
pub struct TaskHandle(Arc<RwLock<TCB>>);

impl TaskHandle {
    pub fn from_arc(arc: Arc<RwLock<TCB>>) -> Self {
        TaskHandle(arc)
    }

    pub fn from(tcb: TCB) -> Self {
        /* Construct a TaskHandle with a TCB. */
        TaskHandle(Arc::new(RwLock::new(tcb)))
    }

    /* This function is for use in FFI. */
    pub fn as_raw(self) -> ffi::xTaskHandle {
        Arc::into_raw(self.0) as *mut _
    }

    pub fn get_priority(&self) -> UBaseType{
        /* Get the priority of a task.
         * Since this method is so frequently used, I used a funtion to do it.
         */
        self.0.read().unwrap().get_priority()
    }

    pub fn add_task_to_ready_list (&self) -> Result<(), FreeRtosError>{
        let unwrapped_tcb = get_tcb_from_handle!(self);
        let priority = self.get_priority();

        traceMOVED_TASK_TO_READY_STATE!(&unwrapped_tcb);
        record_ready_priority! (priority);

        // let list_to_insert = (*READY_TASK_LISTS).write().unwrap();
        /* let list_to_insert = match list_to_insert {
            Ok(lists) => lists[unwrapped_tcb.task_priority as usize],
            Err(_) => {
                warn!("List was locked, read failed");
                return Err(FreeRtosError::DeadLocked);
            }
        };
        */
        list_insert_end! (nth_ready_list_mut!(priority), unwrapped_tcb.state_list_item);
        tracePOST_MOVED_TASK_TO_READY_STATE!(&unwrapped_tcb);
        Ok(())
    }

    fn add_new_task_to_ready_list (&self) -> Result<(), FreeRtosError>{
        let unwrapped_tcb = get_tcb_from_handle!(self);

        taskENTER_CRITICAL!();
        {
            // We don't need to initialise task lists any more.

            set_current_number_of_tasks!(get_current_number_of_tasks!() + 1);
            /* CURRENT_TCB won't be None. See task_global.rs. */
            let unwrapped_cur = get_current_task_handle!();
            if !get_scheduler_running!() {
                if unwrapped_cur.get_priority() <= unwrapped_tcb.task_priority {
                    /* If the scheduler is not already running, make this task the
                       current task if it is the highest priority task to be created
                       so far. */
                    set_current_task_handle!(self.clone());
                }
                else {
                    mtCOVERAGE_TEST_MARKER! ();
                }
            }
            set_task_number!(get_task_number!() + 1);
            traceTASK_CREATE!(self.clone());
            self.add_task_to_ready_list()?;
        }
        taskEXIT_CRITICAL!();
        if get_scheduler_running!() {
            let current_task_priority = get_current_task_handle!().get_priority();
            if current_task_priority < unwrapped_tcb.task_priority{
                taskYIELD_IF_USING_PREEMPTION!();
            }
            else {
                mtCOVERAGE_TEST_MARKER! ();
            }
        }
        else {
            mtCOVERAGE_TEST_MARKER! ();
        }

        Ok(())
    }

    pub fn get_event_list_item(&self) -> Arc<RwLock<ListItem>> {
        get_tcb_from_handle!(self).get_event_list_item()
    }

    pub fn get_state_list_item(&self) -> Arc<RwLock<ListItem>> {
        get_tcb_from_handle!(self).get_state_list_item()
    }

    pub fn get_name(&self) -> String {
        get_tcb_from_handle!(self).get_name()
    }

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn get_run_time(&self) -> TickType{
        get_tcb_from_handle!(self).get_run_time()
    }

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn set_run_time(&self, next_val: TickType) -> TickType{
        get_tcb_from_handle_mut!(self).set_run_time(next_val)
    }

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn get_delay_aborted (&self) -> u8 {
        get_tcb_from_handle!(self).get_delay_aborted()
    }

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn set_delay_aborted (&self, next_val: u8) -> u8 {
        get_tcb_from_handle!(self).set_delay_aborted(next_val)
    }
}

#[macro_export]
macro_rules! get_tcb_from_handle {
    ($handle: expr) => (
        match $handle.0.try_read() {
            Ok(a) => a,
            Err(_) => {
                warn!("TCB was locked, read failed");
                panic!("Task handle locked!");
            }
        }
    )
}

#[macro_export]
macro_rules! get_tcb_from_handle_mut {
    ($handle: expr) => (
        match $handle.0.try_write() {
            Ok(a) => a,
            Err(_) => {
                warn!("TCB was locked, write failed");
                panic!("Task handle locked!");
            }
        }
    )
}
/*

/*
   TODO : prvResetNextTaskUnblockTime list.c : 551
   TODO : prvDeleteTCB list.c : 480
*/
pub prv_reset_next_task_unblock_time () {
    if (list_is_empty!(pxDelayedTaskList))
    {
        xNextTaskUnblockTime = portMAX_DELAY;
    }
    else {
        ( pxTCB ) = ( TCB_t * ) listGET_OWNER_OF_HEAD_ENTRY( pxDelayedTaskList );
		xNextTaskUnblockTime = listGET_LIST_ITEM_VALUE( &( ( pxTCB )->xStateListItem ) );

    }
}

    pub fn delete_task (task_to_delete: task_handle){
        let mut px_tcb: *mut task_control_block;
        taskENTER_CRITICAL!(){
            px_tcb = get_tcb_from_handle (task_to_delete);
            if list_remove!(&px_tcb.state_list_item) == 0 {
                task_reset_ready_priority (&px_tcb.priority);
            }
            else {
                mtCOVERAGE_TEST_MARKER!();
            }

            if get_list_item_container(&px_tcb.evnet_list_item).is_some() {
                list_remove!(&px_tcb.state_list_item);
            }
            else {
                mtCOVERAGE_TEST_MARKER! ();
            }

            set_task_number!(get_task_number!()+1);
            if px_tcb == CURRENT_TCB {
                list_insert_end!(task_waiting_termination,px_tcb.state_list_item);
                deleted_tasks_waiting_clean_up += 1;
                //!FIXME YeildPending
                portPRE_TASK_DELETE_HOOK! (px_tcb,YeildPending);
            }
            else {
                set_task_number!(get_task_number!()-1);
                //!FIXME todo
                delete_tcb(pc_tcb);
                reset_next_task_unblock_time ();
            }
            //!FIXME todo
            trace_task_delete();
        }taskEXIT_CRITICAL!();

        if get_scheduler_running!(){
            config_assert (schedule_suspended == 0 ? 1 : 0);
            portYIELD_WITHIN_API! ();
        }
        else {
            mtCOVERAGE_TEST_MARKER! ();
        }
    }

    pub fn suspend_task (task_to_suspend: task_handle){
        let mut px_tcb: *mut task_control_block;
        taskENTER_CRITICAL!(){
            px_tcb = get_tcb_from_handle (task_to_suspend);
            traceTASK_SUSPEND(&px_tcb);
            if list_remove!(px_tcb.unwrap().state_list_item) == 0 {
                task_reset_ready_priority (&px_tcb.unwrap().priority);
            }
            else {
                mtCOVERAGE_TEST_MARKER! ();
            }

            if get_list_item_container!(px_tcb.unwrap().evnet_list_item).is_some() {
                list_remove!(px_tcb.unwrap().state_list_item);
            }
            else {
                mtCOVERAGE_TEST_MARKER! ();
            }
            list_insert_end!(task_waiting_termination,px_tcb.unwrap().state_list_item);
        }taskEXIT_CRITICAL!();

        if get_scheduler_running!(){
            taskENTER_CRITICAL!(){
                prv_reset_next_task_unblock_time();
            }taskEXIT_CRITICAL!();
        }
        else {
            mtCOVERAGE_TEST_MARKER! ();
        }

        if px_tcb == CURRENT_TCB {
            if get_scheduler_running!(){
                config_assert (schedule_suspended == 0 ? 1 : 0);
                portYIELD_WITHIN_API! ();
            }
            else {
                if current_list_length!(SUSPEND_TASK_LIST) == get_current_number_of_tasks!() {
                    px_tcb = None;
                }
                else {
                    task_switch_context();
                }
            }
        }
        else {
            mtCOVERAGE_TEST_MARKER!()
        }
    }

    pub fn resume_task (task_to_resume: task_handle){
        let mut px_tcb: *mut task_control_block;
        config_assert (task_to_resume);
        if px_tcb.is_some() && px_tcb!=CURRENT_TCB {
            taskENTER_CRITICAL!(){
                if get_task_is_tasksuspended(&px_tcb) {
                    teace_task_RESUME (&px_tcb);
                    list_remove! (px_tcb.unwrap().state_list_item);
                    add_task_to_ready_list(px_tcb);
                    if px_tcb.priority >= CURRENT_TCB.priority {
                        taskYIELD_IF_USING_PREEMPTION();
                    }else {
                        mtCOVERAGE_TEST_MARKER! ();
                    }
                }
                else {
                    mtCOVERAGE_TEST_MARKER! ();
                }
            }taskEXIT_CRITICAL!();
        }
        else {
            mtCOVERAGE_TEST_MARKER! ();
        }
    }
*/
