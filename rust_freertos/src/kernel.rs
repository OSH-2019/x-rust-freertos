// kernel.rs, FreeRTOS scheduler control APIs.
// This file is created by Fan Jinhao.
// Functions defined in this file are explained in Chapter 9 and 10.

use crate::*; // TODO: Is this line necessary?
use crate::port::{TickType, UBaseType};
use crate::projdefs::pdFALSE;

/*
 * Originally from task. h
 *
 * Macro for forcing a context switch.
 *
 */
#[macro_export]
macro_rules! taskYIELD {
    () => (
        portYIELD!()
    )
}

#[macro_export]
macro_rules! taskYIELD_IF_USING_PREEMPTION {
    () => (
        #[cfg(configUSE_PREEMPTION)]
        portYIELD_WITHIN_API!()
    )
}
/*
 * Originally from task. h
 *
 * Macro to mark the start of a critical code region.  Preemptive context
 * switches cannot occur when in a critical region.
 *
 * NOTE: This may alter the stack (depending on the portable implementation)
 * so must be used with care!
 *
 */
#[macro_export]
macro_rules! taskENTER_CRITICAL {
    () => (
        portENTER_CRITICAL!()
    )
}

#[macro_export]
macro_rules! taskENTER_CRITICAL_FROM_ISR {
    () => (
        portSET_INTERRUPT_MASK_FROM_ISR!()
    )
}

/*
 * Originally from task. h
 *
 * Macro to mark the end of a critical code region.  Preemptive context
 * switches cannot occur when in a critical region.
 *
 * NOTE: This may alter the stack (depending on the portable implementation)
 * so must be used with care!
 *
 */
#[macro_export]
macro_rules! taskEXIT_CRITICAL {
    () => (
        portEXIT_CRITICAL!()
    )
}

#[macro_export]
macro_rules! taskEXIT_CRITICAL_FROM_ISR {
    ($x: expr) => (
        portCLEAR_INTERRUPT_MASK_FROM_ISR!($x)
    )
}

/// # Description:
/// Macro to disable all maskable interrupts.
/// * Implemented by: Fan Jinhao.
/// * C implementation: task.h
///
/// # Arguments 
///
/// # Return
/// 
/// Nothing

#[macro_export]
macro_rules! taskDISABLE_INTERRUPTS {
    () => (
        portDISABLE_INTERRUPTS!()
    )
}

/// # Description:
/// Macro to enable microcontroller interrupts.
/// 
/// * Implemented by: Fan Jinhao.
/// * C implementation: task.h
///
/// # Arguments 
///
/// # Return
/// 
/// Nothing

#[macro_export]
macro_rules! taskENABLE_INTERRUPTS {
    () => (
        portENABLE_INTERRUPTS!()
    )
}


/// # Description:
/// 
/// Starts the real time kernel tick processing.  After calling the kernel
/// has control over which tasks are executed and when.
/// 
/// See the demo application file main.c for an example of creating
/// tasks and starting the kernel.
/// 
/// * Implemented by: Fan Jinhao.
/// * C implementation: 
///
/// # Arguments 
/// 
///
/// # Return
/// 
/// Nothing
///
/// # Example
/// TODO: Finish the example.
/// ```
///  void vAFunction( void )
///  {
///	 // Create at least one task before starting the kernel.
///	 xTaskCreate( vTaskCode, "NAME", STACK_SIZE, NULL, tskIDLE_PRIORITY, NULL );
///
///	 // Start the real time kernel with preemption.
///	 vTaskStartScheduler ();
///
///	 // Will not get here unless a task calls vTaskEndScheduler ()
/// }
/// ```
pub fn task_start_scheduler() {
    /* Add the idle task at the lowest priority. */
    create_idle_task();

    #[cfg(configUSE_TIMERS)]
    create_timer_task();

    initialize_scheduler();
}

/// # Description:
/// The fist  part of task_start_scheduler(), creates the idle task. 
/// Will panic if task creation fails.
/// * Implemented by: Fan Jinhao.
/// * C implementation: tasks.c 1831-1866
///
/// # Arguments 
/// 
///
/// # Return
/// 
/// Nothing
fn create_idle_task() {
    // TODO: Wait for task_create.
    // On fail, panic!("Heap not enough to allocate idle task");
}

