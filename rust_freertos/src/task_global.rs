use crate::*;
use crate::port::{BaseType, UBaseType, TickType};
use crate::list::ListLink;
use crate::task_control::TaskHandle;
use std::sync::RwLock;

/* Some global variables. */
pub static mut TICK_COUNT: TickType = 0;
pub static mut TOP_READY_PRIORITY: UBaseType = 0;
pub static mut PENDED_TICKS: UBaseType = 0;
pub static mut SCHEDULER_RUNNING: bool = false;
pub static mut YIELD_PENDING: bool = false;
pub static mut NUM_OF_OVERFLOWS: BaseType = 0;
pub static mut TASK_NUMBER: UBaseType = 0;
pub static mut NEXT_TASK_UNBLOCK_TIME: TickType = 0;
pub static mut CURRENT_NUMBER_OF_TASKS: UBaseType = 0;

/* GLOBAL TASK LISTS ARE CHANGED TO INTEGERS, WHICH ARE THEIR IDS. */

/* Current_TCB and global task lists. */
lazy_static! {
    /* Initialise CURRENT_TCB as early as it is declared rather than when the scheduler starts running.
     * This isn't reasonable actually, but avoided the complexity of using an additional Option<>.
     * Use RwLock to wrap TaskHandle because sometimes we need to change CURRENT_TCB.
     * We use setter and getter to modify CURRENT_TCB, they are defined at the end of this file.
     */
    pub static ref CURRENT_TCB: RwLock<Option<TaskHandle>> = RwLock::new(None);
    pub static ref READY_TASK_LISTS: [ListLink; configMAX_PRIORITIES!()] = Default::default();

    /* Delayed tasks (two lists are used - one for delays that have overflowed the current tick count.
    */
    // Points to the delayed task list currently being used.
    pub static ref DELAYED_TASK_LIST: ListLink = Default::default();

    /* Points to the delayed task list currently being used
     * to hold tasks that have overflowed the current tick count.
     */
    pub static ref OVERFLOW_DELAYED_TASK_LIST: ListLink = Default::default();

    /* Tasks that have been readied while the scheduler was suspended.
     * They will be moved to the ready list when the scheduler is resumed.
     */
    pub static ref PENDING_READY_LIST: ListLink = Default::default();
}

#[cfg(feature = "INCLUDE_vTaskDelete")]
lazy_static! {
    // Tasks that have been deleted - but their memory not yet freed.
    pub static ref TASKS_WAITING_TERMINATION: ListLink = Default::default();
    pub static ref DELETED_TASKS_WAITING_CLEAN_UP: ListLink = Default::default();
}

#[cfg(feature = "INCLUDE_vTaskSuspend")]
lazy_static! {
    // Tasks that are currently suspended.
    pub static ref SUSPENDED_TASK_LIST: ListLink = Default::default();
}
/* ------------------ End global lists ------------------- */

/* Context switches are held pending while the scheduler is suspended.  Also,
interrupts must not manipulate the xStateListItem of a TCB, or any of the
lists the xStateListItem can be referenced from, if the scheduler is suspended.
*/
pub static mut SCHEDULER_SUSPENDED: UBaseType = 0;

/*< Holds the value of a timer/counter the last time a task was switched in. */
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
pub static mut TASK_SWITCHED_IN_TIME: u32 = 0;

/*< Holds the total amount of execution time as defined by the run time counter clock. */
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
pub static mut TOTAL_RUN_TIME: u32 = 0;

/* Setters and getters of the above global variables to avoid redundancy of unsafe blocks. */
#[macro_export]
macro_rules! set_scheduler_suspended {
    ($next_val: expr) => (
        unsafe {
            trace!("SCHEDULER_SUSPENDED was set to {}", $next_val);
            crate::task_global::SCHEDULER_SUSPENDED = $next_val;
        }
    )
}

#[macro_export]
macro_rules! get_scheduler_suspended {
    () => (
        unsafe {
            crate::task_global::SCHEDULER_SUSPENDED
        }
    )
}

#[macro_export]
macro_rules! get_top_ready_priority {
    () => (
        unsafe {
            crate::task_global::TOP_READY_PRIORITY
        }
    )
}

#[macro_export]
macro_rules! set_top_ready_priority {
    ($new_top_ready_priority: expr) => (
        unsafe {
            trace!("TOP_READY_PRIORITY was set to {}", $new_top_ready_priority);
            crate::task_global::TOP_READY_PRIORITY = $new_top_ready_priority;
        }
    )
}

#[macro_export]
macro_rules! set_pended_ticks {
    ($next_val: expr) => (
        unsafe {
            trace!("PENDED_TICKS was set to {}", $next_val);
            crate::task_global::PENDED_TICKS = $next_val
        }
    )
}

#[macro_export]
macro_rules! get_pended_ticks {
    () => (
        unsafe {
            crate::task_global::PENDED_TICKS
        }
    )
}

#[macro_export]
macro_rules! set_task_number {
    ($next_val: expr) => (
        unsafe {
            trace!("TASK_NUMBER was set to {}", $next_val);
            crate::task_global::TASK_NUMBER = $next_val
        }
    )
}

#[macro_export]
macro_rules! get_task_number {
    () => (
        unsafe {
            crate::task_global::TASK_NUMBER
        }
    )
}

#[macro_export]
macro_rules! get_yield_pending {
    () => (
        unsafe {
            crate::task_global::YIELD_PENDING
        }
    )
}

