use crate::port::*; 
use crate::projdefs::FreeRtosError;
use crate::task_global::*;
use crate::list;
use crate::list::{ItemLink};
use crate::*;
use std::boxed::FnBox;
use std::sync::{Weak, Arc, RwLock};
use std::mem;

//* task states
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum task_state {
    running = 0,
    ready = 1,
    blocked = 2,
    suspended = 3,
    deleted = 4,
}

pub enum updated_top_priority {
    Updated,
    Notupdated,
}

#[derive(Debug)]
pub struct task_control_block {
    //* basic information
    state_list_item: ItemLink,
    event_list_item: ItemLink,
    task_priority: UBaseType,
    task_stacksize: UBaseType,
    task_name: String,
    // `stack_pos` is StackType because raw pointer can't be sent between threads safely.
    stack_pos: StackType,

    //* end of stack
    // #[cfg(portStack_GROWTH)]{}
    // end_of_stack: *mut StackType,

    //* nesting
    #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
    critical_nesting: UBaseType,

    //* reverse priority
    #[cfg(feature = "configUSE_MUTEXES")]
    base_priority: UBaseType,
    #[cfg(feature = "configUSE_MUTEXES")]
    mutexes_held: UBaseType,

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    runtime_counter: TickType,

    //* notify information
    #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
    notified_value: u32,
    #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
    notify_state  : u8,
    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    delay_aborted : bool,
}

pub type TCB = task_control_block;
pub type Task = task_control_block;
impl task_control_block {
    pub fn new() -> Self {
        task_control_block {
            state_list_item: Default::default(),
            event_list_item: Default::default(),
            task_priority  : 1,
            task_stacksize : configMINIMAL_STACK_SIZE!(),
            task_name      : String::from("Unnamed"),
            stack_pos      : 0,

            //* nesting
            #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
            critical_nesting: 0,

            //* reverse priority
            #[cfg(feature = "configUSE_MUTEXES")]
            base_priority: 0,
            #[cfg(feature = "configUSE_MUTEXES")]
            mutexes_held: 0,

            #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
            runtime_counter: 0,

            //* notify information
            #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
            notified_value: 0,
            #[cfg(feature = "configUSE_TASK_NOTIFICATIONS")]
            notify_state  : 0,
            #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
            delay_aborted : false,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.task_name = name.to_owned().to_string();
        self
    }

    pub fn stacksize(mut self, stacksize: UBaseType) -> Self {
        self.task_stacksize = stacksize;
        self
    }

    pub fn priority(mut self, priority: UBaseType) -> Self {
        if priority >= configMAX_PRIORITIES!() {
            warn!("Specified priority larger than system maximum priority, will be reduced.");
            info!(
                "MAX_PRIORITY is {}, but got {}",
                configMAX_PRIORITIES!() - 1,
                priority
            );
            self.task_priority = configMAX_PRIORITIES!() - 1;
        } else {
            self.task_priority = priority;
        }
        self
    }

    pub fn initialise<F>(mut self, func: F) -> Result<TaskHandle, FreeRtosError>
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let size_of_stacktype = std::mem::size_of::<StackType>();
        let stacksize_as_bytes = size_of_stacktype * self.task_stacksize as usize;
        trace!(
            "Initialising Task: {}, stack size: {} bytes",
            self.task_name,
            stacksize_as_bytes
        );

        // Return `Err` if malloc fails.
        let px_stack = port::port_malloc(stacksize_as_bytes)?;

        // A trick here. By changing raw pointer `px_stack` to StackType,
        // avoid using unsafe `*mut` as a struct field.
        // We don't lost any information here because raw pointers are actually addresses,
        // which can be stored as plain numbers.
        self.stack_pos = px_stack as StackType;
        trace!(
            "stack_pos for task {} is {}",
            self.task_name,
            self.stack_pos
        );

        let mut top_of_stack = self.stack_pos + self.task_stacksize as StackType - 1;
        top_of_stack = top_of_stack & portBYTE_ALIGNMENT_MASK as StackType;

        let f = Box::new(Box::new(func) as Box<FnBox()>); // Pass task function as a parameter.
        let param_ptr = &*f as *const _ as *mut _; // Convert to raw pointer.
        trace!(
            "Function ptr of {} is at {:X}",
            self.get_name(),
            param_ptr as u64
        );

        /* We use a wrapper function to call the task closure,
         * this is how freertos.rs approaches this problem, and is explained here:
         * https://stackoverflow.com/questions/32270030/how-do-i-convert-a-rust-closure-to-a-c-style-callback
         */
        let result = port::port_initialise_stack(top_of_stack as *mut _,
                                                 Some(run_wrapper),
                                                 param_ptr
        );
        match result {
            Ok(_) => {
                trace!("Stack initialisation succeeded");
                /* We MUST forget `f`, otherwise it will be freed at the end of this function.
                 * But we need to call `f` later in `run_wrapper`, which will lead to
                 * some unexpected behavior.
                 */
                mem::forget(f);
            }
            Err(e) => return Err(e)
        }

        /* Do a bunch of conditional initialisations. */
        #[cfg(feature = "configUSE_MUTEXES")]
        {
            self.mutexes_held = 0;
            self.base_priority = self.task_priority;
        }

        /* These list items were already initialised when `self` was created.
        list_initialise_item! (self.state_list_item);
        list_initialise_item! (self.event_list_item);
        */

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

        // Create task handle.
        let sp = self.stack_pos;
        let handle = TaskHandle(Arc::new(RwLock::new(self)));
        // TODO: Change type of list_items.
        let state_list_item = handle.get_state_list_item();
        let event_list_item = handle.get_event_list_item();
        list::set_list_item_owner(&state_list_item, handle.clone());
        list::set_list_item_owner(&event_list_item, handle.clone());
        let item_value = (configMAX_PRIORITIES!() - handle.get_priority()) as TickType;
        list::set_list_item_value(&state_list_item, item_value);

        handle.add_new_task_to_ready_list()?;

        Ok(handle)
    }

