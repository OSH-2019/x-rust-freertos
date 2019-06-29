// Depress some warnings caused by our C bindings.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(fnbox)]
#![feature(weak_ptr_eq)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate simplelog;

mod bindings; // This file is generated by bindgen and doesn't show up in the git repo.
mod port;
mod config;
mod projdefs;
mod trace;
mod ffi;
mod list;
mod task_global;
pub mod task_control;
// mod task_api;
pub mod kernel;
mod queue;
mod queue_h;
mod task_queue;
mod queue_api;
//mod mutex;
mod semaphore;

#[cfg(test)]
mod tests {
    use super::*;

    /*
    // Note! This test SHOULD FAIL, showing something like this:
    // test tests::test_vPortYield ... error: process didn't exit successfully: `/rust_freertos/target/debug/deps/rust_freertos-f3432ee83a2dce9a` (signal: 11, SIGSEGV: invalid memory reference)
    #[test]
    fn test_portYIELD() {
        portYIELD!()
    }
    */

    /*
    // Note! This test SHOULD FAIL as well.
    // BUT on my Mac it just doesn't stop running. Weird.
    use port;
    #[test]
    fn test_port_start_scheduler() {
        port::port_start_scheduler();
    }
    */
    use std::sync::Arc;
    #[test]
    fn test_queue() {
        use std::fs::File;
        use simplelog::*;
        use queue_api::Queue;

        // 两个任务共享所有权，所以需Arc包装。
        let q_recv = Arc::new(Queue::new(10));
        let q_sender = Arc::clone(&q_recv);

        // 发送数据的任务代码。
        let sender = move || {
            let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("sender.log").unwrap());
            for i in 1..11 {
                // send方法的参数包括要发送的数据和ticks_to_wait
                q_sender.send(i, pdMS_TO_TICKS!(50)).unwrap();
            }
        };

        // 接收数据的任务代码。
        let receiver = move || {
            let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("recv.log").unwrap());
            let mut sum = 0;
            loop {
                // receive方法的参数只有ticks_to_wait
                if let Ok(x) = q_recv.receive(pdMS_TO_TICKS!(300)) {
                    println!("{}", x);
                    sum += x;
                } else {
                    // 若等待300ms仍未收到数据，则认为发送结束。
                    assert_eq!(sum, 55);
                    kernel::task_end_scheduler();
                }
            }
        };

        // 创建这两个任务。
        let _sender_task = task_control::TCB::new()
                            .name("Sender")
                            .priority(3)
                            .initialise(sender);

        let _receiver_task = task_control::TCB::new()
                            .name("Receiver")
                            .priority(3)
                            .initialise(receiver);

        kernel::task_start_scheduler();
    }
}