/// # Description:
/// The second (optional) part of task_start_scheduler(), 
/// creates the timer task. Will panic if task creation fails.
/// * Implemented by: Fan Jinhao.
/// * C implementation: tasks.c 1868-1879
///
/// # Arguments 
/// 
///
/// # Return
/// 
/// Nothing
fn create_timer_task() {
    // TODO: Wait for task_create.
    // timer::create_timer_task()
    // On fail, panic!("No enough heap space to allocate timer task.");
}

/// # Description:
/// The third part of task_step_scheduler, do some initialziation
/// and call port_start_scheduler() to set up the timer tick.
///
/// * Implemented by: Fan Jinhao.
/// * C implementation: tasks.c 1881-1918.
///
/// # Arguments 
/// 
///
/// # Return
/// 
/// Nothing
fn initialize_scheduler() {
    /* Interrupts are turned off here, to ensure a tick does not occur
       before or during the call to xPortStartScheduler().  The stacks of
       the created tasks contain a status word with interrupts switched on
       so interrupts will automatically get re-enabled when the first task
       starts to run. */
    portDISABLE_INTERRUPTS!();

    // TODO: NEWLIB

    set_next_task_unblock_time!(port::portMAX_DELAY);
    set_scheduler_running!(true);
    set_tick_count!(0);

    /* If configGENERATE_RUN_TIME_STATS is defined then the following
       macro must be defined to configure the timer/counter used to generate
       the run time counter time base. */
    portCONFIGURE_TIMER_FOR_RUN_TIME_STATS!();

    /* Setting up the timer tick is hardware specific and thus in the
       portable interface. */
    if port::port_start_scheduler() != pdFALSE
    {
        /* Should not reach here as if the scheduler is running the
           function will not return. */
    }
    else
    {
        // TODO: Maybe a trace here?
        /* Should only reach here if a task calls xTaskEndScheduler(). */
    }
}

/// # Description:
/// NOTE:  At the time of writing only the x86 real mode port, which runs on a PC
/// in place of DOS, implements this function.
/// 
/// Stops the real time kernel tick.  All created tasks will be automatically
/// deleted and multitasking (either preemptive or cooperative) will
/// stop.  Execution then resumes from the point where vTaskStartScheduler ()
/// was called, as if vTaskStartScheduler () had just returned.
/// 
/// See the demo application file main. c in the demo/PC directory for an
/// example that uses vTaskEndScheduler ().
/// 
/// vTaskEndScheduler () requires an exit function to be defined within the
/// portable layer (see vPortEndScheduler () in port. c for the PC port).  This
/// performs hardware specific operations such as stopping the kernel tick.
/// 
/// vTaskEndScheduler () will cause all of the resources allocated by the
/// kernel to be freed - but will not free resources allocated by application
/// tasks.
/// 
/// * Implemented by: Fan Jinhao.
/// * C implementation: 
///
/// # Arguments 
/// 
///
/// # Return
/// 
/// Nothing
///
/// # Example
/// TODO: Finish the doctest.
/// ```
///
/// void vTaskCode( void * pvParameters )
/// {
/// for( ;; )
/// {
/// // Task code goes here.
/// // At some point we want to end the real time kernel processing
/// // so call ...
/// println!("Task Code called successfully!");
/// vTaskEndScheduler ();
/// }
/// }
/// void vAFunction( void )
/// {
///     // Create at least one task before starting the kernel.
///     xTaskCreate( vTaskCode, "NAME", STACK_SIZE, NULL, tskIDLE_PRIORITY, NULL );
///     // Start the real time kernel with preemption.
///     vTaskStartScheduler ();
///     // Will only get here when the vTaskCode () task has called
///     // vTaskEndScheduler ().  When we get here we are back to single task
///     // execution.
/// }
/// ```

pub fn task_end_scheduler() {
    /* Stop the scheduler interrupts and call the portable scheduler end
       routine so the original ISRs can be restored if necessary.  The port
       layer must ensure interrupts enable bit is left in the correct state. */
    portDISABLE_INTERRUPTS!();
    set_scheduler_running!(false);
    port::port_end_scheduler();
}

