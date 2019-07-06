use crate::kernel;
use crate::list;
use crate::port;
use crate::port::{BaseType, TickType, UBaseType};
use crate::task_control;
use crate::task_control::TaskHandle;
use crate::task_global::*;
use crate::task_queue;
use crate::task_queue::taskEVENT_LIST_ITEM_VALUE_IN_USE;
use crate::trace::*;
use crate::*;

macro_rules! get_tcb_from_handle_inAPI {
    ($task:expr) => {
        match $task {
            Some(t) => t,
            None => get_current_task_handle!(),
        }
    };
}

///  INCLUDE_uxTaskPriorityGet must be defined as 1 for this function to be available.
///  See the configuration section for more information.
/// 
///  Obtain the priority of any task.
/// 
/// 
/// * Implemented by: Fan Jinhao
/// 
/// # Arguments:
///  @param xTask Handle of the task to be queried.  Passing a NULL
///  handle results in the priority of the calling task being returned.
/// 
/// 
/// * Return:
///  @return The priority of xTask.
/// 
pub fn task_priority_get(xTask: Option<TaskHandle>) -> UBaseType {
    let mut uxReturn: UBaseType = 0;
    taskENTER_CRITICAL!();
    {
        let pxTCB = get_tcb_from_handle_inAPI!(xTask);
        uxReturn = pxTCB.get_priority();
    }
    taskEXIT_CRITICAL!();
    return uxReturn;
}

