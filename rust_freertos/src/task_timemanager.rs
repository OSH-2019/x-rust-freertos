use crate::port::*;
use crate::list::*;
use crate::kernel::*;
use crate::task_control::*;
use crate::*;
use std::ffi::*;
use std::mem::*;

pub fn task_delay (ticks_to_delay:TickType) {
    let mut already_yielded = false;

    if ticks_to_delay > 0 {
        assert! (get_scheduler_suspended!() == 0);

        task_suspend_all ();
        {
            traceTASK_DELAY! ();
            add_current_task_to_delayed_list (ticks_to_delay,false);
        }

        already_yielded = task_resume_all ();
    }
    else {
        mtCOVERAGE_TEST_MARKER!();
    }

    if !already_yielded {
        portYIELD_WITHIN_API!();
    }
    else {
        mtCOVERAGE_TEST_MARKER!();
    }
}

// pub fn task_delay_until (previous_wait_time: TickType, time_increasement: TickType) {
//     let mut time_to_wait: TickType = 0;
//     let mut already_yielded = false;
//     let mut should_delay = false;

//     assert! (previous_wait_time);
//     assert! (time_increasement > 0);
//     assert! (get_scheduler_suspended!() == 0);

//     task_suspend_all ();
//     {
//         const const_tick_count: TickType = get_tick_count! ();
//     }
// }