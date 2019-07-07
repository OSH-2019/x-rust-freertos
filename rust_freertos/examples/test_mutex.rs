#[macro_use]
extern crate log;
extern crate rust_freertos;

use rust_freertos::semaphore::Semaphore;
use rust_freertos::*;
use simplelog::*;
use std::sync::Arc;

fn main() {
    let _ = TermLogger::init(LevelFilter::Trace, Config::default());
    let mutex0 = Arc::new(Semaphore::new_mutex());
    let mutex1 = Arc::clone(&mutex0);
    let task0 = move || {
        task_timemanager::task_delay(pdMS_TO_TICKS!(1));
        loop {
            match mutex0.semaphore_down(pdMS_TO_TICKS!(0)) {
                Ok(_) => {
                    for i in 1..11 {
                        trace!("Task0 owns the mutex! -- {}", i);
                    }
                    /*loop {
                    /*you can comment out this loop so that Task1 can successfully down the
                    counting_semaphore*/
                }*/
                match mutex0.semaphore_up() {
                    Ok(_) => {
                        trace!("Task0 dropped the mutex!");
                        kernel::task_end_scheduler();
                    }
                    Err(error) => {
                        trace!("mutex0 semaphore up triggers {}", error);
                    }
                }
                }
                Err(error) => {
                    trace!("mutex0 semaphore take triggers {}", error);
                    task_timemanager::task_delay(pdMS_TO_TICKS!(1));
                    trace!("mutex0 delay in Err over!");
                }
            }
        }
    };
    let task1 = move || {
        loop {
            match mutex1.semaphore_down(pdMS_TO_TICKS!(0)) {
                Ok(_) => {
                    for i in 1..11 {
                        trace!("Task1 owns the mutex! -- {}", i);
                    }
                    task_timemanager::task_delay(pdMS_TO_TICKS!(1));
                    trace!("Task1's priority is {}", get_current_task_priority!());
                    /*loop {
                      }*/
                    match mutex1.semaphore_up() {
                        Ok(_) => {
                            trace!("Task1 dropped the mutex!");
                            task_timemanager::task_delay(pdMS_TO_TICKS!(1));
                            //     kernel::task_end_scheduler();
                        }
                        Err(error) => {
                            trace!("mutex1 semaphore up triggers {}", error);
                        }
                    }
                }
                Err(error) => {
                    trace!("mutex1 semaphore give triggers {}", error);
                }
            }
        }
    };
    let Task0 = task_control::TCB::new()
        .name("Task0")
        .priority(4)
        .initialise(task0);
    let Task1 = task_control::TCB::new()
        .name("Task1")
        .priority(3)
        .initialise(task1);
    let Task12 = task_control::TCB::new()
        .name("Task2")
        .priority(3)
        .initialise(|| loop{});
    kernel::task_start_scheduler();
}
