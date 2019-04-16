// This file is created by Fan Jinhao.
// All the trace macros are defined in this file, along with mtCOVERAGE_*
// These macros may be useful when debugging.
// Macros in this file are adapted from FreeRTOS.h

/* Remove any unused trace macros. */

/* Used to perform any necessary initialisation - for example, open a file
into which trace is to be written. */
#[macro_export]
macro_rules! traceSTART {
    () => {};
}

/* Use to close a trace, for example close a file into which trace has been
written. */
#[macro_export]
macro_rules! traceEND {
    () => {};
}

/* Called after a task has been selected to run.  pxCurrentTCB holds a pointer
to the task control block of the selected task. */
#[macro_export]
macro_rules! traceTASK_SWITCHED_IN {
    () => {};
}

/* Called before stepping the tick count after waking from tickless idle
sleep. */
#[macro_export]
macro_rules! traceINCREASE_TICK_COUNT {
    ($x: expr) => {};
}

/* Called immediately before entering tickless idle. */
#[macro_export]
macro_rules! traceLOW_POWER_IDLE_BEGIN {
    () => {};
}

/* Called when returning to the Idle task after a tickless idle. */
#[macro_export]
macro_rules! traceLOW_POWER_IDLE_END {
    () => {};
}

/* Called before a task has been selected to run.  pxCurrentTCB holds a pointer
to the task control block of the task being switched out. */
#[macro_export]
macro_rules! traceTASK_SWITCHED_OUT {
    () => {};
}

/* Called when a task attempts to take a mutex that is already held by a
lower priority task.  pxTCBOfMutexHolder is a pointer to the TCB of the task
that holds the mutex.  uxInheritedPriority is the priority the mutex holder
will inherit (the priority of the task that is attempting to obtain the
muted. */
#[macro_export]
macro_rules! traceTASK_PRIORITY_INHERIT {
    ($pxTCBOfMutexHolder: expr, $uxInheritedPriority: expr) => {};
}

/* Called when a task releases a mutex, the holding of which had resulted in
the task inheriting the priority of a higher priority task.
pxTCBOfMutexHolder is a pointer to the TCB of the task that is releasing the
mutex.  uxOriginalPriority is the task's configured (base) priority. */
#[macro_export]
macro_rules! traceTASK_PRIORITY_DISINHERIT {
    ($pxTCBOfMutexHolder: expr, $uxOriginalPriority: expr) => {};
}

/* Task is about to block because it cannot read from a
queue/mutex/semaphore.  pxQueue is a pointer to the queue/mutex/semaphore
upon which the read was attempted.  pxCurrentTCB points to the TCB of the
task that attempted the read. */
#[macro_export]
macro_rules! traceBLOCKING_ON_QUEUE_RECEIVE {
    ($pxQueue: expr) => {};
}

/* Task is about to block because it cannot write to a
queue/mutex/semaphore.  pxQueue is a pointer to the queue/mutex/semaphore
upon which the write was attempted.  pxCurrentTCB points to the TCB of the
task that attempted the write. */
#[macro_export]
macro_rules! traceBLOCKING_ON_QUEUE_SEND {
    ($pxQueue: expr) => {};
}

/* The following event macros are embedded in the kernel API calls. */

#[macro_export]
macro_rules! traceMOVED_TASK_TO_READY_STATE {
    ($pxTCB: expr) => {};
}