#[macro_export]
macro_rules! set_yield_pending {
    ($true_or_flase: expr) => (
        unsafe {
            trace!("YIELD_PENDING was set to {}", $true_or_flase);
            crate::task_global::YIELD_PENDING = $true_or_flase;
        }
    )
}

#[macro_export]
macro_rules! set_current_number_of_tasks {
    ($next_val: expr) => (
        unsafe {
            trace!("CURRENT_NUMBER_OF_TASKS was set to {}", $next_val);
            crate::task_global::CURRENT_NUMBER_OF_TASKS = $next_val;
        }
    )
}

#[macro_export]
macro_rules! get_current_number_of_tasks {
    () => (
        unsafe {
            crate::task_global::CURRENT_NUMBER_OF_TASKS
        }
    )
}

#[macro_export]
macro_rules! set_scheduler_running {
    ($true_or_flase: expr) => (
        unsafe {
            trace!("SCHEDULER_RUNNING was set to {}", $true_or_flase);
            crate::task_global::SCHEDULER_RUNNING = $true_or_flase
        }
    )
}

#[macro_export]
macro_rules! get_scheduler_running {
    () => (
        unsafe {
            crate::task_global::SCHEDULER_RUNNING
        }
    )
}


#[macro_export]
macro_rules! get_next_task_unblock_time {
    () => (
        unsafe {
            crate::task_global::NEXT_TASK_UNBLOCK_TIME
        }
    )
}

#[macro_export]
macro_rules! set_next_task_unblock_time {
    ($new_time: expr) => (
        unsafe {
            trace!("NEXT_TASK_UNBLOCK_TIME was set to {}", $new_time);
            crate::task_global::NEXT_TASK_UNBLOCK_TIME = $new_time;
        }
    )
}

#[macro_export]
macro_rules! get_tick_count {
    () => (
        unsafe {
            crate::task_global::TICK_COUNT
        }
    )
}

#[macro_export]
macro_rules! set_tick_count {
    ($next_tick_count: expr) => (
        unsafe {
            trace!("TICK_COUNT was set to {}", $next_tick_count);
            crate::task_global::TICK_COUNT = $next_tick_count;
        }
    )
}

#[macro_export]
macro_rules! get_num_of_overflows {
    () => (
        unsafe {
            crate::task_global::NUM_OF_OVERFLOWS
        }
    )
}

#[macro_export]
macro_rules! set_num_of_overflows {
    ($next_tick_count: expr) => (
        unsafe {
            trace!("NUM_OF_OVERFLOWS was set to {}", $next_tick_count);
            crate::task_global::NUM_OF_OVERFLOWS = $next_tick_count;
        }
    )
}

#[macro_export]
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
macro_rules! set_total_run_time {
    ($next_val: expr) => (
        unsafe {
            trace!("TOTAL_RUN_TIME was set to {}", $next_val);
            TOTAL_RUN_TIME = $next_val
        }
    )
}

#[macro_export]
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
macro_rules! set_task_switch_in_time {
    ($next_val: expr) => (
        unsafe {
            trace!("TASK_SWITCHED_IN_TIME was set to {}", $next_val);
            TASK_SWITCHED_IN_TIME = $next_val
        }
    )
}

#[macro_export]
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
macro_rules! get_total_run_time {
    () => (
        unsafe {
            TOTAL_RUN_TIME
        }
    )
}

#[macro_export]
#[cfg(feature = "configGENERATE_RUN_TIME_STATS")]
macro_rules! get_task_switch_in_time {
    () => (
        unsafe {
            TASK_SWITCHED_IN_TIME
        }
    )
}

#[macro_export]
macro_rules! get_current_task_handle_wrapped {
    () => (
        // NOTE: This macro WILL be deprecated. So please avoid using this macro.
        crate::task_global::CURRENT_TCB.read().unwrap().as_ref()
    )
}

#[macro_export]
macro_rules! get_current_task_handle {
    () => (
        crate::task_global::CURRENT_TCB.read().unwrap().as_ref().unwrap().clone()
    )
}

#[macro_export]
macro_rules! set_current_task_handle {
    ($cloned_new_task: expr) => (
        trace!("CURRENT_TCB changed!");
        *(crate::task_global::CURRENT_TCB).write().unwrap()= Some($cloned_new_task)
    )
}

#[macro_export]
macro_rules! get_current_task_priority {
    () => (
        get_current_task_handle!().get_priority()
    )
}

#[cfg(feature = "INCLUDE_xTaskAbortDelay")]
#[macro_export]
macro_rules! get_current_task_delay_aborted {
    () => (
        get_current_task_handle!().get_delay_aborted()
    )
}
/* ---------- End of global variable setters and getters -----------*/

#[macro_export]
macro_rules! taskCHECK_FOR_STACK_OVERFLOW {
    () => (
        // This macro does nothing.
    )
}

#[macro_export]
macro_rules! switch_delayed_lists {
    () => (
        /* pxDelayedTaskList and pxOverflowDelayedTaskList are switched when the tick
           count overflows. */
        // TODO: tasks.c 239
        unsafe {
            let mut delayed = DELAYED_TASK_LIST.write().unwrap();
            let mut overflowed = OVERFLOW_DELAYED_TASK_LIST.write().unwrap();
            let tmp = (*delayed).clone();
            *delayed = (*overflowed).clone();
            *overflowed = tmp;
        }
    )
}
