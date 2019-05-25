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

    // FIXME  list remove parameters?
    list_remove! ( unblocked_tcb.event_list_item );

    if get_scheduler_suspended!() {
        list_remove! ( unblocked_tcb.state_list_item );
        add_new_task_to_ready_list ( unblocked_tcb );
    }
    else {
        list_insert_end! (xPendingReadyList , unblocked_tcb.event_list_item);
    }

    if( unblocked_tcb.priority > CurrentTCB->priority )
	{
		/* Return true if the task removed from the event list has a higher
		priority than the calling task.  This allows the calling task to know if
		it should force a context switch now. */
		xReturn = true;

		/* Mark that a yield is pending in case the user is not using the
		"xHigherPriorityTaskWoken" parameter to an ISR safe FreeRTOS function. */
		set_yield_pending! (true);
	}
	else
	{
		xReturn = false;
	}

	{
        #![cfg(feature = "configUSE_TICKLESS_IDLE")]
		/* If a task is blocked on a kernel object then xNextTaskUnblockTime
		might be set to the blocked task's time out time.  If the task is
		unblocked for a reason other than a timeout xNextTaskUnblockTime is
		normally left unchanged, because it is automatically reset to a new
		value when the tick count equals xNextTaskUnblockTime.  However if
		tickless idling is used it might be more important to enter sleep mode
		at the earliest possible time - so reset xNextTaskUnblockTime here to
		ensure it is updated at the earliest possible time. */
		reset_next_task_unblock_time ();
	}

	xReturn
}

// TODO : vTaskMissedYield
// * task.c 3076


// TODO : timeout struct
// * task.h 135
// TODO : vTaskSetTimeOutState
// * task.c 3007
// TODO : xTaskCheckForTimeOut
// * task.c 3015