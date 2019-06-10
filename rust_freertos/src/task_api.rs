g[marco_use]
extern crate lazy_static ;
use crate::* ;
use crate::port::* ;
use crate::kernel ;
use crate::task_control::* ;
use crate::task_queue::* ;
pub type StackType = usize;
pub type BaseType = i64;
pub type UBaseType = u64;
pub type TickType = u32;

/*
struct TCB{
    state_list_item : ListItem ,
    event_list_item : ListItem ,
    task_priority   : UBaseType ,
    task_stacksize  : UBaseType ,
    task_name       : String ,
    stack_pos       : *mut StackType,
    #[cfg(feature = "portCRITICAL_NESTING_IN_TCB")]
    critical_nesting: UBaseType,

    // reverse priority
    #[cfg(feature = "configUSE_MUTEXES")]
	base_priority  : UBaseType,
    #[cfg(feature = "configUSE_MUTEXES")]
	mutexes_held   : UBaseType,

    #[cfg(feature = "configGENERATE_RUN_TIME_STATUS")]
	runtime_counter: TickType,

    // notify information
    #[cfg(feature = "config_USE_TASK_NOTIFICATIONS")]
	notified_value: u32,
    #[cfg (not(feature = "config_USE_TASK_NOTIFICATIONS"))]
	notify_state  : u8,
}

pub type TaskHandle = TCB ;
pub type TaskStatus = TCB ;  // ???  another struct
pub type TaskState  = UBaseType ;   // ???

lazy_static!{
    static ref CurrentTCB: TCB = something ;// 从何处获取当前TCB
}
*/
macro_rules! get_tcb_from_handle_inAPI {
    ($task:expr) => (
        match task {
        Some(t) => t ,
        None => get_current_task_handle!()
        }
    )
}
*/

pub fn task_priority_get(xTask:Option<TaskHandle>) -> UBaseType
{
    let mut uxReturn:UBaseType = 0 ;
    taskENTER_CRITICAL!() ;
    {
        let pxTCB: = get_tcb_from_handle_inAPI!(xTask) ;
        uxReturn = pxTCB.get_priority() ;
    }
    taskEXIT_CRITICAL!() ;
    return uxReturn ;
}

