use crate::port::{BaseType, UBaseType, TickType};

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
            crate::task_global::SCHEDULER_SUSPENDED = $next_val
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

