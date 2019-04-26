// kernel.rs, FreeRTOS scheduler control APIs.
// This file is created by Fan Jinhao.

// Functions defined in this file are explained in Chapter 10.

use crate::*; // TODO: Is this line necessary?
use crate::port::TickType;

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


/*
 * task. h
 * <pre>void vTaskStartScheduler( void );</pre>
 *
 * Starts the real time kernel tick processing.  After calling the kernel
 * has control over which tasks are executed and when.
 *
 * See the demo application file main.c for an example of creating
 * tasks and starting the kernel.
 *
 * Example usage:
   <pre>
 void vAFunction( void )
 {
	 // Create at least one task before starting the kernel.
	 xTaskCreate( vTaskCode, "NAME", STACK_SIZE, NULL, tskIDLE_PRIORITY, NULL );

	 // Start the real time kernel with preemption.
	 vTaskStartScheduler ();

	 // Will not get here unless a task calls vTaskEndScheduler ()
 }
   </pre>
 *
 * \defgroup vTaskStartScheduler vTaskStartScheduler
 * \ingroup SchedulerControl
 */
pub fn task_start_scheduler() {
    
}

/*
 * task. h
 * <pre>void vTaskEndScheduler( void );</pre>
 *
 * NOTE:  At the time of writing only the x86 real mode port, which runs on a PC
 * in place of DOS, implements this function.
 *
 * Stops the real time kernel tick.  All created tasks will be automatically
 * deleted and multitasking (either preemptive or cooperative) will
 * stop.  Execution then resumes from the point where vTaskStartScheduler ()
 * was called, as if vTaskStartScheduler () had just returned.
 *
 * See the demo application file main. c in the demo/PC directory for an
 * example that uses vTaskEndScheduler ().
 *
 * vTaskEndScheduler () requires an exit function to be defined within the
 * portable layer (see vPortEndScheduler () in port. c for the PC port).  This
 * performs hardware specific operations such as stopping the kernel tick.
 *
 * vTaskEndScheduler () will cause all of the resources allocated by the
 * kernel to be freed - but will not free resources allocated by application
 * tasks.
 *
 * Example usage:
   <pre>
 void vTaskCode( void * pvParameters )
 {
	 for( ;; )
	 {
		 // Task code goes here.

		 // At some point we want to end the real time kernel processing
		 // so call ...
		 vTaskEndScheduler ();
	 }
 }

 void vAFunction( void )
 {
	 // Create at least one task before starting the kernel.
	 xTaskCreate( vTaskCode, "NAME", STACK_SIZE, NULL, tskIDLE_PRIORITY, NULL );

	 // Start the real time kernel with preemption.
	 vTaskStartScheduler ();

	 // Will only get here when the vTaskCode () task has called
	 // vTaskEndScheduler ().  When we get here we are back to single task
	 // execution.
 }
   </pre>
 *
 * \defgroup vTaskEndScheduler vTaskEndScheduler
 * \ingroup SchedulerControl
 */
pub fn task_end_scheduler() {
    
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
    false
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
pub fn task_step_tick(ticks_to_jump: TickType) {
    
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
    
}