/*
 * task. h
 * <pre>void vTaskSuspendAll( void );</pre>
 *
 * Suspends the scheduler without disabling interrupts.  Context switches will
 * not occur while the scheduler is suspended.
 *
 * After calling vTaskSuspendAll () the calling task will continue to execute
 * without risk of being swapped out until a call to xTaskResumeAll () has been
 * made.
 *
 * API functions that have the potential to cause a context switch (for example,
 * vTaskDelayUntil(), xQueueSend(), etc.) must not be called while the scheduler
 * is suspended.
 *
 * Example usage:
   <pre>
 void vTask1( void * pvParameters )
 {
	 for( ;; )
	 {
		 // Task code goes here.

		 // ...

		 // At some point the task wants to perform a long operation during
		 // which it does not want to get swapped out.  It cannot use
		 // taskENTER_CRITICAL ()/taskEXIT_CRITICAL () as the length of the
		 // operation may cause interrupts to be missed - including the
		 // ticks.

		 // Prevent the real time kernel swapping out the task.
		 vTaskSuspendAll ();

		 // Perform the operation here.  There is no need to use critical
		 // sections as we have all the microcontroller processing time.
		 // During this time interrupts will still operate and the kernel
		 // tick count will be maintained.

		 // ...

		 // The operation is complete.  Restart the kernel.
		 xTaskResumeAll ();
	 }
 }
   </pre>
 * \defgroup vTaskSuspendAll vTaskSuspendAll
 * \ingroup SchedulerControl
 */
pub fn task_suspend_all() {
    /* A critical section is not required as the variable is of type
       BaseType_t.  Please read Richard Barry's reply in the following link to a
       post in the FreeRTOS support forum before reporting this as a bug! -
       http://goo.gl/wu4acr */

    // Increment SCHEDULER_SUSPENDED.
    set_scheduler_suspended!(get_scheduler_suspended!() + 1);
}

/*
 * task. h
 * <pre>BaseType_t xTaskResumeAll( void );</pre>
 *
 * Resumes scheduler activity after it was suspended by a call to
 * vTaskSuspendAll().
 *
 * xTaskResumeAll() only resumes the scheduler.  It does not unsuspend tasks
 * that were previously suspended by a call to vTaskSuspend().
 *
 * @return If resuming the scheduler caused a context switch then pdTRUE is
 *		  returned, otherwise pdFALSE is returned.
 *
 * Example usage:
   <pre>
 void vTask1( void * pvParameters )
 {
	 for( ;; )
	 {
		 // Task code goes here.

		 // ...

		 // At some point the task wants to perform a long operation during
		 // which it does not want to get swapped out.  It cannot use
		 // taskENTER_CRITICAL ()/taskEXIT_CRITICAL () as the length of the
		 // operation may cause interrupts to be missed - including the
		 // ticks.

		 // Prevent the real time kernel swapping out the task.
		 vTaskSuspendAll ();

		 // Perform the operation here.  There is no need to use critical
		 // sections as we have all the microcontroller processing time.
		 // During this time interrupts will still operate and the real
		 // time kernel tick count will be maintained.

		 // ...

		 // The operation is complete.  Restart the kernel.  We want to force
		 // a context switch - but there is no point if resuming the scheduler
		 // caused a context switch already.
		 if( !xTaskResumeAll () )
		 {
			  taskYIELD ();
		 }
	 }
 }
   </pre>
 * \defgroup xTaskResumeAll xTaskResumeAll
 * \ingroup SchedulerControl
 */
