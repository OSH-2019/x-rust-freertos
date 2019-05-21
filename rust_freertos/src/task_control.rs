use crate::port::*;
use crate::list::*;
use crate::kernel::*;
use crate::*;
use std::ffi::*;
use std::mem::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]

// * Enumerate for Errors
pub enum FreeRtosError {
    OutOfMemory,
    QueueSendTimeout,
    QueueReceiveTimeout,
    MutexTimeout,
    Timeout,
    QueueFull,
    StringConversionError,
    TaskNotFound,
    InvalidQueueSize,
    ProcessorHasShutDown
}

// * Task states
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum task_state {
    running   = 0,
    ready     = 1,
    blocked   = 2,
    suspended = 3,
    deleted   = 4
}

// * Top_priorities
pub enum updated_top_priority{
    Updated,
    Notupdated
}

// * TCB control blok stuct
pub struct task_control_block{
    //* basic information
	state_list_item: ListItem,
	evnet_list_item: ListItem,
	task_priority  : UBaseType,
	task_stacksize : UBaseType,
	task_name      : String,
	stack_pos      : *mut StackType,

    //* end of stack
    // #[cfg(portStack_GROWTH)]{}
    // end_of_stack: *mut StackType,

    //* nesting
    #[cfg(portCRITICAL_NESTING_IN_TCB)]
    critical_nesting: UBaseType,

    //* reverse priority
    #[cfg(configUSE_MUTEXES)]
	base_priority  : UBaseType,
	#[cfg(configUSE_MUTEXES)]
	mutexes_held   : UBaseType,

    #[cfg(configGENERATE_RUN_TIME_STATUS)]
	runtime_counter: TickType,

    //* notify information
    #[cfg(config_USE_TASK_NOTIFICATIONS)]
	notified_value: u32,
	#[cfg(config_USE_TASK_NOTIFICATIONS)]
	notify_state  : u8,
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

// * Initialize lists, for rebooting
// * Usage:
// * Input: Nonw
// * Output: None
pub fn initialize_task_list () {
	for priority in (0..configMAX_PRIORITIES!()-1){
		list_initialise! ( READY_TASK_LIST [priority] );
	}

	list_initialise!( DELAY_TASK_LIST1 );
	list_initialise!( DELAY_TASK_LIST2 );
	list_initialise!( PENDING_READY_LIST );

	{
        #![cfg( INCLUDE_vTaskDelete)]
		list_initialise!( TASK_WATCHING_TERMINATION );
	}

	{
        #![cfg( INCLUDE_vTaskSuspend)]
		list_initialise!( SUSPEND_TASK_LIST );
	}

	/* Start with pxDelayedTaskList using list1 and the pxOverflowDelayedTaskList
	using list2. */
	DELAY_TASK_LIST = &DELAY_TASK_LIST1;
	OVERFLOW_DELAY_TASK_LIST = &DELAY_TASK_LIST2;
}

// * Add a task to ready list, the task is already transfer into a Option
// * Usage:
// * Input: new_tcb (Option<tcb>)
// * Output: None
pub fn add_task_to_ready_list (new_tcb: Option<task_control_block>) {
    //* move_task_to_ready_state (new_tcb);
    record_ready_priority! (new_tcb.unwrap().task_priority);
    list_insert_end! (READY_TASK_LIST[new_tcb.unwrap().task_priority],new_tcb.state_list_item);
    //* post for trace
}

// * Add a new task to ready list, coping with new priority
// * Usage:
// * Input: new_tcb (Option<tcb>)
// * Output: None
pub fn add_new_task_to_ready_list (new_tcb: Option<task_control_block>) {
    taskENTER_CRITICAL!();
    {
        set_current_number_of_tasks!(get_current_number_of_tasks!() + 1);
        match CURRENT_TCB {
            None => {
                CURRENT_TCB = new_tcb;
                if get_current_number_of_tasks!() == 1 {
                    initialize_task_list ();
                }
                else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }
            Some (a) => {
            if !get_scheduler_running!() {
                if a.task_priority <= new_tcb.unwrap().task_priority {
                    CURRENT_TCB = new_tcb;
                }
                else {
                    mtCOVERAGE_TEST_MARKER! ();
                }
            }
        }
        }
        set_task_number!(get_task_number!() + 1);
        add_task_to_ready_list(new_tcb);
    }
    taskEXIT_CRITICAL!();
    if get_scheduler_running!() {
        if CURRENT_TCB.task_priority < new_tcb.unwrap().task_priority{
            taskYIELD_IF_USING_PREEMPTION! ();
        }
        else {
            mtCOVERAGE_TEST_MARKER! ();
        }
    }
    else {
        mtCOVERAGE_TEST_MARKER! ();
    }
}

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

impl task_control_block {
    // * Modify basic information
    // * Usage:
    // * Input: new_tcb (Option<tcb>)
    // * Output: None
    pub fn modify_name (&mut self, name:&str) -> &mut Self {
        self.task_name = name.to_owned().to_string();
        self
    }

    pub fn modify_stacksize (&mut self, stacksize: UBaseType) -> &mut Self {
        self.task_stacksize = stacksize;
        self
    }

    pub fn modify_priority (&mut self, priority: UBaseType) -> &mut Self {
        self.task_priority = priority;
        self
    }


    pub fn initialize_new_task (&mut self, pccode: fn(), pcname: String, stack_depth: u16, priority: UBaseType){
        let mut top_of_stack: *mut StackType;
        let mut x: UBaseType;
        top_of_stack = self.stack_pos + stack_depth - 1;
        top_of_stack = top_of_stack & portBYTE_ALIGNMENT_MASK;
        //FIXME fix it later: pcname string
        mtCOVERAGE_TEST_MARKER! ();

        self.task_name = pcname;

        if priority >= configMAX_PRIORITIES!() {
            priority = configMAX_PRIORITIES!() - 1;
        }else {
            mtCOVERAGE_TEST_MARKER! ();
        }

        self.task_priority = priority;

        #[cfg(configUSE_MUTEXES)]
        {
            self.mutexes_held = 0;
            self.base_priority = priority;
        }

        //FIXME list_initialise_item usage?
        list_initialise_item! (self.state_list_item);
        list_initialise_item! (self.evnet_list_item);

        #[cfg(portCRITICAL_NESTING_IN_TCB)]
        {
            self.critical_nesting = 0;
        }

        #[cfg(configGENERATE_RUN_TIME_STATUS)]
        {
            self.runtime_counter = 0;
        }

        #[cfg(config_USE_TASK_NOTIFICATIONS)]
        {
            self.notify_state = taskNOT_WAITING_NOTIFICATION;
            self.notified_value = 0;
        }
    }

    pub fn create_task (pccode: fn(), pcname: String, stack_depth: u16, priority: UBaseType) -> BaseType{
        let mut return_status: BaseType;
        let mut px_stack: *mut StackType;
        //* Ignore the NULLs temporarily
        px_stack = port::port_malloc(stack_depth * INITIAL_CAPACITY);
        let mut px_newtcb: task_control_block;
        px_newtcb.stack_pos = px_stack;
        //FIXME modifying return_status if malloc failed
        //ready to insert into a list
        px_newtcb.initialize_new_task (pccode, pcname, stack_depth, priority);
        let newtcb = Some(px_newtcb);
        add_new_task_to_ready_list (newtcb);
        return_status
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
    }
    else {
        mtCOVERAGE_TEST_MARKER! ();
    }
};