///  INCLUDE_vTaskPrioritySet must be defined as 1 for this function to be available.
///  See the configuration section for more information.
/// 
///  Set the priority of any task.
/// 
///  A context switch will occur before the function returns if the priority
///  being set is higher than the currently executing task.
/// 
/// 
/// * Implemented by: Fan Jinhao
/// 
/// # Arguments:
///  @param xTask Handle to the task for which the priority is being set.
///  Passing a NULL handle results in the priority of the calling task being set.
/// 
///  @param uxNewPriority The priority to which the task will be set.
/// 
/// 
/// * Return:
/// 
pub fn task_priority_set(xTask: Option<TaskHandle>, uxNewPriority: UBaseType) {
    let mut uxNewPriority = uxNewPriority;
    let mut xYieldRequired: bool = false;
    let mut uxCurrentBasePriority: UBaseType = 0;
    let mut uxPriorityUsedOnEntry: UBaseType = 0;

    //valid ensure
    if uxNewPriority >= configMAX_PRIORITIES!() as UBaseType {
        uxNewPriority = configMAX_PRIORITIES!() as UBaseType - 1 as UBaseType;
    } else {
        mtCOVERAGE_TEST_MARKER!();
    }
    taskENTER_CRITICAL!();

    {
        let mut pxTCB = get_tcb_from_handle_inAPI!(xTask);
        traceTASK_PRIORITY_SET!(&pxTCB, &uxNewPriority); // crate?

        {
            #![cfg(feature = "configUSE_MUTEXES")]
            uxCurrentBasePriority = pxTCB.get_base_priority();
        }

        {
            #![cfg(not(feature = "configUSE_MUTEXES"))]
            uxCurrentBasePriority = pxTCB.get_priority();
        }

        if uxCurrentBasePriority != uxNewPriority {
            // change the Priority ;
            if pxTCB != get_current_task_handle!() {
                if uxNewPriority >= get_current_task_priority!() {
                    xYieldRequired = true;
                } else {
                    mtCOVERAGE_TEST_MARKER!();
                }
            } else {;            } // 当前正在执行的task已是最高优先级
        } else if pxTCB == get_current_task_handle!() {
            xYieldRequired = true;
        } else {;        }
        // 其他task优先级设置不需要yield    ???

        {
            #![cfg(feature = "configUSE_MUTEXES")]
            if pxTCB.get_base_priority() == pxTCB.get_priority() {
                pxTCB.set_priority(uxNewPriority);
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
            pxTCB.set_base_priority(uxNewPriority);
        }
        #[cfg(not(feature = "configUSE_MUTEXS"))]
        pxTCB.set_priority(uxNewPriority);

        let event_list_item = pxTCB.get_event_list_item();
        let state_list_item = pxTCB.get_state_list_item();

        if (list::get_list_item_value(&event_list_item) & taskEVENT_LIST_ITEM_VALUE_IN_USE) == 0 {
            list::set_list_item_value(
                &event_list_item,
                (configMAX_PRIORITIES!() as TickType - uxNewPriority as TickType),
            );
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }

        if list::is_contained_within(
            &READY_TASK_LISTS[uxPriorityUsedOnEntry as usize],
            &state_list_item,
        ) {
            if list::list_remove(state_list_item) == 0 as UBaseType {
                portRESET_READY_PRIORITY!(uxPriorityUsedOnEntry, uxTopReadyPriority);
            } else {
                mtCOVERAGE_TEST_MARKER!();
            }
            pxTCB.add_task_to_ready_list();
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }

        if xYieldRequired != false {
            taskYIELD_IF_USING_PREEMPTION!();
        } else {
            mtCOVERAGE_TEST_MARKER!();
        }
    }

    taskEXIT_CRITICAL!();
}

/*
   pub fn task_get_system_state(pxTaskStatusArray:&TaskStatus , uxArraySize:UBaseType , pulTotalRunTime:u32) -> UBaseType
   {
   let mut uxtask:UBaseType = 0 ;
   let mut uxqueue = configmax_priorities!();
   kernel::task_suspend_all();      // ???
   {
/* is there a space in the array for each task in the system? */
if uxarraysize >= uxcurrentnumberoftasks
{
    // while 实现do while
    uxqueue = uxqueue - 1;
    uxtask += prvlisttaskswithinsinglelist( &( pxtaskstatusarray[ uxtask ] ), &( pxreadytaskslists[ uxqueue ] ), eready );
    while uxqueue > ( ubasetype ) tskidle_priority  /*lint !e961 misra exception as the casts are only redundant for some ports. */
    {
        uxqueue--;
        uxtask += prvlisttaskswithinsinglelist( &( pxtaskstatusarray[ uxtask ] ), &( pxreadytaskslists[ uxqueue ] ), eready );
    }

    uxtask += prvlisttaskswithinsinglelist( &( pxtaskstatusarray[ uxtask ] ), pxdelayedtasklist as &list , eblocked );
    uxtask += prvlisttaskswithinsinglelist( &( pxtaskstatusarray[ uxtask ] ), pxoverflowdelayedtasklist as &list , eblocked );

    #[cfg( feature = "include_vtaskdelete" )]
    uxtask += prvlisttaskswithinsinglelist( &( pxtaskstatusarray[ uxtask ] ), &xtaskswaitingtermination, edeleted );

    #[cfg( feature = "include_vtasksuspend" )]
    uxtask += prvlisttaskswithinsinglelist( &( pxtaskstatusarray[ uxtask ] ), &xsuspendedtasklist, esuspended );

    {
        #![cfg( feature = "configgenerate_run_time_stats" )]
        if pultotalruntime != null
        {
            #[cfg( feature = "portalt_get_run_time_counter_value" )]
            portalt_get_run_time_counter_value!( ( &pultotalruntime ) );
            #[cfg(not( feature = "portalt_get_run_time_counter_value" ))]
            &pultotalruntime = portget_run_time_counter_value!();
        }
    }
    {
        #![cfg(not( feature = "configgenrate_run_time_stats" ))]
        if( pultotalruntime != null )
            &pultotalruntime = 0;                               // 用&解除引用
    }
}
else {
    mtcoverage_test_marker!();
}
}
kernel::xtaskresumeall();
return uxtask;
}

