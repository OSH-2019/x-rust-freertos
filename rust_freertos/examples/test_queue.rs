#[macro_use]
extern crate log;
extern crate rust_freertos;

use rust_freertos::*;
use simplelog::*;
use queue_api::Queue;
use std::sync::Arc;

fn test_queue() {
    // 两个任务共享所有权，所以需Arc包装。
    let q_recv = Arc::new(Queue::new(10));
    let q_sender = Arc::clone(&q_recv);
    let _ = TermLogger::init(LevelFilter::Trace, Config::default());
    // 发送数据的任务代码。
    let sender = move || {
        for i in 1..11 {
            // send方法的参数包括要发送的数据和ticks_to_wait
            q_sender.send(i, pdMS_TO_TICKS!(50)).unwrap();
        }
        loop {
        }
    };
    // 接收数据的任务代码。
    let receiver = move || {
        let mut sum = 0;
        loop {
            // receive方法的参数只有ticks_to_wait
            if let Ok(x) = q_recv.receive(pdMS_TO_TICKS!(10)) {
                println!("{}", x);
                sum += x;
            } else {
                trace!("receive END");
                // 若等待30ms仍未收到数据，则认为发送结束。
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