#[macro_export]
macro_rules! tracePOST_MOVED_TASK_TO_READY_STATE {
    ($pxTCB: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_CREATE {
    ($pxNewQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_CREATE_FAILED {
    ($ucQueueType: expr) => {};
}

#[macro_export]
macro_rules! traceCREATE_MUTEX {
    ($pxNewQueue: expr) => {};
}

#[macro_export]
macro_rules! traceCREATE_MUTEX_FAILED {
    () => {};
}

#[macro_export]
macro_rules! traceGIVE_MUTEX_RECURSIVE {
    ($pxMutex: expr) => {};
}

#[macro_export]
macro_rules! traceGIVE_MUTEX_RECURSIVE_FAILED {
    ($pxMutex: expr) => {};
}

#[macro_export]
macro_rules! traceTAKE_MUTEX_RECURSIVE {
    ($pxMutex: expr) => {};
}

#[macro_export]
macro_rules! traceTAKE_MUTEX_RECURSIVE_FAILED {
    ($pxMutex: expr) => {};
}

#[macro_export]
macro_rules! traceCREATE_COUNTING_SEMAPHORE {
    () => {};
}

#[macro_export]
macro_rules! traceCREATE_COUNTING_SEMAPHORE_FAILED {
    () => {};
}

#[macro_export]
macro_rules! traceQUEUE_SEND {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_SEND_FAILED {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_RECEIVE {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_PEEK {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_PEEK_FROM_ISR {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_RECEIVE_FAILED {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_SEND_FROM_ISR {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_SEND_FROM_ISR_FAILED {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_RECEIVE_FROM_ISR {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_RECEIVE_FROM_ISR_FAILED {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_PEEK_FROM_ISR_FAILED {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_DELETE {
    ($pxQueue: expr) => {};
}

#[macro_export]
macro_rules! traceTASK_CREATE {
    ($pxNewTCB: expr) => {};
}

#[macro_export]
macro_rules! traceTASK_CREATE_FAILED {
    () => {};
}

#[macro_export]
macro_rules! traceTASK_DELETE {
    ($pxTaskToDelete: expr) => {};
}

#[macro_export]
macro_rules! traceTASK_DELAY_UNTIL {
    ($x: expr) => {};
}

#[macro_export]
macro_rules! traceTASK_DELAY {
    () => {};
}

#[macro_export]
macro_rules! traceTASK_PRIORITY_SET {
    ($pxTask: expr, $uxNewPriority: expr) => {};
}

#[macro_export]
macro_rules! traceTASK_SUSPEND {
    ($pxTaskToSuspend: expr) => {};
}

#[macro_export]
macro_rules! traceTASK_RESUME {
    ($pxTaskToResume: expr) => {};
}

#[macro_export]
macro_rules! traceTASK_RESUME_FROM_ISR {
    ($pxTaskToResume: expr) => {};
}

#[macro_export]
macro_rules! traceTASK_INCREMENT_TICK {
    ($xTickCount: expr) => {};
}

#[macro_export]
macro_rules! traceTIMER_CREATE {
    ($pxNewTimer: expr) => {};
}

#[macro_export]
macro_rules! traceTIMER_CREATE_FAILED {
    () => {};
}

#[macro_export]
macro_rules! traceTIMER_COMMAND_SEND {
    ($xTimer: expr, $xMessageID: expr, $xMessageValueValue: expr, $xReturn: expr) => {};
}

#[macro_export]
macro_rules! traceTIMER_EXPIRED {
    ($pxTimer: expr) => {};
}

#[macro_export]
macro_rules! traceTIMER_COMMAND_RECEIVED {
    ($pxTimer: expr, $xMessageID: expr, $xMessageValue: expr) => {};
}

#[macro_export]
macro_rules! traceMALLOC {
    ($pvAddress: expr, $uiSize: expr) => {};
}

#[macro_export]
macro_rules! traceFREE {
    ($pvAddress: expr, $uiSize: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_CREATE {
    ($xEventGroup: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_CREATE_FAILED {
    () => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_SYNC_BLOCK {
    ($xEventGroup: expr, $uxBitsToSet: expr, $uxBitsToWaitFor: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_SYNC_END {
    ($xEventGroup: expr, $uxBitsToSet: expr, $uxBitsToWaitFor: expr, $xTimeoutOccurred: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_WAIT_BITS_BLOCK {
    ($xEventGroup: expr, $uxBitsToWaitFor: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_WAIT_BITS_END {
    ($xEventGroup: expr, $uxBitsToWaitFor: expr, $xTimeoutOccurred: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_CLEAR_BITS {
    ($xEventGroup: expr, $uxBitsToClear: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_CLEAR_BITS_FROM_ISR {
    ($xEventGroup: expr, $uxBitsToClear: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_SET_BITS {
    ($xEventGroup: expr, $uxBitsToSet: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_SET_BITS_FROM_ISR {
    ($xEventGroup: expr, $uxBitsToSet: expr) => {};
}

#[macro_export]
macro_rules! traceEVENT_GROUP_DELETE {
    ($xEventGroup: expr) => {};
}

#[macro_export]
macro_rules! tracePEND_FUNC_CALL {
    ($xFunctionToPend: expr, $pvParameter1: expr, $ulParameter2: expr, $ret: expr) => {};
}

#[macro_export]
macro_rules! tracePEND_FUNC_CALL_FROM_ISR {
    ($xFunctionToPend: expr, $pvParameter1: expr, $ulParameter2: expr, $ret: expr) => {};
}

#[macro_export]
macro_rules! traceQUEUE_REGISTRY_ADD {
    ($xQueue: expr, $pcQueueName: expr) => {};
}

#[macro_export]
macro_rules! traceTASK_NOTIFY_TAKE_BLOCK {
    () => {};
}

#[macro_export]
macro_rules! traceTASK_NOTIFY_TAKE {
    () => {};
}

#[macro_export]
macro_rules! traceTASK_NOTIFY_WAIT_BLOCK {
    () => {};
}

#[macro_export]
macro_rules! traceTASK_NOTIFY_WAIT {
    () => {};
}

#[macro_export]
macro_rules! traceTASK_NOTIFY {
    () => {};
}

#[macro_export]
macro_rules! traceTASK_NOTIFY_FROM_ISR {
    () => {};
}

#[macro_export]
macro_rules! traceTASK_NOTIFY_GIVE_FROM_ISR {
    () => {};
}

#[macro_export]
macro_rules! mtCOVERAGE_TEST_MARKER {
    () => {};
}

#[macro_export]
macro_rules! mtCOVERAGE_TEST_DELAY {
    () => {};
}