pub fn task_priority_set(xTask:Option<&TaskHandle> , uxNewPriority:UBaseType)
{
    let mut xYieldRequired:bool = false ;
    let mut uxCurrentBasePriority:UBaseType = 0 ;
    let mut uxPriorityUsedOnEntry:UBaseType = 0 ;

    //valid ensure
    if uxNewPriority >= configMAX_PRIORITIES!() as UBaseType {
        uxNewPriority = configMAX_PRIORITIES!() as UBaseType - 1 as UBaseType ;
    }   else {
        mtCOVERAGE_TEST_MARKER!() ;     
    }
    taskENTER_CRITICAL!() ;
    
    {
        let mut pxTCB = get_tcb_from_handle_inAPI(&xTask) ;
        traceTask_PRIORITY_SET!(&pxTCB , &uxNewPriority) ;  // crate?
        #[cfg(feature = "configUSE_MUTEXES")]
            uxCurrentBasePriority = pxTCB.get_base_priority() ;
        #[cfg(not(feature = "configUSE_MUTEXES") )]
        uxCurrentBasePriority = pxTCB.get_priority() ;
        if uxCurrentBasePriority != uxNewPriority
        {
            // change the Priority ;
            if pxTCB != get_current_task_handle!()
            {
                if uxNewPriority >= get_current_task_priority!()  {
                    xYieldRequired:bool = true ;
                } else {
                    meCOVERAGE_TEST_MARKER!();       // ???
                }
            }
            else
            { ; }         // 当前正在执行的task已是最高优先级
        }
        else if pxTCB == get_current_task_handle!()
        {
            xYieldRequired:bool = true ;
        }
        else
        { ; }
            // 其他task优先级设置不需要yield    ???
        
        {
        #![cfg(feature = "configUSE_MUTEXES" )]
            if pxTCB.get_base_priority() == pxTCB.get_priority() {
                pxTCB.set_priority(uxNewPriority) ;
            }
            else {
                mtCOVERAGE_TEST_MARKER!() ;
            }
            pxTCB.set_base_priority(uxNewPriority) ;
        }
        #[cfg(not(feature = "configUSE_MUTEXS" ))]
            pxTCB.set_priority(uxNewPriority) ;\


        let event_list_item = pxTCB.get_event_list_item() ;
        let state_list_item = pxTCB.get_state_list_item() ;
        
        if (get_list_item_value!(event_list_item) & taskEVENT_LIST_ITEM_VALUE_IN_USE ) == 0 
        {    set_list_item_value( event_list_item, ( ( configMAX_PRIORITIES!() as TickType - uxNewPriority as TickType) ); /*lint !e961 MISRA exception as the casts are only redundant for some ports. */
        }
        else {
	    mtCOVERAGE_TEST_MARKER!();
        }
//???
        if is_contained_within!(nth_ready_list!(uxPriorityUsedOnEntry), state_list_item) != false  
	{
	    if list_remove!( state_list_item ) ==  0 as UBaseType {
    		portRESET_READY_PRIORITY!( uxPriorityUsedOnEntry, uxTopReadyPriority );
            }
            else {
                mtCOVERAGE_TEST_MARKER!();
            }
            pxTCB.add_task_to_ready_list();
        }
        else {
            mtCOVERAGE_TEST_MARKER!();
        }

        if xYieldRequired != false {
            taskYIELD_IF_USING_PREEMPTION!();
        }
        else {
            mtCOVERAGE_TEST_MARKER!();
        }

    }

    taskEXIT_CRITICAL!() ;
}

/*
pub fn task_get_system_state(pxTaskStatusArray:&TaskStatus , uxArraySize:UBaseType , pulTotalRunTime:u32) -> UBaseType
{
    let mut uxTask:UBaseType = 0 ;
    let mut uxQueue = configMAX_PRIORITIES!();
    kernel::task_suspend_all();      // ???
    {
        /* Is there a space in the array for each task in the system? */
        if uxArraySize >= uxCurrentNumberOfTasks 
        {
            // while 实现do while
            uxQueue = uxQueue - 1;
            uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), &( pxReadyTasksLists[ uxQueue ] ), eReady );
            while uxQueue > ( UBaseType ) tskIDLE_PRIORITY  /*lint !e961 MISRA exception as the casts are only redundant for some ports. */
            {
                uxQueue--;
                uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), &( pxReadyTasksLists[ uxQueue ] ), eReady );
            }
            
            uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), ( List_t * ) pxDelayedTaskList, eBlocked );
            uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), ( List_t * ) pxOverflowDelayedTaskList, eBlocked );
            
            #[cfg( feature = "INCLUDE_vTaskDelete" )]
                uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), &xTasksWaitingTermination, eDeleted );

            #[cfg( feature = "INCLUDE_vTaskSuspend" )]
                uxTask += prvListTasksWithinSingleList( &( pxTaskStatusArray[ uxTask ] ), &xSuspendedTaskList, eSuspended );
           
            {
            #![cfg( feature = "configGENERATE_RUN_TIME_STATS" )]
                if pulTotalRunTime != NULL 
                {
                    #[cfg( feature = "portALT_GET_RUN_TIME_COUNTER_VALUE" )]
                    portALT_GET_RUN_TIME_COUNTER_VALUE!( ( *pulTotalRunTime ) );
                    #[cfg(not( feature = "portALT_GET_RUN_TIME_COUNTER_VALUE" ))]
                    &pulTotalRunTime = portGET_RUN_TIME_COUNTER_VALUE!();
                }
            }
            {
            #![cfg(not( feature = "configGENRATE_RUN_TIME_STATS" ))]
                if( pulTotalRunTime != NULL )
                    &pulTotalRunTime = 0;                               // 用&解除引用
            }
        }
        else {
            mtCOVERAGE_TEST_MARKER!();
        }
    }
    kernel::xTaskResumeAll();
    return uxTask;
}
*/

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
            pxTaskStatus.usStackHighWaterMark = prvTaskCheckFreeStackSpace( ( uint8_t & ) pxTCB.pxEndOfStack );
        } else {
            pxTaskStatus.usStackHighWaterMark = prvTaskCheckFreeStackSpace( ( uint8_t & ) pxTCB.pxStack );
        }
    }
    else
    {
        pxTaskStatus.usStackHighWaterMark = 0;
    }
}


pub fn task_get_application_task_tag(xTask:TaskHandle) -> TaskHookFunction
{
    let mut xReturn:TaskHookFunction = 0 ;      // TaskHookFunction
    let mut pxTCB = get_tcb_from_handle_inAPI!(&xTask) ;
    taskENTER_CRITICAL!() ;
    xReturn = pxTCB.get_task_tag ;
    taskEXIT_CRITICAL!() ;
    xReturn ;
}
/*
pub fn task_get_current_task_handle() -> &TaskHandle
{
    let mut xReturn:&TaskHandle = &CurrentTCB ;
    return xReturn ;
}
*/

pub fn task_get_handle(pcNameToQuery:&char) -> TaskHandle
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

pub fn task_get_idle_task_handle() -> TaskHandle
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

//fn task_get_state
