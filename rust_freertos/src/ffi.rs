// DO NOT CHANGE THIS FILE!

// ffi.rs - Foreign function interface.
// This file is created by Fan Jinhao. 
// It's meant to be an interface for C functions to call Rust functions.

use crate::*;
use crate::kernel;
use crate::port::BaseType;
use crate::projdefs::{pdTRUE, pdFALSE};

pub type xTaskHandle = *mut ::std::os::raw::c_void;

#[no_mangle]
extern "C" fn xTaskGetCurrentTaskHandle() -> xTaskHandle {
    trace!("xTaskGetCurrentTaskHandle() called from ffi!");
    get_current_task_handle!().as_raw()
}

#[no_mangle]
extern "C" fn xTaskIncrementTick() -> BaseType{
    trace!("xTaskIncrementTick() called from ffi!");
    if kernel::task_increment_tick() {
        info!("task_increment_tick() returned true");
        pdTRUE
    } else {
        info!("task_increment_tick() returned false");
        pdFALSE
    }
}

#[no_mangle]
extern "C" fn vTaskSwitchContext() {
    trace!("vTaskSwitchContext() called from ffi!");
    kernel::task_switch_context();
}

#[no_mangle]
extern "C" fn vTaskSuspendAll() {
    trace!("vTaskSuspendAll() called from ffi!");
    kernel::task_suspend_all();
}

#[no_mangle]
extern "C" fn xTaskResumeAll() -> BaseType {
    trace!("xTaskResumeAll() called from ffi!");
    if kernel::task_resume_all() {
        info!("task_resume_all() returned true");
        pdTRUE
    } else {
        info!("task_resume_all() returned false");
        pdFALSE
    }
}