pub fn task_test_info(xTask:Option<&TaskHandle>, pxTaskStatus:&TaskStatus, xGetFreeStackSpace:BaseType, eState:TaskState)
{

    /* xTask is NULL then get the state of the calling task. */
    let pxTCB = get_tcb_from_handle!( xTask );

    pxTaskStatus.xHandle = pxTCB;
    pxTaskStatus.task_name = &( pxTCB.task_name [ 0 ] );
    pxTaskStatus.uxCurrentPriority = pxTCB.task_priority;
    pxTaskStatus.pxStackBase = pxTCB.stack_pose;
    pxTaskStatus.xTaskNumber = pxTCB.tcb_Number;

    #[cfg( feature = "configUSE_MUTEXES" )]
    pxTaskStatus.base_priority = pxTCB.base_priority;
    #[cfg(not( feature = "configUSE_MUTEXES" ))]
    pxTaskStatus.base_priority = 0;

    #[cfg ( feature = "configGENERATE_RUN_TIME_STATS" )]
    pxTaskStatus.ulRunTimeCounter = pxTCB.ulRunTimeCounter;
    #[cfg(not( feature = "configGENERATE_RUN_TIME_STATS" ))]
    pxTaskStatus.ulRunTimeCounter = 0;

    /* Obtaining the task state is a little fiddly, so is only done if the
       value of eState passed into this function is eInvalid - otherwise the
       state is just set to whatever is passed in. */
    if eState != eInvalid
    {
        if pxTCB == &CurrentTCB {
            pxTaskStatus.eCurrentState = eRunning;
        } else
        {
            pxTaskStatus.eCurrentState = eState;

            {
                #![cfg(feature = "INCLUDE_vTaskSuspend" )]
                /* If the task is in the suspended list then there is a
                   chance it is actually just blocked indefinitely - so really
                   it should be reported as being in the Blocked state. */
                if eState == eSuspended
                {
                    vTaskSuspendAll();

                    {
                        if listLIST_ITEM_CONTAINER( &( pxTCB.xEventListItem ) ) != NULL
                        {
                            pxTaskStatus.eCurrentState = eBlocked;
                        }
                    }

                    ( void ) xTaskResumeAll();
                }
            }
        }
    }
    else
    {
        pxTaskStatus.eCurrentState = eTaskGetState( pxTCB );
    }

    /* Obtaining the stack space takes some time, so the xGetFreeStackSpace
       parameter is provided to allow it to be skipped. */
    if xGetFreeStackSpace != pdFALSE
    {
        if portSTACK_GROWTH > 0 {
            pxTaskStatus.usStackHighWaterMark = prvTaskCheckFreeStackSpace( pxTCB.pxEndOfStack as &i8 );
        } else {
            pxTaskStatus.usStackHighWaterMark = prvTaskCheckFreeStackSpace( pxTCB.pxStack as &i8 );
        }
    }
    else
    {
        pxTaskStatus.usStackHighWaterMark = 0;
    }
}


pub fn task_get_application_task_tag(xTask:TaskHandle) -> UBaseType
{
    let mut xReturn:UBaseType = 0 ;      // TaskHookFunction
    let mut pxTCB = get_tcb_from_handle_inAPI!(&xTask) ;
    taskENTER_CRITICAL!() ;
    xReturn = pxTCB.get_task_tag ;
    taskEXIT_CRITICAL!() ;
    xReturn ;
}

pub fn task_get_handle(pcNameToQuery:&char) -> &TaskHandle
{
    let mut uxQueue:UBaseType = configMAX_PRIORITIES;
    let mut pxTCB:&TCB = 0 ;

    vTaskSuspendAll();
    {
        /* Search the ready lists. */
        while uxQueue > ( UBaseType_t ) tskIDLE_PRIORITY  /*lint !e961 MISRA exception as the casts are only redundant for some ports. */
        {
            uxQueue--;
            pxTCB = prvSearchForNameWithinSingleList( ( List_t * ) &( pxReadyTasksLists[ uxQueue ] ), pcNameToQuery );

            if pxTCB != NULL
            {
                /* Found the handle. */
                break;
            }
        }

        /* Search the delayed lists. */
        if pxTCB == NULL {
            pxTCB = prvSearchForNameWithinSingleList( ( List_t * ) pxDelayedTaskList, pcNameToQuery );
        } else {
            pxTCB = prvSearchForNameWithinSingleList( ( List_t * ) pxOverflowDelayedTaskList, pcNameToQuery );
        }

        {
            #![cfg ( INCLUDE_vTaskSuspend == 1 )]
            if pxTCB == NULL
                pxTCB = prvSearchForNameWithinSingleList( &xSuspendedTaskList, pcNameToQuery );
        }

        {
            #![cfg( INCLUDE_vTaskDelete == 1 )]
            if pxTCB == NULL
                /* Search the deleted list. */{
                    pxTCB = prvSearchForNameWithinSingleList( &xTasksWaitingTermination, pcNameToQuery );
                }
        }
    }
    xTaskResumeAll();

    return pxTCB;
}

pub fn task_get_idle_task_handle() -> &TaskHandle
{
    /* If xTaskGetIdleTaskHandle() is called before the scheduler has been
       started, then xIdleTaskHandle will be NULL. */
    return IdleTaskHandle;
}

pub fn task_get_stack_high_water_mark(xtask:Option<&TaskHandle>) -> UBaseType
{
    let mut pucEndOfStack = 0;
    let mut uxReturn:UBaseType = 0;

    let pxTCB:&TCB = get_tcb_from_handle(xtask);

    if portSTACK_GROWTH < 0 {
        pucEndOfStack = pxTCB.pxStack;
    } else {
        pucEndOfStack = pxTCB.pxEndOfStack;
    }

    uxReturn = ( UBaseType )prvTaskCheckFreeStackSpace( pucEndOfStack );

    return uxReturn;
}
*/
