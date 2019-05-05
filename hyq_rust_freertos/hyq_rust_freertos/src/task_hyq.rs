//* task API
use crate::port::*;
use crate::list::*;

//* Error type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

pub struct task_control_block{
    //* basic information
    is_none        : bool,
	state_list_item: ListItem,
	evnet_list_item: ListItem,
	task_priority  : UBaseType,
	task_stacksize : UBaseType,
	task_name      : String,
	stack_pos      : *mut StackType,

    //* end of stack
    #[cfg(portStack_GROWTH > 0)]
    end_of_stack: *mut StackType,

    //* nesting
    #[cfg(portCRITICAL_NESTING_IN_TCB == 1)]
    critical_nesting: UBaseType,

    //* reverse priority
    #[cfg(configUSE_MUTEXES == 1)]
	base_priority  : UBaseType,
	#[cfg(configUSE_MUTEXES == 1)]
	mutexes_held   : UBaseType,

    #[cfg(configGENERATE_RUN_TIME_STATUS == 1)]
	runtime_counter: TickType,

    //* notify information
    #[cfg(config_USE_TASK_NOTIFICATIONS == 1)]
	notified_value: u32,
	#[cfg(config_USE_TASK_NOTIFICATIONS == 1)]
	notify_state  : u8,
}

unsafe impl Send for task_result {}

//* Initialize all the lists
//! FIXME   need to be fixed: define all the lists
//! ready_task_list[0...configMAX_PRIORITIES-1]
//! delay_task_list1
//! delay_task_list2
//! pending_ready_list
//! task_watching_termination
//! suspend_task_list
pub fn initialize_task_list () {
	for priority in (0..configMAX_PRIORITIES-1)	{
		list_initialise ( ready_task_list [priority] );
	}

	list_initialise( delay_task_list1 );
	list_initialise( delay_task_list2 );
	list_initialise( pending_ready_list );

	{
        #![cfg( INCLUDE_vTaskDelete == 1 )]
		list_initialise( task_watching_termination );
	}

	{
        #![cfg( INCLUDE_vTaskSuspend == 1 )]
		list_initialise( suspend_task_list );
	}

	/* Start with pxDelayedTaskList using list1 and the pxOverflowDelayedTaskList
	using list2. */
	delay_task_list = &delay_task_list1;
	overflow_delay_task_list = &delay_task_list2;
}

#[macro_export]
macro_rules! record_ready_priority {
    ($priority:expr) => ({
        if ($priority > top_ready_priority)
        {top_ready_priority = $priority;}
    })
}

pub fn add_task_to_ready_list (new_tcb: &task_control_block) {
    //* move_task_to_ready_state (new_tcb);
    record_ready_priority! (new_tcb.task_priority);
    list_insert_end! (ready_task_list[new_tcb.task_priority],new_tcb.state_list_item);
    //* post for trace
}

pub fn add_new_task_to_ready_list (new_tcb: &task_control_block) {
    taskENTER_CRITCAL();
    {
        set_current_number_of_tasks!(get_current_number_of_tasks!() + 1);
        if current_tcb.is_none {
            current_tcb = new_tcb;
            if get_current_number_of_tasks!() == 1 {
                initialize_task_list ();
            }
            else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
        else {
            if !scheduler_running {
                if current_tcb.priority <= new_tcb.priority {
                    current_tcb = new_tcb;
                }
                else {
                    mtCOVERAGE_TEST_MARKER! ();
                }
            }
        }
        task_number += 1;
        add_task_to_ready_list(&new_tcb);
    }
    taskEXIT_CRITICAL();
    if scheduler_running {
        if current_tcb.priority < new_tcb.priority{
            taskYIELD_IF_USING_PREEMPTION ();
        }
        else {
            mtCOVERAGE_TEST_MARKER! ();
        }
    }
    else {
        mtCOVERAGE_TEST_MARKER! ();
    }
}

pub fn task_suspend_all () {
    set_scheduler_suspended!(get_scheduler_suspended!() + 1);
}

impl task_control_block {
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

        self.is_none = false;
        self.task_name = pcname;

        if priority >= configMAX_PRIORITIES {
            priority = configMAX_PRIORITIES - 1;
        }else {
            mtCOVERAGE_TEST_MARKER! ();
        }

        self.task_priority = priority;

        #cfg[(configUSE_MUTEXES == 1)] {
            self.mutexes_held = 0;
            self.base_priority = priority;
        }
        //FIXME list_initialise_item usage?
        self.state_list_item = list_initialise_item ();
        self.evnet_list_item = list_initialise_item ();

        #cfg[(portCRITICAL_NESTING_IN_TCB == 1)] {
            self.critical_nesting = 0;
        }

        #cfg[(configGENERATE_RUN_TIME_STATUS == 1)] {
            self.runtime_counter = 0;
        }
        #cfg[(config_USE_TASK_NOTIFICATIONS == 1)] {
            self.notify_state = taskNOT_WAITING_NOTIFICATION;
            self.notified_value = 0;
        }
    }

    pub fn create_task (pccode: fn(), pcname: String, stack_depth: u16, priority: UBaseType) -> BaseType{
        let mut return_status: BaseType;
        let mut px_stack: *mut StackType;
        //* Ignore the NULLs temporarily
        px_stack = port::port_malloc(stack_depth * mem::size_of<StackType>());
        let mut px_newtcb: task_control_block;
        px_newtcb.modify_stackpos(px_stack);
        //FIXME modifying return_status if malloc failed
        //ready to insert into a list
        px_newtcb.initialize_new_task (pccode, pcname, stack_depth, priority);
        add_new_task_to_ready_list (px_newtcb);
        return_status
    }

    pub fn task_delete (task_to_delete: *mut task_control_block){
        taskENTER_CRITCAL! (){
            let tcb = get_tcb_from_handle (task_to_delete);
            if list_remove ()
        }
    }
}

pub fn suspend_task (task_to_suspend: &tcb_position) {
    taskENTER_CRITCAL!(){
    }
}