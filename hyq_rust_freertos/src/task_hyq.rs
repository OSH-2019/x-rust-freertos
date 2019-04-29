//* task API
use crate::port::*;
use crate::list::*;

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

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum task_state {
    running   = 0,
    ready     = 1,
    blocked   = 2,
    suspended = 3,
    deleted   = 4
}

pub struct task_control_block{
    is_none        : bool,
	state_list_item: ListItem,
	evnet_list_item: ListItem,
	task_priority  : UBaseType,
	task_stacksize : UBaseType,
	task_name      : String,
	stack_pos      : *mut StackType,

    #[cfg(portStack_GROWTH > 0)]
    end_of_stack: *mut StackType,

    #[cfg(portCRITICAL_NESTING_IN_TCB == 1)]
    critical_nesting: UBaseType,

    #[cfg(configUSE_MUTEXES == 1)]
	base_priority  : UBaseType,
	#[cfg(configUSE_MUTEXES == 1)]
	mutexes_held   : UBaseType,

    #[cfg(configGENERATE_RUN_TIME_STATUS == 1)]
	runtime_counter: TickType,

    #[cfg(config_USE_TASK_NOTIFICATIONS == 1)]
	notified_value: u32,
	#[cfg(config_USE_TASK_NOTIFICATIONS == 1)]
	notify_state  : u8,
}

unsafe impl Send for task_result {}

pub fn initialize_task_list () {
	for priority in (0..configMAX_PRIORITIES-1)	{
		list_initialise ( ready_task_list [priority] );
	}

	list_initialise( delay_task_list1 );
	list_initialise( delay_task_list2 );
	list_initialise( pending_ready_list );

	if cfg!( INCLUDE_vTaskDelete == 1 )
	{
		list_initialise( task_watching_termination );
	}

	if cfg!( INCLUDE_vTaskSuspend == 1 )
	{
		list_initialise( suspend_task_list );
	}

	/* Start with pxDelayedTaskList using list1 and the pxOverflowDelayedTaskList
	using list2. */
	delay_task_list = &delay_task_list1;
	overflow_delay_task_list = &delay_task_list2;
}

pub add_new_task_to_ready_list (new_tcb: &task_control_block) {
    taskENTER_CRITCAL();
    {
        current_number_of_tasks += 1;
        if current_tcb.is_none {
            current_tcb = new_tcb;
            if current_number_of_tasks == 1 {
                initialize_task_list ();
            }
            else {
                COVERAGE_TEST_MARKER();
            }
        }
        else {
            if !scheduler_running {
                if current_tcb.priority <= new_tcb.priority {
                    current_tcb = new_tcb;
                }
                else {
                    COVERAGE_TEST_MARKER ();
                }
            }
        }
    }
    task_number += 1;
    if scheduler_running {
        if current_tcb.priority < new_tcb.priority{
            taskYIELD_IF_USING_PREEMPTION ();
        }
        else {
            COVERAGE_TEST_MARKER ();
        }
    }
    else {
        COVERAGE_TEST_MARKER ();
    }
}