    pub fn get_state_list_item(&self) -> ItemLink {
        Arc::clone(&self.state_list_item)
    }

    pub fn get_event_list_item(&self) -> ItemLink {
        Arc::clone(&self.event_list_item)
    }

    pub fn get_priority(&self) -> UBaseType {
        self.task_priority
    }

    pub fn set_priority(&mut self, new_priority: UBaseType) {
        self.task_priority = new_priority;
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
    pub fn get_delay_aborted (&self) -> bool {self.delay_aborted}

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn set_delay_aborted (&mut self, next_val: bool) -> bool {
        let prev_val: bool = self.delay_aborted;
        self.delay_aborted = next_val;
        prev_val
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn get_mutex_held_count(&self) -> UBaseType{
        self.mutexes_held
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn set_mutex_held_count(&mut self, new_count: UBaseType) {
        self.mutexes_held = new_count;
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn get_base_priority(&self) -> UBaseType{
        self.base_priority
    }
}

impl PartialEq for TCB {
    fn eq(&self, other: &Self) -> bool {
        self.stack_pos == other.stack_pos
    }
}

/* Task call wrapper function. */
extern "C" fn run_wrapper(func_to_run: CVoidPointer) {
    info!(
        "Run_wrapper: The function is at position: {:X}",
        func_to_run as u64
    );
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
    ($priority:expr) => {{
        if $priority > get_top_ready_priority!() {
            set_top_ready_priority!($priority);
        }
    }};
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
    OVERFLOW_DELAYED_TASK_LIST = &DELAY_TASK_LIST2;
}
*/

/* Since multiple `TaskHandle`s may refer to and own a same TCB at a time,
 * we wrapped TCB within a `tuple struct` using `Arc<RwLock<_>>`
 */
#[derive(Clone)]
pub struct TaskHandle(Arc<RwLock<TCB>>);

impl PartialEq for TaskHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.0.read().unwrap() == *other.0.read().unwrap()
    }
}

impl From<Weak<RwLock<TCB>>> for TaskHandle {
    fn from(weak_link: Weak<RwLock<TCB>>) -> Self {
        TaskHandle(weak_link
                   .upgrade()
                   .unwrap_or_else(|| panic!("Owner is not set"))
                   )
    }
}


impl From<TaskHandle> for Weak<RwLock<TCB>> {
    fn from(task: TaskHandle) -> Self {
        Arc::downgrade(&task.0)
    }
}

impl TaskHandle {
    pub fn from_arc(arc: Arc<RwLock<TCB>>) -> Self {
        TaskHandle(arc)
    }

    /// # Description:
    /// Construct a TaskHandle with a TCB. */
    /// * Implemented by: Fan Jinhao.
    /// * C implementation: 
    ///
    /// # Arguments 
    /// * `tcb`: The TCB that we want to get TaskHandle from.
    ///
    /// # Return
    /// 
    /// The created TaskHandle.
    pub fn from(tcb: TCB) -> Self {
        // TODO: Implement From.
        TaskHandle(Arc::new(RwLock::new(tcb)))
    }

    /* This function is for use in FFI. */
    pub fn as_raw(self) -> ffi::xTaskHandle {
        Arc::into_raw(self.0) as *mut _
    }

    pub fn get_priority(&self) -> UBaseType {
        /* Get the priority of a task.
         * Since this method is so frequently used, I used a funtion to do it.
         */
        self.0.read().unwrap().get_priority()
    }

    pub fn set_priority(&self, new_priority: UBaseType) {
        get_tcb_from_handle_mut!(self).set_priority(new_priority);
    }

    /// # Description:
    /// Place the task represented by pxTCB into the appropriate ready list for
    /// the task.  It is inserted at the end of the list.
    ///
    /// * Implemented by: Fan Jinhao.
    /// * C implementation: 
    ///
    /// # Arguments 
    /// 
    ///
    /// # Return
    /// 
    /// TODO
    pub fn add_task_to_ready_list(&self) -> Result<(), FreeRtosError> {
        let unwrapped_tcb = get_tcb_from_handle!(self);
        let priority = self.get_priority();

        traceMOVED_TASK_TO_READY_STATE!(&unwrapped_tcb);
        record_ready_priority!(priority);

        // let list_to_insert = (*READY_TASK_LISTS).write().unwrap();
        /* let list_to_insert = match list_to_insert {
            Ok(lists) => lists[unwrapped_tcb.task_priority as usize],
            Err(_) => {
                warn!("List was locked, read failed");
                return Err(FreeRtosError::DeadLocked);
            }
        };
        */
        // TODO: This line is WRONG! (just for test)
        // set_list_item_container!(unwrapped_tcb.state_list_item, list::ListName::READY_TASK_LISTS_1);
        list::list_insert_end(&READY_TASK_LISTS[priority as usize], 
                              Arc::clone(&unwrapped_tcb.state_list_item));
        tracePOST_MOVED_TASK_TO_READY_STATE!(&unwrapped_tcb);
        Ok(())
    }

    /// # Description:
    /// Called after a new task has been created and initialised to place the task
    /// under the control of the scheduler.
    /// 
    /// * Implemented by: Fan Jinhao.
    /// * C implementation: 
    ///
    /// # Arguments 
    /// 
    ///
    /// # Return
    /// 
    /// TODO
    fn add_new_task_to_ready_list(&self) -> Result<(), FreeRtosError> {
        let unwrapped_tcb = get_tcb_from_handle!(self);

        taskENTER_CRITICAL!();
        {
            // We don't need to initialise task lists any more.
            let n_o_t = get_current_number_of_tasks!() + 1;
            set_current_number_of_tasks!(n_o_t);
            /* CURRENT_TCB won't be None. See task_global.rs. */
            if task_global::CURRENT_TCB.read().unwrap().is_none() {
                set_current_task_handle!(self.clone());
                if get_current_number_of_tasks!() != 1 {
                    mtCOVERAGE_TEST_MARKER!(); // What happened?
                }
            } else {
                let unwrapped_cur = get_current_task_handle!();
                if !get_scheduler_running!() {
                    if unwrapped_cur.get_priority() <= unwrapped_tcb.task_priority {
                        /* If the scheduler is not already running, make this task the
                        current task if it is the highest priority task to be created
                        so far. */
                        set_current_task_handle!(self.clone());
                    } else {
                        mtCOVERAGE_TEST_MARKER!();
                    }
                }
            }
            set_task_number!(get_task_number!() + 1);
            traceTASK_CREATE!(self.clone());
            self.add_task_to_ready_list()?;
        }
        taskEXIT_CRITICAL!();
        if get_scheduler_running!() {
            let current_task_priority = get_current_task_handle!().get_priority();
            if current_task_priority < unwrapped_tcb.task_priority {
                taskYIELD_IF_USING_PREEMPTION!();
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }

        Ok(())
    }

    pub fn get_event_list_item(&self) -> ItemLink {
        get_tcb_from_handle!(self).get_event_list_item()
    }

    pub fn get_state_list_item(&self) -> ItemLink {
        get_tcb_from_handle!(self).get_state_list_item()
    }

    pub fn get_name(&self) -> String {
        get_tcb_from_handle!(self).get_name()
    }

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn get_run_time(&self) -> TickType {
        get_tcb_from_handle!(self).get_run_time()
    }

    #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
    pub fn set_run_time(&self, next_val: TickType) -> TickType {
        get_tcb_from_handle_mut!(self).set_run_time(next_val)
    }

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn get_delay_aborted (&self) -> bool {
        get_tcb_from_handle!(self).get_delay_aborted()
    }

    #[cfg(feature = "INCLUDE_xTaskAbortDelay")]
    pub fn set_delay_aborted (&self, next_val: bool) -> bool {
        get_tcb_from_handle_mut!(self).set_delay_aborted(next_val)
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn get_mutex_held_count(&self) -> UBaseType{
        get_tcb_from_handle!(self).get_mutex_held_count()
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn set_mutex_held_count(&self, new_count: UBaseType) {
        get_tcb_from_handle_mut!(self).set_mutex_held_count(new_count)
    }

    #[cfg(feature = "configUSE_MUTEXES")]
    pub fn get_base_priority(&self) -> UBaseType{
        get_tcb_from_handle!(self).get_base_priority()
    }
}

#[macro_export]
macro_rules! get_tcb_from_handle {
    ($handle: expr) => {
        match $handle.0.try_read() {
            Ok(a) => a,
            Err(_) => {
                warn!("TCB was locked, read failed");
                panic!("Task handle locked!");
            }
        }
    };
}

#[macro_export]
macro_rules! get_tcb_from_handle_mut {
    ($handle: expr) => {
        match $handle.0.try_write() {
            Ok(a) => a,
            Err(_) => {
                warn!("TCB was locked, write failed");
                panic!("Task handle locked!");
            }
        }
    };
}

pub fn delete_tcb (tcb_to_delete :std::sync::RwLockReadGuard<'_, task_control::task_control_block>)
{
    /* This call is required specifically for the TriCore port.  It must be
	above the vPortFree() calls.  The call is also used by ports/demos that
	want to allocate and clean RAM statically. */

    //port_free (*tcb_to_delete.stack_pos);
}

pub fn add_current_task_to_delayed_list (ticks_to_wait: TickType, can_block_indefinitely: bool) {
    /*
     * The currently executing task is entering the Blocked state.  Add the task to
     * either the current or the overflow delayed task list.
     */
    trace!("ADD");

    let unwrapped_cur = get_current_task_handle!();
    trace!("Remove succeeded");

    {
        #![cfg(feature = "INCLUDE_xTaskAbortDelay")]
        /* About to enter a delayed list, so ensure the ucDelayAborted flag is
           reset to pdFALSE so it can be detected as having been set to pdTRUE
           when the task leaves the Blocked state. */

        unwrapped_cur.set_delay_aborted(false);

        // NOTE by Fan Jinhao: Is this line necessary?
        // set_current_task_handle!(unwrapped_cur);
    }
    trace!("Abort succeeded");

    /* Remove the task from the ready list before adding it to the blocked list
       as the same list item is used for both lists. */
    if list::list_remove(unwrapped_cur.get_state_list_item()) == 0 {
        trace!("Returned 0");
        /* The current task must be in a ready list, so there is no need to
           check, and the port reset macro can be called directly. */
        portRESET_READY_PRIORITY! ( unwrapped_cur.get_priority () , get_top_ready_priority!() );
    } else {
        trace!("Returned not 0");
        mtCOVERAGE_TEST_MARKER!();
    }

    trace!("Remove succeeded");
    {
        #![cfg(feature = "INCLUDE_vTaskSuspend")]
        if ticks_to_wait == portMAX_DELAY && can_block_indefinitely {
            /* Add the task to the suspended task list instead of a delayed task
               list to ensure it is not woken by a timing event.  It will block
               indefinitely. */
            let cur_state_list_item = unwrapped_cur.get_state_list_item();
            list::list_insert_end(&SUSPEND_TASK_LIST, cur_state_list_item);
        } else {
            /* Calculate the time at which the task should be woken if the event
               does not occur.  This may overflow but this doesn't matter, the
               kernel will manage it correctly. */
            let time_to_wake = get_tick_count!() + ticks_to_wait;

            /* The list item will be inserted in wake time order. */
            let cur_state_list_item = unwrapped_cur.get_state_list_item();
            list::set_list_item_value(&cur_state_list_item, time_to_wake);

            if time_to_wake < get_tick_count!() {
                /* Wake time has overflowed.  Place this item in the overflow
                   list. */
                list::list_insert(&OVERFLOW_DELAYED_TASK_LIST, cur_state_list_item);
            } else {
                /* The wake time has not overflowed, so the current block list
                   is used. */
                set_list_item_container!(cur_state_list_item, DELAYED_TASK_LIST);
                list::list_insert(&DELAYED_TASK_LIST, unwrapped_cur.get_state_list_item());

                /* If the task entering the blocked state was placed at the
                   head of the list of blocked tasks then xNextTaskUnblockTime
                   needs to be updated too. */
                if time_to_wake < get_next_task_unblock_time!() {
                    set_next_task_unblock_time!( time_to_wake );
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }
        }
    }

    {
        #![cfg(not(feature = "INCLUDE_vTaskSuspend"))]
        /* Calculate the time at which the task should be woken if the event
           does not occur.  This may overflow but this doesn't matter, the kernel
           will manage it correctly. */
        let time_to_wake = get_tick_count!() + ticks_to_wait;

        let cur_state_list_item = unwrapped_cur.get_state_list_item();
        /* The list item will be inserted in wake time order. */
        list::set_list_item_value(&cur_state_list_item, time_to_wake);

        if time_to_wake < get_tick_count!()
        {
            /* Wake time has overflowed.  Place this item in the overflow list. */
            list::list_insert(&OVERFLOW_DELAYED_TASK_LIST, cur_state_list_item);
        }
        else
        {
            /* The wake time has not overflowed, so the current block list is used. */
            list::list_insert(&DELAYED_TASK_LIST, unwrapped_cur.get_state_list_item());

            /* If the task entering the blocked state was placed at the head of the
               list of blocked tasks then xNextTaskUnblockTime needs to be updated
               too. */
            if time_to_wake < get_next_task_unblock_time!()
            {
                set_next_task_unblock_time!( time_to_wake );
            }
            else
            {
                mtCOVERAGE_TEST_MARKER!();
            }
        }

        /* Avoid compiler warning when INCLUDE_vTaskSuspend is not 1. */
        // ( void ) xCanBlockIndefinitely;
    }

    trace!("Place succeeded");
}


/*
pub fn reset_next_task_unblock_time () {
    if list_is_empty! (get_list!(DELAYED_TASK_LIST)) {
		/* The new current delayed list is empty.  Set xNextTaskUnblockTime to
		the maximum possible value so it is	extremely unlikely that the
		if( xTickCount >= xNextTaskUnblockTime ) test will pass until
		there is an item in the delayed list. */
        set_next_task_unblock_time! (portMAX_DELAY);
    }
    else {
		/* The new current delayed list is not empty, get the value of
		the item at the head of the delayed list.  This is the time at
		which the task at the head of the delayed list should be removed
		from the Blocked state. */
        let mut temp = get_owner_of_head_entry! (get_list!(DELAYED_TASK_LIST));
        set_next_task_unblock_time! (get_list_item_value!(temp.clone().unwrap().read().unwrap().get_state_list_item()));
    }
}

pub fn task_delete (task_to_delete: TaskHandle)
{
    taskENTER_CRITICAL!();
    {
        /* If null is passed in here then it is the calling task that is
		being deleted. */
        let pxtcb = get_tcb_from_handle! (task_to_delete);

        /* Remove task from the ready list. */
        if list_remove! (pxtcb.get_state_list_item()) == 0 {
            taskRESET_READY_PRIORITY!(pxtcb.get_priority());
        }
        else {
            mtCOVERAGE_TEST_MARKER!();
        }

        /* Is the task waiting on an event also? */
		if get_list_item_container! (pxtcb.get_event_list_item ()).is_some() {
            list_remove! (pxtcb.get_event_list_item());
        }else {
            mtCOVERAGE_TEST_MARKER!();
        }


        /* Increment the uxTaskNumber also so kernel aware debuggers can
        detect that the task lists need re-generating.  This is done before
        portPRE_TASK_DELETE_HOOK() as in the Windows port that macro will
        not return. */

		set_task_number!(get_task_number!() + 1);

        if *pxtcb == *get_tcb_from_handle! (get_current_task_handle!())
		{
            /* A task is deleting itself.  This cannot complete within the
            task itself, as a context switch to another task is required.
            Place the task in the termination list.  The idle task will
            check the termination list and free up any memory allocated by
            the scheduler for the TCB and stack of the deleted task. */
            list_insert_end! ( get_list!(TASKS_WAITING_TERMINATION), pxtcb.get_state_list_item()  );

            /* Increment the ucTasksDeleted variable so the idle task knows
            there is a task that has been deleted and that it should therefore
            check the xTasksWaitingTermination list. */
            unsafe{
                DELETED_TASKS_WAITING_CLEAN_UP = DELETED_TASKS_WAITING_CLEAN_UP + 1;
            }
            /* The pre-delete hook is primarily for the Windows simulator,
            in which Windows specific clean up operations are performed,
            after which it is not possible to yield away from this task -
            hence xYieldPending is used to latch that a context switch is
            required. */
            portPRE_TASK_DELETE_HOOK!( pxtcb, get_yield_pending!() );
        }
        else{
                set_current_number_of_tasks! (get_current_number_of_tasks!() - 1);

				delete_tcb ( pxtcb );

				/* Reset the next expected unblock time in case it referred to
				the task that has just been deleted. */
				reset_next_task_unblock_time ();
		}
        // FIXME
		//traceTASK_DELETE!(task_to_delete);
    }
	taskEXIT_CRITICAL!();

    let mut pxtcb = get_tcb_from_handle! (task_to_delete);

		/* Force a reschedule if it is the currently running task that has just
		been deleted. */
		if get_scheduler_suspended!() > 0
		{
			if *pxtcb == *get_tcb_from_handle!( get_current_task_handle!())
			{
				assert!( get_scheduler_suspended!() == 0 );
				portYIELD_WITHIN_API! ();
			}
			else
			{
				mtCOVERAGE_TEST_MARKER! ();
			}
		}
}

pub fn suspend_task (task_to_suspend: TaskHandle){
    let mut px_tcb = get_tcb_from_handle! (task_to_suspend);
    taskENTER_CRITICAL!();
    {
        traceTASK_SUSPEND!(&px_tcb);
        if list_remove!(px_tcb.get_state_list_item()) == 0 {
            taskRESET_READY_PRIORITY! (px_tcb.get_priority());
        }
        else {
            mtCOVERAGE_TEST_MARKER! ();
        }

        if get_list_item_container!(px_tcb.get_event_list_item()).is_some() {
            list_remove!(px_tcb.get_state_list_item());
        }
        else {
            mtCOVERAGE_TEST_MARKER! ();
        }
        list_insert_end!(get_list!(TASKS_WAITING_TERMINATION),px_tcb.get_state_list_item());
    }taskEXIT_CRITICAL!();

    if get_scheduler_running!(){
        taskENTER_CRITICAL!();
        {
            reset_next_task_unblock_time();
        }
        taskEXIT_CRITICAL!();
    }
    else {
        mtCOVERAGE_TEST_MARKER! ();
    }

    if *px_tcb == *get_tcb_from_handle!( get_current_task_handle!()) {
        if get_scheduler_running!(){
            assert! (get_scheduler_suspended!() != 0);
            portYIELD_WITHIN_API! ();
        }
        else {
            if current_list_length!(get_list!(SUSPENDED_TASK_LIST)) != (get_current_number_of_tasks!()) as usize{
                task_switch_context();
            }
        }
    }
    else {
        mtCOVERAGE_TEST_MARKER!();
    }
}

pub fn task_is_tasksuspended (xtask: &TaskHandle) -> BaseType
{
	let mut xreturn:BaseType = 0;
	let tcb = get_tcb_from_handle! (xtask);
    /* Accesses xPendingReadyList so must be called from a critical
    section. */

    /* It does not make sense to check if the calling task is suspended. */
    //assert!( xtask );

    /* Is the task being resumed actually in the suspended list? */
    if is_contained_within! ( get_list!(SUSPENDED_TASK_LIST) , tcb.get_state_list_item() )
    {
        /* Has the task already been resumed from within an ISR? */
        if !is_contained_within! ( get_list!(PENDING_READY_LIST) , tcb.get_event_list_item() )
        {
            /* Is it in the suspended list because it is in the	Suspended
            state, or because is is blocked with no timeout? */
            if is_contained_within! ( get_list!( 0 ), tcb.get_event_list_item() )
            {
                xreturn = 1;
            }
            else
            {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
        else
        {
            mtCOVERAGE_TEST_MARKER!();
        }
    }
    else
    {
        mtCOVERAGE_TEST_MARKER!();
    }

    xreturn
}

pub fn resume_task (task_to_resume: TaskHandle){
    let mut px_tcb = get_tcb_from_handle! (task_to_resume);

    if /*NULL !*px_tcb && */ *px_tcb == *get_tcb_from_handle!( get_current_task_handle!()) {
        taskENTER_CRITICAL!();
        {
            if task_is_tasksuspended (&task_to_resume) == 1 {
                //trace_task_RESUME! (px_tcb);
                let current_task_priority = get_current_task_handle!().get_priority();
                list_remove! (px_tcb.get_state_list_item());
                task_to_resume.add_task_to_ready_list();
                if px_tcb.get_priority() >= current_task_priority {
                    taskYIELD_IF_USING_PREEMPTION!();
                }else {
                    mtCOVERAGE_TEST_MARKER! ();
                }
            }
            else {
                mtCOVERAGE_TEST_MARKER! ();
            }
        }
        taskEXIT_CRITICAL!();
    }
    else {
        mtCOVERAGE_TEST_MARKER!();
    }
}
*/