pub fn task_resume_all() -> bool {
    let already_yielded = false;

    // TODO: This is a recoverable error, use Result<> instead.
    assert!(get_scheduler_suspended!() > pdFALSE as UBaseType,
    "The call to task_resume_all() does not match \
    a previous call to vTaskSuspendAll().");


    /* It is possible that an ISR caused a task to be removed from an event
       list while the scheduler was suspended.  If this was the case then the
       removed task will have been added to the xPendingReadyList.  Once the
       scheduler has been resumed it is safe to move all the pending ready
       tasks from this list into their appropriate ready list. */
    taskENTER_CRITICAL!();
    {
        // Decrement SCHEDULER_SUSPENDED.
        set_scheduler_suspended!(get_scheduler_suspended!() - 1);
        if get_scheduler_suspended!() == pdFALSE as UBaseType {
            if get_current_number_of_tasks!() > 0 {
                /* Move any readied tasks from the pending list into the
                   appropriate ready list. */
                if move_tasks_to_ready_list() {
                    /* A task was unblocked while the scheduler was suspended,
                       which may have prevented the next unblock time from being
                       re-calculated, in which case re-calculate it now.  Mainly
                       important for low power tickless implementations, where
                       this can prevent an unnecessary exit from low power
                       state. */
                    reset_next_task_unblock_time();
                }

                /* If any ticks occurred while the scheduler was suspended then
                   they should be processed now.  This ensures the tick count does
                   not	slip, and that any delayed tasks are resumed at the correct
                   time. */
                process_pended_ticks();

                if get_yield_pending!() {

                    {
                        #![cfg(configUSE_PREEMPTION)]
                        already_yielded = true;
                    }

                    taskYIELD_IF_USING_PREEMPTION!();
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }

            }

        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    }

    already_yielded
}

fn move_tasks_to_ready_list() -> bool {
    /*
     * TODO: Wait until list and TCB is defined.
    while( listLIST_IS_EMPTY( &xPendingReadyList ) == pdFALSE )
    {
        pxTCB = ( TCB_t * ) listGET_OWNER_OF_HEAD_ENTRY( ( &xPendingReadyList ) );
        ( void ) uxListRemove( &( pxTCB->xEventListItem ) );
        ( void ) uxListRemove( &( pxTCB->xStateListItem ) );
        prvAddTaskToReadyList( pxTCB );

        /* If the moved task has a priority higher than the current
           task then a yield must be performed. */
        if( pxTCB->uxPriority >= pxCurrentTCB->uxPriority )
        {
            xYieldPending = pdTRUE;
        }
        else
        {
            mtCOVERAGE_TEST_MARKER();
        }
    }
    */
    false
}

fn reset_next_task_unblock_time() {
    /*
     * TODO: Wait for list and task.

    TCB_t *pxTCB;

    if( listLIST_IS_EMPTY( pxDelayedTaskList ) != pdFALSE )
    {
        /* The new current delayed list is empty.  Set xNextTaskUnblockTime to
           the maximum possible value so it is	extremely unlikely that the
           if( xTickCount >= xNextTaskUnblockTime ) test will pass until
           there is an item in the delayed list. */
        xNextTaskUnblockTime = portMAX_DELAY;
    }
    else
    {
        /* The new current delayed list is not empty, get the value of
           the item at the head of the delayed list.  This is the time at
           which the task at the head of the delayed list should be removed
           from the Blocked state. */
        ( pxTCB ) = ( TCB_t * ) listGET_OWNER_OF_HEAD_ENTRY( pxDelayedTaskList );
        xNextTaskUnblockTime = listGET_LIST_ITEM_VALUE( &( ( pxTCB )->xStateListItem ) );
    }

    */
}

