// kernel.rs, FreeRTOS scheduler control APIs.
// This file is created by Fan Jinhao.
// Functions defined in this file are explained in Chapter 9 and 10.

use crate::*; // TODO: Is this line necessary?
use crate::port::{TickType, UBaseType};
use crate::projdefs::pdFALSE;
use crate::task_control::{TaskHandle, TCB};
use crate::task_global::*;
use std::sync::{Arc, RwLock};
// use crate::task_control::TCB;

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
        #[cfg(feature = "configUSE_PREEMPTION")]
        portYIELD_WITHIN_API!();
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
/// ```
/// task_control::TCB::new()
///                   .name("NAME")
///                   .priority(1)
///                   .initialise(|| println!("Hello world!"))
///
/// // Start the real time kernel with preemption.
/// task_start_scheduler();
///
/// // Will not get here unless a task calls vTaskEndScheduler ()
/// ```
pub fn task_start_scheduler() {
    #[cfg(feature = "configUSE_TIMERS")]
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
pub fn create_idle_task() -> TaskHandle{
    let idle_task_fn = | | {
        loop {
            /* THIS IS THE RTOS IDLE TASK - WHICH IS CREATED AUTOMATICALLY WHEN THE
               SCHEDULER IS STARTED. */
        
            /* See if any tasks have deleted themselves - if so then the idle task
               is responsible for freeing the deleted task's TCB and stack. */
            check_tasks_waiting_termination();

            /* If we are not using preemption we keep forcing a task switch to
               see if any other task has become available.  If we are using
               preemption we don't need to do this as any task becoming available
               will automatically get the processor anyway. */
            #[cfg(not(feature = "configUSE_PREEMPTION"))]
            taskYIELD!();

            {
                #![cfg(all(feature = "configUSE_PREEMPTION", feature = "configIDLE_SHOULD_YIELD"))]
                /* When using preemption tasks of equal priority will be
                   timesliced.  If a task that is sharing the idle priority is ready
                   to run then the idle task should yield before the end of the
                   timeslice.

                   A critical region is not required here as we are just reading from
                   the list, and an occasional incorrect value will not matter.  If
                   the ready list at the idle priority contains more than one task
                   then a task other than the idle task is ready to execute. */
                if current_list_length!(nth_ready_list!(0)) > 1
                {
                    taskYIELD!();
                }
                else
                {
                    mtCOVERAGE_TEST_MARKER!();
                }
            }

            {
                #![cfg(feature = "configUSE_IDLE_HOOK")]
                // TODO: Use IdleHook
                // extern void vApplicationIdleHook( void );

                /* Call the user defined function from within the idle task.  This
                   allows the application designer to add background functionality
                   without the overhead of a separate task.
                   NOTE: vApplicationIdleHook() MUST NOT, UNDER ANY CIRCUMSTANCES,
                   CALL A FUNCTION THAT MIGHT BLOCK. */
                // vApplicationIdleHook();
                trace!("Idle Task running");
            }
        }
    };

    TCB::new()
        .priority(0)
        .name("Idle")
        .initiailise(idle_task_fn)
        .unwrap_or_else(|err| panic!("Idle task creation failed with error: {:?}", err))
}

fn check_tasks_waiting_termination() {
    // TODO: Wait for task_delete.
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
    // TODO: This function relies on the software timer, which we may not implement.
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
/// task_control::TCB::new()
///                   .name("NAME")
///                   .priority(1)
///                   .initialise(|| { loop {
///                         println!("Task Code called successfully!");
///                         // Task code goes here.
///                         // At some point we want to end the real time kernel processing
///                         // so call ...
///                         task_end_scheduler();
///                     });
///
/// // Start the real time kernel with preemption.
/// task_start_scheduler();
/// // Will only get here when the vTaskCode () task has called
/// // vTaskEndScheduler ().  When we get here we are back to single task
/// // execution.
///
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
                        #![cfg(feature = "configUSE_PREEMPTION")]
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
    let has_unblocked_task = false;
    while !list_is_empty!(PENDING_READY_LIST) {
        let task_handle = TaskHandle::from_arc(
            get_owner_of_head_entry!(PENDING_READY_LIST).unwrap());
        let event_list_item = task_handle.get_event_list_item();
        let state_list_item = task_handle.get_state_list_item();

        if let Some(list) = get_list_item_container!(event_list_item) {
            list_remove!(list, state_list_item);
        } else {
            warn!("State_list_item of task {} should have container", 
                  task_handle.get_name());
        }

        if let Some(list) = get_list_item_container!(event_list_item) {
            list_remove!(list, state_list_item);
        } else {
            warn!("State_list_item of task {} should have container", 
                  task_handle.get_name());
        }

        task_handle.add_task_to_ready_list();

        /* If the moved task has a priority higher than the current
           task then a yield must be performed. */
        if task_handle.get_priority() >= get_current_task_priority!()
        {
            set_yield_pending!(true);
        }
        else
        {
            mtCOVERAGE_TEST_MARKER!();
        }
    }
    false
}

fn reset_next_task_unblock_time() {
    if list_is_empty!( DELAYED_TASK_LIST )
    {
        /* The new current delayed list is empty.  Set xNextTaskUnblockTime to
           the maximum possible value so it is	extremely unlikely that the
           if( xTickCount >= xNextTaskUnblockTime ) test will pass until
           there is an item in the delayed list. */
        set_next_task_unblock_time!(port::portMAX_DELAY);
    }
    else
    {
        /* The new current delayed list is not empty, get the value of
           the item at the head of the delayed list.  This is the time at
           which the task at the head of the delayed list should be removed
           from the Blocked state. */
        let task_handle = TaskHandle::from_arc(
            get_owner_of_head_entry!(DELAYED_TASK_LIST).unwrap()
            );
        set_next_task_unblock_time!(
            get_list_item_value!(task_handle.get_state_list_item())
            );
    }
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
#[cfg(feature = "configUSE_TICKLESS_IDLE")]
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

        #[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
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

    /* Find the highest priority queue that contains ready tasks. */
    while list_is_empty!(nth_ready_list!(top_priority))
    {
        assert!(top_priority > 0, "No task found with a non-zero priority");
        top_priority -= 1;
    }

    /* listGET_OWNER_OF_NEXT_ENTRY indexes through the list, so the tasks of
       the same priority get an equal share of the processor time. */
    let next_task = TaskHandle::from_arc(
        get_owner_of_next_entry!(
            nth_ready_list!(top_priority),
            get_current_task_handle!().get_state_list_item()
            ).unwrap()
        );

    set_current_task_handle!(next_task);

    set_top_ready_priority!(top_priority);
}

#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
fn generate_context_switch_stats() {
    /*
    #ifdef portALT_GET_RUN_TIME_COUNTER_VALUE
    portALT_GET_RUN_TIME_COUNTER_VALUE( ulTotalRunTime );
    #else
    ulTotalRunTime = portGET_RUN_TIME_COUNTER_VALUE();
    #endif
    */
    let total_run_time = portGET_RUN_TIME_COUNTER_VALUE!() as u32;
    trace!("Total runtime: {}", total_run_time);
    set_total_run_time!(total_run_time);
    
    /* Add the amount of time the task has been running to the
       accumulated time so far.  The time the task started running was
       stored in ulTaskSwitchedInTime.  Note that there is no overflow
       protection here so count values are only valid until the timer
       overflows.  The guard against negative values is to protect
       against suspect run time stat counter implementations - which
       are provided by the application, not the kernel. */
    let task_switched_in_time = get_task_switch_in_time!();
    if total_run_time > task_switched_in_time
    {
        let current_task = get_current_task_handle!();
        let old_run_time = current_task.get_run_time();
        current_task.set_run_time(old_run_time + total_run_time - task_switched_in_time);
    }
    else
    {
        mtCOVERAGE_TEST_MARKER!();
    }
    set_task_switch_in_time!(total_run_time);
}

pub fn task_increment_tick() -> bool {
    // TODO: tasks.c 2500
    let mut switch_required = false;

    /* Called by the portable layer each time a tick interrupt occurs.
       Increments the tick then checks to see if the new tick value will cause any
       tasks to be unblocked. */
    traceTASK_INCREMENT_TICK!( get_tick_count!() );

    if get_scheduler_suspended!() != pdFALSE as UBaseType {
        /* Minor optimisation.  The tick count cannot change in this
           block. */
        let const_tick_count = get_tick_count!() + 1;

        /* Increment the RTOS tick, switching the delayed and overflowed
           delayed lists if it wraps to 0. */
        set_tick_count!(const_tick_count);

        if const_tick_count == 0 {
            // NOTE: This macro has yet been implemented.
            switch_delayed_lists!();
        }
        else {
            mtCOVERAGE_TEST_MARKER!();
        }

        /* See if this tick has made a timeout expire.  Tasks are stored in
           the	queue in the order of their wake time - meaning once one task
           has been found whose block time has not expired there is no need to
           look any further down the list. */
        if const_tick_count >= get_next_task_unblock_time!()
        {
            loop {
                if list_is_empty!( DELAYED_TASK_LIST ) {
                    /* The delayed list is empty.  Set xNextTaskUnblockTime
                       to the maximum possible value so it is extremely
                       unlikely that the
                       if( xTickCount >= xNextTaskUnblockTime ) test will pass
                       next time through. */
                    set_next_task_unblock_time!(port::portMAX_DELAY);
                    break;
                }
                else {
                    /* The delayed list is not empty, get the value of the
                       item at the head of the delayed list.  This is the time
                       at which the task at the head of the delayed list must
                       be removed from the Blocked state. */
                    let delay_head_entry_owner = get_owner_of_head_entry!(DELAYED_TASK_LIST).unwrap();
                    // TODO: This is probably an error of `list`
                    let task_handle = TaskHandle::from_arc(delay_head_entry_owner);
                    let state_list_item = task_handle.get_state_list_item();
                    let event_list_item = task_handle.get_event_list_item();
                    let item_value = get_list_item_value!(state_list_item);

                    if const_tick_count < item_value
                    {
                        /* It is not time to unblock this item yet, but the
                           item value is the time at which the task at the head
                           of the blocked list must be removed from the Blocked
                           state -	so record the item value in
                           xNextTaskUnblockTime. */
                        set_next_task_unblock_time!(item_value);
                        break;
                    }
                    else
                    {
                        mtCOVERAGE_TEST_MARKER!();
                    }

                    /* It is time to remove the item from the Blocked state. */
                    if let Some(list) = get_list_item_container!(state_list_item) {
                        list_remove!(list, state_list_item);
                    } else {
                        warn!("State_list_item of task {} doesn't have container", 
                              task_handle.get_name());
                    }

                    /* Is the task waiting on an event also?  If so remove
                       it from the event list. */
                    if let Some(list) = get_list_item_container!(event_list_item) {
                        list_remove!(list, event_list_item);
                    }
                    else {
                        trace!("Task {} isn't waiting on an event list", task_handle.get_name());
                    }

                    /* Place the unblocked task into the appropriate ready
                       list. */
                    task_handle.add_task_to_ready_list();

                    /* A task being unblocked cannot cause an immediate
                       context switch if preemption is turned off. */
                    {
                        #![cfg(feature = "configUSE_PREEMPTION")]
                        /* Preemption is on, but a context switch should
                           only be performed if the unblocked task has a
                           priority that is equal to or higher than the
                           currently executing task. */
                        if task_handle.get_priority() >= get_current_task_priority!()
                        {
                            switch_required = true;
                        }
                        else
                        {
                            mtCOVERAGE_TEST_MARKER!();
                        }
                    }
                }
            }
        }

        /* Tasks of equal priority to the currently running task will share
           processing time (time slice) if preemption is on, and the application
           writer has not explicitly turned time slicing off. */
        {
            #![cfg(all(feature = "configUSE_PREEMPTION", feature = "configUSE_TIME_SLICING"))]
            if current_list_length!(nth_ready_list!(get_current_task_priority!())) > 1 {
                switch_required = true;
            }
            else {
                mtCOVERAGE_TEST_MARKER!();
            }
        }

        {
            #![cfg(feature = "configUSE_TICK_HOOK")]
            /* Guard against the tick hook being called when the pended tick
               count is being unwound (when the scheduler is being unlocked). */
            if get_pended_ticks!() == 0
            {
                // vApplicationTickHook();
            }
            else
            {
                mtCOVERAGE_TEST_MARKER!();
            }
        }
    }
    switch_required
}
