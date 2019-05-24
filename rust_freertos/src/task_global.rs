use crate::*;
use crate::port::{BaseType, UBaseType, TickType};
use crate::list::{LIST, List};
use crate::task_control::TaskHandle;
use std::sync::{Arc, RwLock};

/* Current_TCB and global task lists. */
lazy_static! {

    /* Initialise CURRENT_TCB as early as it is declared rather than when the scheduler starts running.
     * This isn't reasonable actually, but avoided the complexity of using an additional Option<>.
     * Use RwLock to wrap TaskHandle because sometimes we need to change CURRENT_TCB.
     * We use setter and getter to modify CURRENT_TCB, they are defined at the end of this file.
     */
    pub static ref CURRENT_TCB: RwLock<TaskHandle> = RwLock::new(kernel::create_idle_task());

    /* change READY_TASK_LISTS to Arc<RwLock<List>>
     * TODO: Change READT_TASK_LISTS back to `List` because it is too hard to use.
     */
    pub static ref READY_TASK_LISTS: LIST = Arc::new(RwLock::new(
            (0..configMAX_PRIORITIES!())
                .map(|_| List_new!())
                .collect()
                ));

    /* Delayed tasks (two lists are used -
     * one for delays that have overflowed the current tick count.
     */
    pub static ref DELAYED_TASK_LIST1: List = List_new!();
    pub static ref DELAYED_TASK_LIST2: List = List_new!();

    // Points to the delayed task list currently being used.
    pub static ref DELAYED_TASK_LIST: &'static List = &DELAYED_TASK_LIST1;

    /* Points to the delayed task list currently being used
     * to hold tasks that have overflowed the current tick count.
     */
    pub static ref OVERFLOW_DELAYED_TASK_LIST: &'static List = &DELAYED_TASK_LIST2;

    /* Tasks that have been readied while the scheduler was suspended.
     * They will be moved to the ready list when the scheduler is resumed.
     */
    pub static ref PENDING_READY_LIST: List = List_new!();
}

/* Conditionally compiled global lists. */
#[cfg(feature = "INCLUDE_vTaskDelete")]
lazy_static! {
    // Tasks that have been deleted - but their memory not yet freed.
    pub static ref TASKS_WAITING_TERMINATION: List = List_new!();
    pub static ref DELETED_TASKS_WAITING_CLEAN_UP: UBaseType = 0;
}

#[cfg(feature = "INCLUDE_vTaskSuspend")]
lazy_static! {
    // Tasks that are currently suspended.
    pub static ref SUSPENDED_TASK_LIST: List = List_new!();
}

/* ------------------ End global lists ------------------- */

/* Some global variables. */
pub static mut CURRENT_NUMBER_OF_TASKS: UBaseType = 0;
pub static mut TICK_COUNT: TickType = 0;
pub static mut TOP_READY_PRIORITY: UBaseType = 0;
pub static mut PENDED_TICKS: UBaseType = 0;
pub static mut SCHEDULER_RUNNING: bool = false;
pub static mut YIELD_PENDING: bool = false;
pub static mut NUM_OF_OVERFLOWS: BaseType = 0;
pub static mut TASK_NUMBER: UBaseType = 0;
pub static mut NEXT_TASK_UNBLOCK_TIME: TickType = 0;

/* Context switches are held pending while the scheduler is suspended.  Also,
interrupts must not manipulate the xStateListItem of a TCB, or any of the
lists the xStateListItem can be referenced from, if the scheduler is suspended.
If an interrupt needs to unblock a task while the scheduler is suspended then it
moves the task's event list item into the xPendingReadyList, ready for the
kernel to move the task from the pending ready list into the real ready list
when the scheduler is unsuspended.  The pending ready list itself can only be
accessed from a critical section. */
pub static mut SCHEDULER_SUSPENDED: UBaseType = 0;

/* Setters and getters of the above global variables to avoid redundancy of unsafe blocks. */
#[macro_export]
macro_rules! set_scheduler_suspended {
    ($next_val: expr) => (
        unsafe {
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
            crate::task_global::TOP_READY_PRIORITY = $new_top_ready_priority;
        }
    )
}

#[macro_export]
macro_rules! set_pended_ticks {
    ($next_val: expr) => (
        unsafe {
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
            crate::task_global::YIELD_PENDING = $true_or_flase;
        }
    )
}

#[macro_export]
macro_rules! set_current_number_of_tasks {
    ($next_val: expr) => (
        unsafe {
            crate::task_global::CURRENT_NUMBER_OF_TASKS = $next_val
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
            crate::task_global::TICK_COUNT = $next_tick_count;
        }
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
macro_rules! get_current_task_handle {
    () => (
        match crate::task_global::CURRENT_TCB.try_read() {
            Ok(task_handle) => task_handle,
            Err(_) => panic!("Failed to get current task handle")
        }
    )
}

#[macro_export]
macro_rules! set_current_task_handle {
    ($cloned_new_task: expr) => (
        *(crate::task_global::CURRENT_TCB).write().unwrap() = $cloned_new_task
    )
}

#[macro_export]
macro_rules! get_current_task_priority {
    () => (
        get_current_task_handle!().get_priority()
    )
}

#[macro_export]
macro_rules! get_nth_ready_list {
    ($n: expr) => (
        
    )
}

#[macro_export]
macro_rules! switch_delayed_lists {
    () => (
        /* pxDelayedTaskList and pxOverflowDelayedTaskList are switched when the tick
           count overflows. */
        // TODO: tasks.c 239
    )
}
