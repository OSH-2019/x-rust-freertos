// This file is created by Fan Jinhao.

// NOTE! These type aliases may vary across different platforms.
// TODO: Find a better way to define these types.
pub type StackType = usize;
pub type BaseType = i64;
pub type UBaseType = u64;
pub type TickType = u32;

// TODO: A better way to define "TaskFunction".
// This type alias is taken from FreeRTOS.rs.
// see https://github.com/hashmismatch/freertos.rs/blob/master/src/task.rs
pub type TaskFunction = &'static FnOnce() -> ();

#[cfg(configUSE_16_BIT_TICKS)]
pub const portMAX_DELAY: TickType = 0xffff;
#[cfg(not(configUSE_16_BIT_TICKS))]
pub const portMAX_DELAY: TickType = 0xffffffff;

#[macro_export]
macro_rules! portYIELD {
    () => {
        vPortYield()
    };
}

// TODO: Is it appropriate to place this definition here?
#[macro_export]
macro_rules! portYIELD_WITHIN_API {
    () => {
        portYIELD()
    };
}

#[macro_export]
macro_rules! portEND_SWITCHING_ISR {
    ($xSwitchRequired: expr) => {
        if $xSwitchRequired {
            vPortYieldFromISR();
        }
    };
}

#[macro_export]
macro_rules! portYIELD_FROM_ISR {
    ($xSwitchRequired: expr) => {
        portEND_SWITCHING_ISR($xSwitchRequired)
    };
}

#[macro_export]
macro_rules! portSET_INTERRUPT_MASK_FROM_ISR {
    () => {
        (xPortSetInterruptMask() as BaseType)
    };
}

#[macro_export]
macro_rules! portCLEAR_INTERRUPT_MASK_FROM_ISR {
    ($xMask: expr) => {
        vPortClearInterruptMask(xMask as BaseType_t)
    };
}

#[macro_export]
macro_rules! portSET_INTERRUPT_MASK {
    () => {
        vPortDisableInterrupts()
    };
}

#[macro_export]
macro_rules! portCLEAR_INTERRUPT_MASK {
    () => {
        vPortEnableInterrupts()
    };
}

#[macro_export]
macro_rules! portDISABLE_INTERRUPTS {
    () => {
        portSET_INTERRUPT_MASK()
    };
}

#[macro_export]
macro_rules! portENABLE_INTERRUPTS {
    () => {
        portCLEAR_INTERRUPT_MASK()
    };
}

#[macro_export]
macro_rules! portENTER_CRITICAL {
    () => {
        vPortEnterCritical()
    };
}

#[macro_export]
macro_rules! portEXIT_CRITICAL {
    () => {
        vPortExitCritical()
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

// TODO: traceTASK_DELETE() and traceTASK_CREATE()
// These functions were not wrapped because they require a void* pointer.

#[macro_export]
macro_rules! portCONFIGURE_TIMER_FOR_RUN_TIME_STATS {
    () => {
        vPortFindTicksPerSecond()
    };
}

#[macro_export]
macro_rules! portGET_RUN_TIME_COUNTER_VALUE {
    () => {
        ulPortGetTimerValue()
    };
}

#[macro_export]
macro_rules! portTICK_PERIOS_MS {
    () => {
        1000 as TickType / configTICK_RATE_HZ!()
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