//! FIXME  !!!NEED TO BE REDESIGN!!!
pub fn task_start_scheduler () {
    let mut xreturn: BaseType;
	/* Add the idle task at the lowest priority. */
	if cfg!( configSUPPORT_STATIC_ALLOCATION == 1 )
	{
		let mut idle_task_tcb_buffer : mut* StackType_t;
		let mut idle_task_stack_buffer : mut* StackType_t;
		let mut idle_task_stack_size : u32;

		/* The Idle task is created using user provided RAM - obtain the
		address of the RAM then create the idle task. */
		vApplicationGetIdleTaskMemory ( idle_task_tcb_buffer, idle_task_stack_buffer, idle_task_stack_size );

		xIdleTaskHandle = xTaskCreateStatic(	prvIdleTask,
												"IDLE",
												ulIdleTaskStackSize,
												( void * ) NULL,
												( tskIDLE_PRIORITY | portPRIVILEGE_BIT ),
												pxIdleTaskStackBuffer,
												pxIdleTaskTCBBuffer ); /*lint !e961 MISRA exception, justified as it is not a redundant explicit cast to all supported compilers. */

		if( xIdleTaskHandle != NULL )
		{
			xreturn = pdPASS;
		}
		else
		{
			xreturn = pdFAIL;
		}
	}
	#else
	{
		/* The Idle task is being created using dynamically allocated RAM. */
		xreturn = xTaskCreate(	prvIdleTask,
								"IDLE", configMINIMAL_STACK_SIZE,
								( void * ) NULL,
								( tskIDLE_PRIORITY | portPRIVILEGE_BIT ),
								&xIdleTaskHandle ); /*lint !e961 MISRA exception, justified as it is not a redundant explicit cast to all supported compilers. */
	}
	#endif /* configSUPPORT_STATIC_ALLOCATION */

	#if ( configUSE_TIMERS == 1 )
	{
		if( xreturn == pdPASS )
		{
			xreturn = xTimerCreateTimerTask();
		}
		else
		{
			mtCOVERAGE_TEST_MARKER();
		}
	}
	#endif /* configUSE_TIMERS */

	if( xReturn == pdPASS )
	{
		/* Interrupts are turned off here, to ensure a tick does not occur
		before or during the call to xPortStartScheduler().  The stacks of
		the created tasks contain a status word with interrupts switched on
		so interrupts will automatically get re-enabled when the first task
		starts to run. */
		portDISABLE_INTERRUPTS();

		#if ( configUSE_NEWLIB_REENTRANT == 1 )
		{
			/* Switch Newlib's _impure_ptr variable to point to the _reent
			structure specific to the task that will run first. */
			_impure_ptr = &( pxCurrentTCB->xNewLib_reent );
		}
		#endif /* configUSE_NEWLIB_REENTRANT */

		xNextTaskUnblockTime = portMAX_DELAY;
		xSchedulerRunning = pdTRUE;
		xTickCount = ( TickType_t ) 0U;

		/* If configGENERATE_RUN_TIME_STATS is defined then the following
		macro must be defined to configure the timer/counter used to generate
		the run time counter time base. */
		portCONFIGURE_TIMER_FOR_RUN_TIME_STATS();

		/* Setting up the timer tick is hardware specific and thus in the
		portable interface. */
		if( xPortStartScheduler() != pdFALSE )
		{
			/* Should not reach here as if the scheduler is running the
			function will not return. */
		}
		else
		{
			/* Should only reach here if a task calls xTaskEndScheduler(). */
		}
	}
	else
	{
		/* This line will only be reached if the kernel could not be started,
		because there was not enough FreeRTOS heap to create the idle task
		or the timer task. */
		configASSERT( xreturn != errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY );
	}

	/* Prevent compiler warnings if INCLUDE_xTaskGetIdleTaskHandle is set to 0,
	meaning xIdleTaskHandle is not used anywhere else. */
	( void ) xIdleTaskHandle;
}
/*-----------------------------------------------------------*/

pub fn task_end_scheduler () {
	/* Stop the scheduler interrupts and call the portable scheduler end
	routine so the original ISRs can be restored if necessary.  The port
	layer must ensure interrupts enable	bit is left in the correct state. */
	portDISABLE_INTERRUPTS();
	scheduler_running = false;
	vPortEndScheduler();
}
/*----------------------------------------------------------*/

