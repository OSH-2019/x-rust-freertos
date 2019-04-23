//* task API
//! We need list.h

// * Chapter 8 : Basic task function
//TODO: Start a task
// * Input : void
// * Output : void
// * Info : page 108
fn vTaskStartScheuler() {
}

//TODO: Initialize Hardware
// * Input : void
// * Output : BaseType
// * Info : page 109
fn xPortStartScheduler() -> BaseType{
    unimplemented!();
}

//TODO: Create a task
// * Input : - pxTaskCode : TaskFunction_t
// *         - pcName : const char * const
// *         - usStackDepth : const u16
// *         - pvParameters : void * const
// * STUB : void * ??
// *         - uxPriviority : UBaseType
// *         - pxCreatedTask : TaskHandle * const
// * Output : BaseType
// * Info : page 109
fn xTaskCreate() {
    unimplemented!();
}

//TODO: Initialize a task
// * Input : - pxTaskCode : TaskFunction_t
// *         - pcName : const char * const
// *         - ulStackDepth : const u16
// *         - pvParameters : void * const
// * STUB : void * ??
// *         - uxPriviority : UBaseType
// *         - pxCreatedTask : TaskHandle * const
// *         - PXnEWtcb : TCB *
// *         - xRegions : const MemoryRegion
// * STUB : What's this???
// * Output : void
// * Info : page 119
fn prvInitailizeNewTask() {
    unimplemented!();
}

//TODO: Initailize the stack
// * Input : - pxTopOfStack : StackType *
// *         - pxCode : TaskFunction
// *         - pvParameters : void *
// * STUB : void * ??
// * Output : StackType
// * Info : page 122
fn pxPortInitailizeStack () {
    unimplemented!();
}

//TODO: Push the task into the stack
// * Input : - pxNewTCB : TCB_t
// * Output : void
// * Info : page 123
fn prvAddNewTaskToReadyList() {
    unimplemented!();
}

//TODO: Delete the Task
// * Input : - xTaskToEDelete : TaskHandle
// * Output : void
// * Info : page 125
fn vTaskDelete () {
    unimplemented!();
}

//TODO: Suspend the Task
// * Input : - xTaskToSuspend : TaskHandle
// * Output : void
// * Info : page 127
fn vTaskSuspend () {
    unimplemented!();
}

//TODO: Resume the Task
// * Input : - TaskToResume : TaskHandle
// * Output : void
// * Info : page 128
fn vTaskResume() {
    unimplemented!();
}

// * Chap 9 : Switch between tasks
//TODO: Attain next Task
// * Input : void
// * Output : void
// * Info : page 136
fn vTaskSwitchContext() {
    unimplemented!();
}

//TODO: Select the Highest Priority Task
// * MACRO
// * Info : page 136
fn taskSELECT_HIGHEST_PRIORITY_TASK() {
    unimplemented!();
}

//TODO: Judge whether to switch
// * Input : void
// * Output : BaseType
// * Info : page 139
fn xTaskIncrementTick() {
    unimplemented!();
}

// * Chap 10 : Control the Kernel
//TODO: Switch tasks
// * Input : void
// * Output : void
// * Info : page 146
fn taskYIELD() {
    unimplemented!();
}

//TODO: Go to CRITICAL
// * Input : void
// * Output : void
// * Info : page 146
fn taskENTER_CRITCAL() {
    unimplemented!();
}

//TODO: Exit CRITICAL
// * Input : void
// * Output : void
// * Info : page 146
fn taskEXIT_CRITICAL() {
    unimplemented!();
}

//TODO: Go to CRITICAL from ISR
// * Input : void
// * Output : void
// * Info : page 146
fn taskENTER_CRITCAL_FROM_ISR() {
    unimplemented!();
}

//TODO: Exit CRITICAL from ISR
// * Input : void
// * Output : void
// * Info : page 146
fn taskEXIT_CITICAL_FROM_ISR () {
    unimplemented!();
}

//TODO: Enable Interupt
// * Input : void
// * Output : void
// * Info : page 146
fn taskDISABLE_INTERUPTS ()  {
    unimplemented!();
}

//TODO: Disable Interput
// * Input : void
// * Output : void
// * Info : page 146
fn taskENABLE_INTERUPTS () {
    unimplemented!();
}

//TODO: Enable Scheduler
// * Input : void
// * Output : void
// * Info : page 146
fn vTaskStartScheduler() {
    unimplemented!();
}

//TODO: Disable Scheduler
// * Input : void
// * Output : void
// * Info : page 146
fn vTaskEndScheduler() {
    unimplemented!();
}

//TODO: Suspend all
// * Input : void
// * Output : void
// * Info : page 147
fn vTaskSuspendAll() {
    unimplemented!();
}

//TODO: Resume all
// * Input : void
// * Output : void
// * Info : page 147
fn xTaskResumeAll() {
    unimplemented!();
}

//TODO: Set step tick
// * Input : - xTicksToJump : TickType
// * Output : void
// * Info : page 148
fn vTaskStepTick (){
    unimplemented!();
}

// * Chap 11 : Task API