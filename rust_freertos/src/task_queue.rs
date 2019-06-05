use crate::port::*;
use crate::list::*;
// use crate::kernel::*;
use crate::*;
use crate::task_control::*;
use crate::task_global::*;
use std::sync::Arc;

/* The item value of the event list item is normally used to hold the priority
of the task to which it belongs (coded to allow it to be held in reverse
priority order).  However, it is occasionally borrowed for other purposes.  It
is important its value is not updated due to a task priority change while it is
being used for another purpose.  The following bit definition is used to inform
the scheduler that the value should not be changed - in which case it is the
responsibility of whichever module is using the value to ensure it gets set back
to its original value when it is released. */
#[cfg(feature = "configUSE_16_BIT_TICKS")]
const taskEVENT_LIST_ITEM_VALUE_IN_USE: TickType = 0x8000;
#[cfg(not(feature = "configUSE_16_BIT_TICKS"))]
const taskEVENT_LIST_ITEM_VALUE_IN_USE: TickType = 0x80000000;

// TODO : vTaskRemoveFromEventList
// * task.c 2894

pub fn task_remove_from_event_list (event_list:& List) -> bool {
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

#[derive(Debug,Default)]
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

pub fn task_check_for_timeout (pxtimeout: &mut time_out, ticks_to_wait: &mut TickType) -> bool {
    let mut xreturn: bool = false;
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

        if cfglock2 && *ticks_to_wait == portMAX_DELAY {
            xreturn = false;
        }

        if 0 != pxtimeout.overflow_count && const_tick_count >= pxtimeout.time_on_entering
        {
            xreturn = true;
        }
        else if const_tick_count - pxtimeout.time_on_entering  < *ticks_to_wait{
            *ticks_to_wait -= const_tick_count - pxtimeout.time_on_entering;
            task_set_time_out_state (pxtimeout);
            xreturn = false;
        } else {
            xreturn = true;
        }
    }
    taskEXIT_CRITICAL! ();

    xreturn
}

// TODO : vTaskPlaceOnEventList
// * tasks.c 2820
pub fn task_place_on_event_list (event_list:& List, ticks_to_wait: TickType) {
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

#[cfg(feature = "configUSE_MUTEXES")]
pub fn task_increment_mutex_held_count() {
    /* If xSemaphoreCreateMutex() is called before any tasks have been created
       then pxCurrentTCB will be NULL. */
    match get_current_task_handle_wrapped!() {
        Some(current_task) => {
            let new_val = current_task.get_mutex_held_count() + 1;
            current_task.set_mutex_held_count(new_val)
        },
        None => ()
    }
}

#[cfg(feature = "configUSE_MUTEXES")]
pub fn task_priority_inherit(mutex_holder: Option<TaskHandle>) {
    /* NOTE by Fan Jinhao: Maybe mutex_holder should be `&Option<TaskHandle>`.
     * But I'll leave it for now.
     */

    /* If the mutex was given back by an interrupt while the queue was
       locked then the mutex holder might now be NULL. */
    if let Some(task) = mutex_holder {
        /* If the holder of the mutex has a priority below the priority of
           the task attempting to obtain the mutex then it will temporarily
           inherit the priority of the task attempting to obtain the mutex. */
        let current_task_priority = get_current_task_priority!();
        let this_task_priority = task.get_priority();

        if this_task_priority < current_task_priority
        {
            /* Adjust the mutex holder state to account for its new
               priority.  Only reset the event list item value if the value is
               not being used for anything else. */
            let event_list_item = task.get_event_list_item();
            if (get_list_item_value!(event_list_item)
                & taskEVENT_LIST_ITEM_VALUE_IN_USE) == 0 {
                let new_item_val = (configMAX_PRIORITIES!() - current_task_priority) as TickType;
                set_list_item_value!( event_list_item, new_item_val);
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }

            /* If the task being modified is in the ready state it will need
               to be moved into a new list. */
            let state_list_item = task.get_state_list_item();
            if is_contained_within!( nth_ready_list!(this_task_priority), state_list_item) {
                if list_remove!(state_list_item) ==  0 {
                    taskRESET_READY_PRIORITY!( this_task_priority );
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }

                /* Inherit the priority before being moved into the new list. */
                task.set_priority(current_task_priority);
                task.add_task_to_ready_list().unwrap();
            }
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
}

#[cfg(feature = "configUSE_MUTEXES")]
pub fn task_priority_disinherit(mutex_holder: Option<TaskHandle>) -> bool{
    /* NOTE by Fan Jinhao: Maybe mutex_holder should be `&Option<TaskHandle>`.
     * But I'll leave it for now.
     */
    let mut ret_val: bool = false;

    if let Some(task) = mutex_holder {
        /* A task can only have an inherited priority if it holds the mutex.
           If the mutex is held by a task then it cannot be given from an
           interrupt, and if a mutex is given by the holding task then it must
           be the running state task. */

        // TODO: is_current_task(). configASSERT( pxTCB == pxCurrentTCB );

        let mutex_held = task.get_mutex_held_count();
        assert!(mutex_held > 0);
        let mutex_held = mutex_held - 1;
        task.set_mutex_held_count(mutex_held);

        /* Has the holder of the mutex inherited the priority of another
           task? */
        let this_task_priority = task.get_priority();
        let this_task_base_priority = task.get_base_priority();
        if this_task_priority != this_task_base_priority {
            /* Only disinherit if no other mutexes are held. */
            if mutex_held == 0 {
                let state_list_item = task.get_state_list_item();

                /* A task can only have an inherited priority if it holds
                   the mutex.  If the mutex is held by a task then it cannot be
                   given from an interrupt, and if a mutex is given by the
                   holding	task then it must be the running state task.  Remove
                   the	holding task from the ready	list. */
                if list_remove!(state_list_item) == 0 {
                    taskRESET_READY_PRIORITY!(this_task_priority);
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }

                /* Disinherit the priority before adding the task into the
                   new	ready list. */
                traceTASK_PRIORITY_DISINHERIT!( &task, this_task_base_priority );
                task.set_priority(this_task_base_priority);

                /* Reset the event list item value.  It cannot be in use for
                   any other purpose if this task is running, and it must be
                   running to give back the mutex. */
                let new_item_val = (configMAX_PRIORITIES!() - this_task_priority) as TickType;
                set_list_item_value!(task.get_event_list_item(), new_item_val);
                task.add_task_to_ready_list().unwrap();

                /* Return true to indicate that a context switch is required.
                   This is only actually required in the corner case whereby
                   multiple mutexes were held and the mutexes were given back
                   in an order different to that in which they were taken.
                   If a context switch did not occur when the first mutex was
                   returned, even if a task was waiting on it, then a context
                   switch should occur when the last mutex is returned whether
                   a task is waiting on it or not. */
                ret_val = true;
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }

    ret_val
}
