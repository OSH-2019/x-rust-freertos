extern crate rust_freertos;

fn main() {
    use rust_freertos::*;
    use rust_freertos::semaphore::Semaphore;
    use simplelog::*;

    let _ = TermLogger::init(LevelFilter::Trace, Config::default());
    let recursive_mutex = Semaphore::create_recursive_mutex();

    let mutex_holder = move || {
        for i in 1..11 {
            recursive_mutex.down_recursive(0);
            assert!(recursive_mutex.get_recursive_count() == i);
        }

        for j in 1..11 {
            recursive_mutex.up_recursive();
            assert!(recursive_mutex.get_recursive_count() == 10-j);
        }
        kernel::task_end_scheduler();
    };

    let recursive_mutex_holder = task_control::TCB::new()
        .name("Recursive_mutex_holder")
        .priority(3)
        .initialise(mutex_holder);

    kernel::task_start_scheduler();
}
