use crate::port::*;
use crate::list::*;
// use crate::kernel::*;
use crate::*;
use crate::task_control::*;
use crate::task_global::*;
use std::sync::Arc;

// TODO : vTaskRemoveFromEventList
// * task.c 2894

pub fn task_remove_from_event_list (event_list: List) -> bool {
    let unblocked_tcb = get_owner_of_head_entry!(event_list).unwrap();
    let unblocked_tcb = TaskHandle::from_arc(unblocked_tcb);
    // configASSERT( unblocked_tcb );
    let mut xreturn: bool = false;

    list_remove! ( unblocked_tcb.get_event_list_item() );

    if get_scheduler_suspended!() > 0 {
        list_remove! ( unblocked_tcb.get_state_list_item() );
        unblocked_tcb.add_task_to_ready_list().unwrap();
    } else {
        list_insert_end! (get_list!(PENDING_READY_LIST) , unblocked_tcb.get_event_list_item());
    }

    if unblocked_tcb.get_priority() > get_current_task_priority!()
    {
        /* Return true if the task removed from the event list has a higher
           priority than the calling task.  This allows the calling task to know if
           it should force a context switch now. */
        xreturn = true;

        /* Mark that a yield is pending in case the user is not using the
           "xHigherPriorityTaskWoken" parameter to an ISR safe FreeRTOS function. */
        set_yield_pending! (true);
    }
    else
    {
        xreturn = false;
    }

    {
        #![cfg(feature = "configUSE_TICKLESS_IDLE")]
        reset_next_task_unblock_time ();
    }

    xreturn
}

// TODO : vTaskMissedYield
// * task.c 3076

pub fn task_missed_yield() {
    set_yield_pending! (false);
}

// TODO : timeout struct
// * task.h 135

#[derive(Debug)]
pub struct time_out {
    overflow_count: BaseType,
    time_on_entering: TickType,
}


// TODO : vTaskSetTimeOutState
// * task.c 3007

pub fn task_set_time_out_state ( pxtimeout: &mut time_out ){
    // assert! ( pxtimeout );
    pxtimeout.overflow_count = get_num_of_overflows!();
    pxtimeout.time_on_entering = get_tick_count!();
}

//  TODO : xTaskCheckForTimeOut
// * task.c 3015

fn task_check_for_timeout (pxtimeout: time_out, ticks_to_wait: TickType) -> (time_out, TickType, bool){
    let mut pxtimeout = pxtimeout;
    let mut xreturn: bool = false;
    let mut ticks_to_wait = ticks_to_wait;
    // assert! (pxtimeout);
    // assert! (ticks_to_wait);

    taskENTER_CRITICAL! ();
    {
        let const_tick_count: TickType = get_tick_count!();
        let unwrapped_cur = get_current_task_handle!();
        let mut cfglock1 = false;
        let mut cfglock2 = false;

        {
            #![cfg(feature = "INCLUDE_xTaskAbortDelay")]
            cfglock1 = true;
        }

        {
            #![cfg(feature = "INCLUDE_vTaskSuspend")]
            cfglock2 = true;
        }


        if cfglock1 && unwrapped_cur.get_delay_aborted() {
            unwrapped_cur.set_delay_aborted(false);
            xreturn = true;
        }

        if cfglock2 && ticks_to_wait == portMAX_DELAY {
            xreturn = false;
        }

        if 0 != pxtimeout.overflow_count && const_tick_count >= pxtimeout.time_on_entering
        {
            xreturn = true;
        }
        else if const_tick_count - pxtimeout.time_on_entering  < ticks_to_wait{
            ticks_to_wait -= const_tick_count - pxtimeout.time_on_entering;
            task_set_time_out_state (&mut pxtimeout);
            xreturn = false;
        } else {
            xreturn = true;
        }
    }
    taskEXIT_CRITICAL! ();

    (pxtimeout, ticks_to_wait, xreturn)
}

// TODO : vTaskPlaceOnEventList
// * tasks.c 2820
pub fn task_place_on_event_list (event_list: List, ticks_to_wait: TickType) {
    // assert! ( event_list );

    /* THIS FUNCTION MUST BE CALLED WITH EITHER INTERRUPTS DISABLED OR THE
       SCHEDULER SUSPENDED AND THE QUEUE BEING ACCESSED LOCKED. */

    /* Place the event list item of the TCB in the appropriate event list.
       This is placed in the list in priority order so the highest priority task
       is the first to be woken by the event.  The queue that contains the event
       list is locked, preventing simultaneous access from interrupts. */

    let unwrapped_cur = get_current_task_handle!();
    list_insert!( event_list, &( unwrapped_cur.get_event_list_item() ) );

    add_current_task_to_delayed_list( ticks_to_wait, true );
}
