use crate::port::*;
use crate::list::*;
use crate::kernel::*;
use crate::*;
use std::ffi::*;
use std::mem::*;
use crate::task_control::*;

// TODO : vTaskRemoveFromEventList
// * task.c 2894

pub fn task_remove_from_event_list (event_list: List) -> bool {
    let mut unblocked_tcb = get_owner_of_head_entry! (event_list);
    configASSERT( unblocked_tcb );
	let mut xreturn: bool = false;

    list_remove! ( unblocked_tcb.event_list_item );

    if get_scheduler_suspended!() {
        list_remove! ( unblocked_tcb.state_list_item );
        add_new_task_to_ready_list ( unblocked_tcb );
    }
    else {
        list_insert_end! (xPendingReadyList , unblocked_tcb.event_list_item);
    }

    if( unblocked_tcb.priority > get_current_task_priority!() )
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

// TODO : timeout struct
// * task.h 135

#[derive(Debug)]
struct time_out {
	overflow_count: BaseType;
	time_on_entering: TickType;
};


// TODO : vTaskSetTimeOutState
// * task.c 3007

pub fn task_set_time_out_state ( pxtimeout: &mut time_out ){
	assert! ( pxtimeout );
	pxtimeout.overflow_count = NUM_OF_OVERFLOWS;
	pxtimeout.time_on_entering = TICK_COUNT;
}

// TODO : xTaskCheckForTimeOut
// * task.c 3015

pub fn task_check_for_timeout (pxtimeout: time_out, ticks_to_wait: TickType) -> BaseType {
	let mut xreturn: BaseType = false;
	assert! (pxtimeout);
	assert! (ticks_to_wait);

	taskENTER_CRITICAL! ();
	{
		const const_tick_count: TickType = TICK_COUNT;
		let unwrapped_cur = get_current_task_handle!();

		{
			#![cfg(feature = "INCLUDE_xTaskAbortDelay")]
			if unwrapped_cur.delay_aborted {
				unwrapped_cur.set_delayed_aborted(false);
				xreturn = true;
			}
			else
		}

		{
			#![cfg(feature = "INCLUDE_vTaskSuspend")]
			if ticks_to_wait == portMAX_DELAY {
				xreturn = false;
			}
			else
		}

		if 0 != pxtimeout.overflow_count && const_tick_count >= pxtimeout.time_on_entering
		{
			xreturn = true;
		}
		else if const_tick_count - pxtimeout.time_on_entering  < ticks_to_wait{
			ticks_to_wait -= const_tick_count - pxtimeout.time_on_entering;
			task_set_time_out_state ();
			xreturn = false;
		} else {
			xreturn = true;
		}
	}
	taskEXIT_CRITICAL! ();

	xreturn
}
