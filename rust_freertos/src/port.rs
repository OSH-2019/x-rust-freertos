// port.c - The wrapper of portable functions written in C.
// This file is created by Fan Jinhao.
use crate::bindings::*;
use crate::projdefs::FreeRtosError;

// NOTE! These type aliases may vary across different platforms.
// TODO: Find a better way to define these types.
pub type StackType = usize;
pub type BaseType = i64;
pub type UBaseType = u64;
pub type TickType = u32;
pub type CVoidPointer = *mut ::std::os::raw::c_void;

#[cfg(target_arch = "x86_64")]
pub const portBYTE_ALIGNMENT_MASK: UBaseType = 8;
#[cfg(not(target_arch = "x86_64"))]
pub const portBYTE_ALIGNMENT_MASK: UBaseType = 4;

#[cfg(feature = "configUSE_16_BIT_TICKS")]
pub const portMAX_DELAY: TickType = 0xffff;
#[cfg(not(feature = "configUSE_16_BIT_TICKS"))]
pub const portMAX_DELAY: TickType = 0xffffffff;

/* -------------------- Macros starting with "port_" ----------------- */
#[macro_export]
macro_rules! portYIELD {
    () => {
        unsafe { crate::bindings::vPortYield() }
    };
}

// TODO: Is it appropriate to place this definition here?
#[macro_export]
macro_rules! portYIELD_WITHIN_API {
    () => {
        portYIELD!()
    };
}

#[macro_export]
macro_rules! portEND_SWITCHING_ISR {
    ($xSwitchRequired: expr) => {
        if $xSwitchRequired {
            unsafe {
                crate::bindings::vPortYieldFromISR();
            }
        }
    };
}

#[macro_export]
macro_rules! portYIELD_FROM_ISR {
    ($xSwitchRequired: expr) => {
        unsafe { portEND_SWITCHING_ISR($xSwitchRequired) }
    };
}

#[macro_export]
macro_rules! portSET_INTERRUPT_MASK_FROM_ISR {
    () => {
        unsafe { (crate::bindings::xPortSetInterruptMask() as BaseType) }
    };
}

#[macro_export]
macro_rules! portCLEAR_INTERRUPT_MASK_FROM_ISR {
    ($xMask: expr) => {
        unsafe { crate::bindings::vPortClearInterruptMask(xMask as BaseType_t) }
    };
}

#[macro_export]
macro_rules! portSET_INTERRUPT_MASK {
    () => {
        unsafe { crate::bindings::vPortDisableInterrupts() }
    };
}

#[macro_export]
macro_rules! portCLEAR_INTERRUPT_MASK {
    () => {
        unsafe { crate::bindings::vPortEnableInterrupts() }
    };
}

#[macro_export]
macro_rules! portDISABLE_INTERRUPTS {
    () => {
        unsafe { portSET_INTERRUPT_MASK!() }
    };
}

#[macro_export]
macro_rules! portENABLE_INTERRUPTS {
    () => {
        unsafe { portCLEAR_INTERRUPT_MASK() }
    };
}

#[macro_export]
macro_rules! portENTER_CRITICAL {
    () => {
        unsafe { crate::bindings::vPortEnterCritical() }
    };
}

#[macro_export]
macro_rules! portEXIT_CRITICAL {
    () => {
        unsafe { crate::bindings::vPortExitCritical() }
    };
}

// TODO: TASK_FUNCTION and TASK_FUNCTION_PROTO may be defined as a macro.
// They were not defined because we haven't decided the prototype of a task function.

#[macro_export]
macro_rules! portNOP {
    () => {
        // This is an empty function.
    };
}

#[macro_export]
macro_rules! traceTASK_DELETE {
    ($pxTaskToDelete: expr) => {
        unsafe {
            // TODO: Add a trace!()
            bindings::vPortForciblyEndThread(std::sync::Arc::into_raw($pxTaskToDelete) as *mut _)
        }
    };
}

#[macro_export]
macro_rules! traceTASK_CREATE {
    ($pxTaskHandle: expr) => {
        unsafe {
            trace!("Task creation accomplished.");
            bindings::vPortAddTaskHandle(std::sync::Arc::into_raw($pxTaskHandle.0) as *mut _)
        }
    };
}

#[macro_export]
macro_rules! portCONFIGURE_TIMER_FOR_RUN_TIME_STATS {
    () => {
        unsafe { crate::bindings::vPortFindTicksPerSecond() }
    };
}

