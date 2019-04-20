// config.rs - Macros starting with "config" who evaluate to a numeric value. 
// This file is adapted from FreeRTOSConfig.h
use crate::port::TickType;

// TODO: Provide configASSERT() (or not?).
#[macro_export]
macro_rules! configTICK_RATE_HZ {
    () => {
        1000 as port::TickType
    };
}

#[macro_export]
macro_rules! configMINIMAL_STACK_SIZE {
    () => {
        64 as u8
    };
}

#[macro_export]
macro_rules! configTOTAL_HEAP_SIZE {
    () => {
        64 * 1024 as usize
    };
}

#[macro_export]
macro_rules! configMAX_TASK_NAME_LEN {
    () => {
        16
    };
}

#[macro_export]
macro_rules! configQUEUE_REGISTRY_SIZE {
    () => {
        20
    };
}

#[macro_export]
macro_rules! configMAX_PRIORITIES {
    () => {
        10
    };
}

#[macro_export]
macro_rules! configTIMER_TASK_PRIORITY {
    () => {
        configMAX_PRIORITIES!() - 1
    };
}

#[macro_export]
macro_rules! configTIMER_TASK_STACK_DEPTH {
    () => {
        configMINIMAL_STACK_SIZE * 2
    };
}
