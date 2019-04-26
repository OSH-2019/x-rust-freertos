// DO NOT CHANGE THIS FILE!

// ffi.rs - Foreign function interface.
// This file is created by Fan Jinhao. 
// It's meant to be an interface for C functions to call Rust functions.

use crate::kernel;
use crate::port::BaseType;
use crate::projdefs::{pdTRUE, pdFALSE};

pub type xTaskHandle = *mut ::std::os::raw::c_void;

#[no_mangle]
extern "C" fn xTaskGetCurrentTaskHandle() -> xTaskHandle {
    println!("xTaskGetCurrentTaskHandle() called!");
    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn xTaskIncrementTick() -> i64{
    println!("xTaskIncrementTick() called!");
    0
}

#[no_mangle]
extern "C" fn vTaskSwitchContext() {
    println!("vTaskSwitchContext() called!");
    kernel::task_switch_context();
}

#[no_mangle]
extern "C" fn vTaskSuspendAll() {
    println!("vTaskSuspendAll() called!");
    kernel::task_suspend_all();
}

#[no_mangle]
extern "C" fn xTaskResumeAll() -> BaseType {
    println!("xTaskResumeAll() called!");
    if kernel::task_resume_all() {
        pdTRUE
    } else {
        pdFALSE
    }
}