pub fn task_suspend_all () {
	/* A critical section is not required as the variable is of type
	BaseType_t.  Please read Richard Barry's reply in the following link to a
	post in the FreeRTOS support forum before reporting this as a bug! -
	http://goo.gl/wu4acr */
	++scheduler_suspended;
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
        COVERAGE_TEST_MARKER ();

        self.is_none = false;
        self.task_name = pcname;

        if priority >= configMAX_PRIORITIES {
            priority = configMAX_PRIORITIES - 1;
        }else {
            COVERAGE_TEST_MARKER ();
        }

        self.task_priority = priority;

        if cfg!(configUSE_MUTEXES == 1) {
            self.mutexes_held = 0;
            self.base_priority = priority;
        }
        //FIXME list_initialise_item usage?
        self.state_list_item = list_initialise_item ();
        self.evnet_list_item = list_initialise_item ();

        if cfg!(portCRITICAL_NESTING_IN_TCB == 1) {
            self.critical_nesting = 0;
        }

        if cfg!(configGENERATE_RUN_TIME_STATUS == 1){
            self.runtime_counter = 0;
        }

        if cfg!(config_USE_TASK_NOTIFICATIONS == 1){
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
        px_newtcb.initialize_new_task (pccode,pcname,stack_depth,priority);
        add_new_task_to_ready_list (px_newtcb);
        return_status
    }
}

// * Chapter 8 : Basic task function
//TODO: Start a task
// * Input : void
// * Output : void
// * Info : page 108
fn vTaskStartScheuler() {
}

//TODO: Delete the Task
// * Input : - xTaskToEDelete : TaskHandle
// * Output : void
// * Info : page 125
fn vTaskDelete () {
    unimplemented!();
}

//TODO: Suspend the Task
// * Input : - xTaskToSuspend : TaskHandle
// * Output : void
// * Info : page 127
fn vTaskSuspend () {
    unimplemented!();
}

//TODO: Resume the Task
// * Input : - TaskToResume : TaskHandle
// * Output : void
// * Info : page 128
fn vTaskResume() {
    unimplemented!();
}

// * Chap 9 : Switch between tasks
//TODO: Attain next Task
// * Input : void
// * Output : void
// * Info : page 136
fn vTaskSwitchContext() {
    unimplemented!();
}

//TODO: Select the Highest Priority Task
// * MACRO
// * Info : page 136
fn taskSELECT_HIGHEST_PRIORITY_TASK() {
    unimplemented!();
}

//TODO: Judge whether to switch
// * Input : void
// * Output : BaseType
// * Info : page 139
fn xTaskIncrementTick() {
    unimplemented!();
}

// * Chap 10 : Control the Kernel
//TODO: Switch tasks
// * Input : void
// * Output : void
// * Info : page 146
fn taskYIELD() {
    unimplemented!();
}

//TODO: Go to CRITICAL
// * Input : void
// * Output : void
// * Info : page 146
fn taskENTER_CRITCAL() {
    unimplemented!();
}

//TODO: Exit CRITICAL
// * Input : void
// * Output : void
// * Info : page 146
fn taskEXIT_CRITICAL() {
    unimplemented!();
}

//TODO: Go to CRITICAL from ISR
// * Input : void
// * Output : void
// * Info : page 146
fn taskENTER_CRITCAL_FROM_ISR() {
    unimplemented!();
}

//TODO: Exit CRITICAL from ISR
// * Input : void
// * Output : void
// * Info : page 146
fn taskEXIT_CITICAL_FROM_ISR () {
    unimplemented!();
}

//TODO: Enable Interupt
// * Input : void
// * Output : void
// * Info : page 146
fn taskDISABLE_INTERUPTS ()  {
    unimplemented!();
}

//TODO: Disable Interput
// * Input : void
// * Output : void
// * Info : page 146
fn taskENABLE_INTERUPTS () {
    unimplemented!();
}

//TODO: Enable Scheduler
// * Input : void
// * Output : void
// * Info : page 146
fn vTaskStartScheduler() {
    unimplemented!();
}

//TODO: Disable Scheduler
// * Input : void
// * Output : void
// * Info : page 146
fn vTaskEndScheduler() {
    unimplemented!();
}

//TODO: Suspend all
// * Input : void
// * Output : void
// * Info : page 147
fn vTaskSuspendAll() {
    unimplemented!();
}

//TODO: Resume all
// * Input : void
// * Output : void
// * Info : page 147
fn xTaskResumeAll() {
    unimplemented!();
}

//TODO: Set step tick
// * Input : - xTicksToJump : TickType
// * Output : void
// * Info : page 148
fn vTaskStepTick (){
    unimplemented!();
}

// * Chap 11 : Task API
