#[marco_use]
extern crate lazy_static ;
use crate::port::* ;

pub type StackType = usize;
pub type BaseType = i64;
pub type UBaseType = u64;
pub type TickType = u32;

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
//缺少 task_name task_handle task_tag task_state tcb_number???

pub type TaskHandle = TCB ;
pub type TaskStatus = TCB ;  // ???  another struct
pub type TaskState  = UBaseType ;   // ???

lazy_static!{
    static ref CurrentTCB: TCB = something ;// 从何处获取当前TCB
}

fn get_tcb_from_handle(task:Option<&TCB>) -> &TCB{
    match task{
        Some(t) => t ,
        None => &CurrentTCB
    }
}



fn task_priority_get(xTask:Option<&TaskHandle>) -> UBaseType
{
    let mut uxReturn:UBaseType = 0 ;
    taskENTER_CRITICAL() ;
    {
        let pxTCB: &TCB = get_tcb_from_handle(xTask) ;
        uxReturn = pxTCB.task_priority ;
    }
    taskEXIT_CRITICAL() ;
    return uxReturn ;
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
                ;
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

fn task_get_system_state(pxTaskStatusArray:&TaskStatus , uxArraySize:UBaseType , pulTotalRunTime:u32) -> uBaseType
{
    let mut uxTask:UBaseType = 0 ;
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

// 乱七八糟的functions和参数
fn task_test_info(xTask:Option<&TaskHandle>, pxTaskStatus:&TaskStatus, xGetFreeStackSpace:BaseType, eState:TaskState)
{

    /* xTask is NULL then get the state of the calling task. */
    let pxTCB : &TCB = get_tcb_from_handle( xTask );

    pxTaskStatus.xHandle = pxTCB;
    pxTaskStatus.task_name = &( pxTCB.task_name [ 0 ] );
    pxTaskStatus.uxCurrentPriority = pxTCB.task_priority;
    pxTaskStatus.pxStackBase = pxTCB.stack_pose;
    pxTaskStatus.xTaskNumber = pxTCB.tcb_Number;

    #[cfg( configUSE_MUTEXES )]
        pxTaskStatus.base_priority = pxTCB.base_priority;
    #![cfg(configUSE_MUTEXES )]
        pxTaskStatus.base_priority = 0;

    #[cfg ( configGENERATE_RUN_TIME_STATS )]
        pxTaskStatus.ulRunTimeCounter = pxTCB.ulRunTimeCounter;
    #![cfg ( configGENERATE_RUN_TIME_STATS )]
        pxTaskStatus.ulRunTimeCounter = 0;

    /* Obtaining the task state is a little fiddly, so is only done if the
    value of eState passed into this function is eInvalid - otherwise the
    state is just set to whatever is passed in. */
    if( eState != eInvalid )
    {
            if( pxTCB == &CurrentTCB )
                    pxTaskStatus.eCurrentState = eRunning;
            else
            {
                    pxTaskStatus.eCurrentState = eState;

                    #[cfg( INCLUDE_vTaskSuspend )]
                    {
                            /* If the task is in the suspended list then there is a
                            chance it is actually just blocked indefinitely - so really
                            it should be reported as being in the Blocked state. */
                            if( eState == eSuspended )
                            {
                                    vTaskSuspendAll();
                                    {
                                            if( listLIST_ITEM_CONTAINER( &( pxTCB->xEventListItem ) ) != NULL )
                                            {
                                                    pxTaskStatus->eCurrentState = eBlocked;
                                            }
                                    }
                                    ( void ) xTaskResumeAll();
                            }
                    }
            }
    }
    else
    {
            pxTaskStatus->eCurrentState = eTaskGetState( pxTCB );
    }

    /* Obtaining the stack space takes some time, so the xGetFreeStackSpace
    parameter is provided to allow it to be skipped. */
    if( xGetFreeStackSpace != pdFALSE )
    {
            #[cfg( portSTACK_GROWTH > 0 )]
            {
                    pxTaskStatus->usStackHighWaterMark = prvTaskCheckFreeStackSpace( ( uint8_t * ) pxTCB->pxEndOfStack );
            }
            #![cfg( portSTACK_GROWTH > 0 )]
            {
                    pxTaskStatus->usStackHighWaterMark = prvTaskCheckFreeStackSpace( ( uint8_t * ) pxTCB->pxStack );
            }
    }
    else
    {
            pxTaskStatus->usStackHighWaterMark = 0;
    }
}


fn task_get_application_task_tag(xTask:TaskHandle) -> TaskHookFunction
{
    let mut xReturn:TaskHookFunction = 0 ;      // TaskHookFunction
    let mut pxTCB: &TCB = get_tcb_from_handle(&xTask) ;
    taskENTER_CRITICAL() ;
    xReturn = pxTCB.task_tag ;
    taskEXIT_CRITICAL() ;
    xReturn ;
}

fn task_get_current_task_handle() -> &TaskHandle
{
    let mut xReturn:&TaskHandle = &CurrentTCB ;
    return xReturn ;
}

// ???
fn task_get_handle(pcNameToQuery:&char) -> TaskHandle
{
    let mut uxQueue:UBaseType = configMAX_PRIORITIES;
    let mut pxTCB:&TCB = 0 ;

    /* Task names will be truncated to configMAX_TASK_NAME_LEN - 1 bytes. */
    configASSERT( strlen( pcNameToQuery ) < configMAX_TASK_NAME_LEN );

    vTaskSuspendAll();
    {
            /* Search the ready lists. */
            do
            {
                    uxQueue--;
                    pxTCB = prvSearchForNameWithinSingleList( ( List_t * ) &( pxReadyTasksLists[ uxQueue ] ), pcNameToQuery );

                    if( pxTCB != NULL )
                    {
                            /* Found the handle. */
                            break;
                    }

            } while( uxQueue > ( UBaseType_t ) tskIDLE_PRIORITY ); /*lint !e961 MISRA exception as the casts are only redundant for some ports. */

            /* Search the delayed lists. */
            if( pxTCB == NULL )
                    pxTCB = prvSearchForNameWithinSingleList( ( List_t * ) pxDelayedTaskList, pcNameToQuery );

            if( pxTCB == NULL )
                    pxTCB = prvSearchForNameWithinSingleList( ( List_t * ) pxOverflowDelayedTaskList, pcNameToQuery );

            #[cfg ( INCLUDE_vTaskSuspend == 1 )]
                    if( pxTCB == NULL )
			pxTCB = prvSearchForNameWithinSingleList( &xSuspendedTaskList, pcNameToQuery );         

            #[cfg( INCLUDE_vTaskDelete == 1 )]
                    if( pxTCB == NULL )
                            /* Search the deleted list. */
                            pxTCB = prvSearchForNameWithinSingleList( &xTasksWaitingTermination, pcNameToQuery );
    }
    xTaskResumeAll();

    return pxTCB;
}

fn task_get_idle_task_handle() -> TaskHandle
{
    /* If xTaskGetIdleTaskHandle() is called before the scheduler has been
    started, then xIdleTaskHandle will be NULL. */
    configASSERT( ( IdleTaskHandle != NULL ) );         // 这玩意儿是个全局变量???
    return IdleTaskHandle;
}

fn task_get_stack_high_water_mark(xtask:Option<&TaskHandle>) -> UBaseType
{
    let mut pucEndOfStack = 0;
    let mut uxReturn:UBaseType = 0;

    let pxTCB:&TCB = get_tcb_from_handle(xtask);

    {
    #[cfg( portSTACK_GROWTH < 0 )]
            pucEndOfStack = pxTCB.pxStack;
    }
    {
    #![cfg( portSTACK_GROWTH < 0 )]
            pucEndOfStack = pxTCB.pxEndOfStack;
    }

    uxReturn = ( UBaseType )prvTaskCheckFreeStackSpace( pucEndOfStack );

    return uxReturn;
}

//fn task_get_state
