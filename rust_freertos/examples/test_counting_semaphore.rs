#[macro_use]
extern crate log;
extern crate rust_freertos;

use rust_freertos::*;
use std::sync::Arc;
use simplelog::*;
use semaphore::Semaphore;

fn main() {
    let _ = TermLogger::init(LevelFilter::Trace, Config::default());
    let cs0 = Arc::new(Semaphore::create_counting(2));
    let cs1 = Arc::clone(&cs0);
    let cs2 = Arc::clone(&cs0);
    let task_want_resources0 = move || {
        loop {
            trace!("Enter Task0!");
            match cs0.semaphore_down(pdMS_TO_TICKS!(10)) {
                Ok(_) => {
                    for i in 1..11 {
                        trace!("cs0 owns the counting semaphore! -- {}", i);
                    }
                    // loop {
                    /*you can comment out this loop so that Task1 can successfully down the
                      counting_semaphore*/
                    // }
                    match cs0.semaphore_up() {
                        Ok(_) => {
                            trace!("Task0 Finished!");
                            break;
                        }
                        Err(error) => {
                            trace!("cs0 semaphore give triggers {}", error);
                        }
                    }
                },
                Err(error) => {
                    trace!("cs0 semaphore take triggers {}", error);
                },
            }
        }
        loop {
        }
    };
    let task_want_resources1 = move || {
        loop {
            trace!("Enter Task1!");
            match cs1.semaphore_down(pdMS_TO_TICKS!(10)) {
                Ok(_) => {
                    for i in 1..11 {
                        trace!("cs1 owns the counting semaphore! -- {}", i);
                    }
                    match cs1.semaphore_up() {
                        Ok(_) => {
                            trace!("Test COUNTING SEMAPHORE COMPLETE!");
                            kernel::task_end_scheduler();
                            break;
                        }
                        Err(error) => {
                            trace!("cs1 semaphore give triggers {}", error);
                            kernel::task_end_scheduler();
                        }
                    }
                },
                Err(error) => {
                    trace!("cs1 semaphore take triggers {}", error);
                    kernel::task_end_scheduler();
                },
            }
        }
        loop {
        }
    };
    let task_want_resources2 = move || {
        loop {
            trace!("Enter Task2!");
            match cs2.semaphore_down(pdMS_TO_TICKS!(50)) {
                Ok(_) => {
                    trace!("Task2 OK!");
                    for i in 1..11 {
                        trace!("cs2 owns the counting semaphore! -- {}", i);
                    }
                    loop {
                        /*you can comment out this loop so that Task1 can successfully down the
                          counting_semaphore*/
                    }
                    match cs2.semaphore_up() {
                        Ok(_) => {
                            trace!("Task2 Finished!");
                            break;
                        }
                        Err(error) => {
                            trace!("cs2 semaphore give triggers {}", error);
                        }
                    }
                },
                Err(error) => {
                    trace!("cs2 semaphore take triggers {}", error);
                },
            }
        }
        loop {
        }
    };
    let _task0 = task_control::TCB::new()
        .name("Task0")
        .priority(3)
        .initialise(task_want_resources0);
    let _task1 = task_control::TCB::new()
        .name("Task1")
        .priority(3)
        .initialise(task_want_resources1);
    let _task2 = task_control::TCB::new()
        .name("Task2")
        .priority(3)
        .initialise(task_want_resources2);
    kernel::task_start_scheduler();
}
