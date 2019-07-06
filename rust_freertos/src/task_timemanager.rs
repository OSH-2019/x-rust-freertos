use crate::kernel::*;
use crate::list::*;
use crate::port::*;
use crate::task_control::*;
use crate::*;
use std::ffi::*;
use std::mem::*;

///  Delay a task for a given number of ticks.  The actual time that the
///  task remains blocked depends on the tick rate.  The constant
///  portTICK_PERIOD_MS can be used to calculate real time from the tick
///  rate - with the resolution of one tick period.
///
///  INCLUDE_vTaskDelay must be defined as 1 for this function to be available.
///  See the configuration section for more information.
///
///
///  vTaskDelay() specifies a time at which the task wishes to unblock relative to
///  the time at which vTaskDelay() is called.  For example, specifying a block
///  period of 100 ticks will cause the task to unblock 100 ticks after
///  vTaskDelay() is called.  vTaskDelay() does not therefore provide a good method
///  of controlling the frequency of a periodic task as the path taken through the
///  code, as well as other task and interrupt activity, will effect the frequency
///  at which vTaskDelay() gets called and therefore the time at which the task
///  next executes.  See vTaskDelayUntil() for an alternative API function designed
///  to facilitate fixed frequency execution.  It does this by specifying an
///  absolute time (rather than a relative time) at which the calling task should
///  unblock.
///
/// * Implemented by: Fan Jinhao
///
/// # Arguments:
///  `ticks_to_delay` The amount of time, in tick periods, that the calling task should block.
///
/// * Return:
///

pub fn task_delay(ticks_to_delay: TickType) {
    let mut already_yielded = false;

    if ticks_to_delay > 0 {
        assert!(get_scheduler_suspended!() == 0);

        task_suspend_all();
        {
            traceTASK_DELAY!();
            add_current_task_to_delayed_list(ticks_to_delay, false);
        }

        already_yielded = task_resume_all();
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }

    if !already_yielded {
        portYIELD_WITHIN_API!();
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
}
