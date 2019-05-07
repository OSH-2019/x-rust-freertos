#[marco_use]
extern crate lazy_static ;
use crate::port::* ;

struct TCB{
    state_list_item : ListItem ,
    event_list_item : ListItem ,
    task_priority   : UBaseType ,
    task_stacksize  : UBaseType ,
    task_name       : String ,
    stack_pos       : *mut StackType,
    #[cfg(portCRITICAL_NESTING_IN_TCB)]
    critical_nesting: UBaseType,

    //* reverse priority
    #[cfg(configUSE_MUTEXES)]
	base_priority  : UBaseType,
	#[cfg(configUSE_MUTEXES)]
	mutexes_held   : UBaseType,

    #[cfg(configGENERATE_RUN_TIME_STATUS)]
	runtime_counter: TickType,

    //* notify information
    #[cfg(config_USE_TASK_NOTIFICATIONS)]
	notified_value: u32,
	#[cfg(config_USE_TASK_NOTIFICATIONS)]
	notify_state  : u8,
}

pub type TaskHandle = TCB ;

lazy_static!{
    static ref CurrentTCB: TCB = something ;// 从何处获取当前TCB
}

fn get_tcb_from_handle(task:Option<&TCB>) -> &TCB{
    match task{
        Some(t) => t ,
        None => &CurrentTCB
    }
}



fn task_priority_get(xTask:TaskHandle) -> UBaseType
{
    let mut uxReturn : UBaseType = 0 ;
    taskENTER_CRITICAL() ;
    {
        let pxTCB: &TCB = get_tcb_from_handle(&xTask) ;
        uxReturn = pxTCB.task_priority ;
    }
    taskEXIT_CRITICAL() ;
    uxReturn ;
}

fn task_priority_set(xTask:TaskHandle , uxNewPriority:UBaseType)
{
    let mut xYieldRequired:UBaseType = pdFALSE ;
    let mut uxCurrentBasePriority:UBaseType = 0 ;
    let mut uxPriorityUsedOnEntry:UBaseType = 0 ;

    //valid ensure
    if (uxNewPriority >= (UBaseType) configMAX_PRIORITIES)
        uxNewPriority = (UBaseType) configMAX_PRIORITIES - (UBaseType)1U ;  // ???
    else
        mtCOVERAGE_TEST_MARKER() ;      //???

    taskENTER_CRITICAL() ;
    
    {
        let mut pxTCB:&TCB = get_tcb_from_handle(&xTask) ;
        traceTask_PRIORITY_SET(&pxTCB , &uxNewPriority) ;  // crate?
        uxCurrentBasePriority = (configUSE_MUTEXES == 1) ? pxTCB.base_priority : pxTCB.task_priority ;
        if (uxCurrentBasePriority != uxNewPriority)
        {
            // change the Priority ;
            if (pxTCB != &CurrentTCB)
            {
                if (uxNewPriority >= CurrentTCB.task_priority)
                    xYieldRequired:UBaseType = pdTRUE ;
                else
                    meCOVERAGE_TEST_MARKER();       // ???
            }
            else
            // 当前正在执行的task已是最高优先级
                ；
        }
        else if (pxTCB == CurrentTCB)
            xYieldRequired = pdTRUE ;
        else
            ;
            // 其他task优先级设置不需要yield    ???
        
        {
        #[cfg( configUSE_MUTEXES )]
            if (pxTCB.base_priority == pxTCB.task_priority)
                pxTCB.task_priority = uxNewPriority ;
            else
                mtCOVERAGE_TEST_MARKER() ;
            pxTCB.base_priority = uxNewPriority ;
        }
        #![cfg( configUSE_MUTEXS )]
            pxTCB.task_priority = uxNewPriority ;
        
        // 其中包含了乱七八糟的外部function
        if( ( listGET_LIST_ITEM_VALUE( &( pxTCB->event_list_item ) ) & taskEVENT_LIST_ITEM_VALUE_IN_USE ) == 0UL )
	    listSET_LIST_ITEM_VALUE( &( pxTCB->event_list_item ), ( ( TickType ) configMAX_PRIORITIES - ( TickType ) uxNewPriority ) ); /*lint !e961 MISRA exception as the casts are only redundant for some ports. */
        else
	    mtCOVERAGE_TEST_MARKER();
	
        if( listIS_CONTAINED_WITHIN( &( pxReadyTasksLists[ uxPriorityUsedOnEntry ] ), &( pxTCB->xStateListItem ) ) != pdFALSE )
	{
	    if( uxListRemove( &( pxTCB->xStateListItem ) ) == ( UBaseType ) 0 )
    		portRESET_READY_PRIORITY( uxPriorityUsedOnEntry, uxTopReadyPriority );
            else
                mtCOVERAGE_TEST_MARKER();
            prvAddTaskToReadyList( pxTCB );
        }
        else
            mtCOVERAGE_TEST_MARKER();
        
        if( xYieldRequired != pdFALSE )
            taskYIELD_IF_USING_PREEMPTION();
        else
            mtCOVERAGE_TEST_MARKER();
        ( void ) uxPriorityUsedOnEntry;
    }

    taskEXIT_CRITICAL() ;
}

fn task_get_system_state()
{
    let mut uxTask:UBaseType = 0
    let mut uxQueue = configMAX_PRIORITIES;
    vTaskSuspendAll();      // ???
    {
        /* Is there a space in the array for each task in the system? */
        if( uxArraySize >= uxCurrentNumberOfTasks )
        {
            do
            {
                uxQueue--;
                uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), &( pxReadyTasksLists[ uxQueue ] ), eReady );
            } while( uxQueue > ( UBaseType ) tskIDLE_PRIORITY ); /*lint !e961 MISRA exception as the casts are only redundant for some ports. */
            
            uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), ( List_t * ) pxDelayedTaskList, eBlocked );
            uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), ( List_t * ) pxOverflowDelayedTaskList, eBlocked );
            
            #[cfg( INCLUDE_vTaskDelete )]
                uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), &xTasksWaitingTermination, eDeleted );

            #[cfg( INCLUDE_vTaskSuspend )]
                uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), &xSuspendedTaskList, eSuspended );
            
            #[cfg( configGENERATE_RUN_TIME_STATS )]
            {
                if( pulTotalRunTime != NULL )
                {
                    #ifdef portALT_GET_RUN_TIME_COUNTER_VALUE
                    portALT_GET_RUN_TIME_COUNTER_VALUE( ( *pulTotalRunTime ) );
                    #else
                    *pulTotalRunTime = portGET_RUN_TIME_COUNTER_VALUE();
                    #endif
                }
            }
            #![cfg( configGENRATE_RUN_TIME_STATS )]
                if( pulTotalRunTime != NULL )
                    *pulTotalRunTime = 0;
        }
        else
            mtCOVERAGE_TEST_MARKER();
    }
    xTaskResumeAll();
    return uxTask;
}


//fn TaskGetInfo()

fn task_get_application_task_tag(xTask:TaskHandle) -> TaskHookFunction
{
    let mut xReturn:TaskHookFunction = 0 ;      // TaskHookFunction
    let mut pxTCB: &TCB = get_tcb_from_handle(&xTask) ;
    taskENTER_CRITICAL() ;
    xReturn = pxTCB.task_tag ;
    taskEXIT_CRITICAL() ;
    xReturn ;
}

fn task_get_current_task_handle() -> TaskHandle
{
    let mut xReturn:TaskHandle = CurrentTCB ;
    xReturn ;
}