#[macro_export]
macro_rules! portGET_RUN_TIME_COUNTER_VALUE {
    () => {
        unsafe { crate::bindings::ulPortGetTimerValue() }
    };
}

#[macro_export]
macro_rules! portTICK_PERIOS_MS {
    () => {
        1000 as TickType / config::configTICK_RATE_HZ!()
    };
}

// This macro was not implemented by port.c, so it was left blank.
// You can modify it yourself.
#[macro_export]
macro_rules! portCLEAN_UP_TCB {
    ($pxTCB: expr) => {
        $pxTCB
    };
}

// This macro was not implemented by port.c, so it was left blank.
// You can modify it yourself.
#[macro_export]
macro_rules! portPRE_TASK_DELETE_HOOK {
    ($pvTaskToDelete:expr, $pxYieldPending: expr) => {};
}

// This macro was not implemented by port.c, so it was left blank.
// You can modify it yourself.
#[macro_export]
macro_rules! portSETUP_TCB {
    ($pxTCB:expr) => {
        $pxTCB
    };
}

// This macro was not implemented by port.c, so it was left blank.
// You can modify it yourself.
#[macro_export]
macro_rules! portSUPPRESS_TICKS_AND_SLEEP {
    ($xExpectedIdleTime:expr) => {};
}

// This macro was not implemented by port.c, so it was left blank.
// You can modify it yourself.
#[macro_export]
macro_rules! portTASK_USES_FLOATING_POINT {
    () => {};
}

// This macro was not implemented by port.c, so it was left blank.
// You can modify it yourself.
#[macro_export]
macro_rules! portASSERT_IF_INTERRUPT_PRIORITY_INVALID {
    () => {};
}

// This macro was not implemented by port.c, so it was left blank.
// You can modify it yourself.
#[macro_export]
macro_rules! portASSERT_IF_IN_ISR {
    () => {};
}

#[macro_export]
macro_rules! portRESET_READY_PRIORITY {
    ($uxPriority: expr, $uxTopReadyPriority: expr) => {
        // This macro does nothing.
    };
}

/*------------------- Functions starting with "Port_" ----------------- */

// NOTE: I made some changes to the following function names!

/*
 * Map to the memory management routines required for the port.
 */
pub fn port_malloc(size: usize) -> Result<CVoidPointer, FreeRtosError> {
    unsafe { 
        let ret_ptr = pvPortMalloc(size);
        if ret_ptr.is_null() {
            error!("Malloc returned null.");
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(ret_ptr)
        }
    }
}

pub fn port_free(pv: *mut ::std::os::raw::c_void) {
    unsafe { vPortFree(pv) }
}

/* NOTE: vPortInitialiseBlocks() was declared but not implemented.

    pub fn port_initialize_blocks() {
        unsafe {
            vPortInitialiseBlocks()
        }
    }

*/

/* NOTE: xPortGetFreeHeapSize() was declared but not implemented

    pub fn port_get_free_heap_size() -> usize{
        unsafe {
            xPortGetFreeHeapSize()
        }
    }

*/

/* NOTE: xPortGetMinimumEverFreeHeapSize() was declared but not implemented

    pub fn port_get_minimum_ever_free_heap_size() -> usize {
        unsafe {
            xPortGetMinimumEverFreeHeapSize()
        }
    }

*/

/*
 * Setup the hardware ready for the scheduler to take control.  This generally
 * sets up a tick interrupt and sets timers for the correct tick frequency.
 */
pub fn port_start_scheduler() -> BaseType {
    unsafe { xPortStartScheduler() }
}

/*
 * Undo any hardware/ISR setup that was performed by xPortStartScheduler() so
 * the hardware is left in its original condition after the scheduler stops
 * executing.
 */
pub fn port_end_scheduler() {
    unsafe { vPortEndScheduler() }
}

/*
 * Setup the stack of a new task so it is ready to be placed under the
 * scheduler control.  The registers have to be placed on the stack in
 * the order that the port expects to find them.
 *
 */
pub fn port_initialise_stack(
    pxTopOfStack: *mut StackType,
    pxCode: TaskFunction_t,
    pvParameters: *mut ::std::os::raw::c_void,
) -> Result<*mut StackType, FreeRtosError> {
    let ret_val = unsafe { pxPortInitialiseStack(pxTopOfStack, pxCode, pvParameters) };
    if ret_val.is_null() {
        error!("Port failed to initialise task stack!");
        Err(FreeRtosError::PortError)
    } else {
        Ok(ret_val)
    }
}