fn process_pended_ticks() {
    let mut pended_counts = get_pended_ticks!();

    if pended_counts > 0 {
        loop {
            if task_increment_tick() {
                set_yield_pending!(true);
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }

            pended_counts -= 1;

            if pended_counts <= 0 {
                break;
            }
        }

        set_pended_ticks!(0);

    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
}

/// Only available when configUSE_TICKLESS_IDLE is set to 1.
/// If tickless mode is being used, or a low power mode is implemented, then
/// the tick interrupt will not execute during idle periods.  When this is the
/// case, the tick count value maintained by the scheduler needs to be kept up
/// to date with the actual execution time by being skipped forward by a time
/// equal to the idle period.
/// 
/// * Implemented by: Fan Jinhao.
/// * C implementation: 
///
/// # Arguments 
/// 
///
/// # Return
/// 
/// Nothing
#[cfg(configUSE_TICKLESS_IDLE)]
pub fn task_step_tick(ticks_to_jump: TickType) {
    /* Correct the tick count value after a period during which the tick
       was suppressed.  Note this does *not* call the tick hook function for
       each stepped tick. */
    let cur_tick_count = get_tick_count!(); // NOTE: Is this a bug in FreeRTOS?
    let next_task_unblock_time = get_next_task_unblock_time!();

    // TODO: Add explanations about this assertion.
    assert!(cur_tick_count + ticks_to_jump <= next_task_unblock_time);

    set_tick_count!(cur_tick_count + ticks_to_jump);

    traceINCREASE_TICK_COUNT!( xTicksToJump );
}

/// THIS FUNCTION MUST NOT BE USED FROM APPLICATION CODE.  IT IS ONLY
/// INTENDED FOR USE WHEN IMPLEMENTING A PORT OF THE SCHEDULER AND IS
/// AN INTERFACE WHICH IS FOR THE EXCLUSIVE USE OF THE SCHEDULER.
///
/// Sets the pointer to the current TCB to the TCB of the highest priority task
/// that is ready to run.
///
/// * Implemented by: Fan Jinhao.
/// * C implementation:
///
/// # Arguments
///
/// # Return
///
/// Nothing
pub fn task_switch_context() {
    if get_scheduler_suspended!() > pdFALSE as UBaseType {
        /* The scheduler is currently suspended - do not allow a context
           switch. */
        set_yield_pending!(true);
    } else {
        set_yield_pending!(false);
        traceTASK_SWITCHED_OUT!();

        #[cfg(configGENERATE_RUN_TIME_STATS)]
        generate_context_switch_stats();

        /* Check for stack overflow, if configured. */
        taskCHECK_FOR_STACK_OVERFLOW!();

        /* Select a new task to run using either the generic Rust or port
           optimised asm code. */
        task_select_highest_priority_task();
        traceTASK_SWITCHED_IN!();

        // TODO: configUSE_NEWLIB_REENTRANT 
    }
}

fn task_select_highest_priority_task() {
    let top_priority: UBaseType = get_top_ready_priority!();
    /*
     * TODO: Wait until these functions and variables are defined.

    /* Find the highest priority queue that contains ready tasks. */
    while( listLIST_IS_EMPTY( &( pxReadyTasksLists[ uxTopPriority ] ) ) )
    {
        assert!(top_prioity > 0, "No task found with a non-zero priority");
        top_priority -= 1;
    }
    /* listGET_OWNER_OF_NEXT_ENTRY indexes through the list, so the tasks of
       the same priority get an equal share of the processor time. */
    listGET_OWNER_OF_NEXT_ENTRY( pxCurrentTCB, &( pxReadyTasksLists[ uxTopPriority ] ) );

    */
    set_top_ready_priority!(top_priority);
}

#[cfg(configGENERATE_RUN_TIME_STATS)]
fn generate_context_switch_stats() {
    // TODO: Wait until CurrentTCB is defined.
    /*
    #ifdef portALT_GET_RUN_TIME_COUNTER_VALUE
    portALT_GET_RUN_TIME_COUNTER_VALUE( ulTotalRunTime );
    #else
    ulTotalRunTime = portGET_RUN_TIME_COUNTER_VALUE();
    #endif

    /* Add the amount of time the task has been running to the
       accumulated time so far.  The time the task started running was
       stored in ulTaskSwitchedInTime.  Note that there is no overflow
       protection here so count values are only valid until the timer
       overflows.  The guard against negative values is to protect
       against suspect run time stat counter implementations - which
       are provided by the application, not the kernel. */
    if( ulTotalRunTime > ulTaskSwitchedInTime )
    {
        pxCurrentTCB->ulRunTimeCounter += ( ulTotalRunTime - ulTaskSwitchedInTime );
    }
    else
    {
        mtCOVERAGE_TEST_MARKER();
    }
    ulTaskSwitchedInTime = ulTotalRunTime;
    */
}

pub fn task_increment_tick() -> bool {
    // TODO: tasks.c 2500
    false
}
