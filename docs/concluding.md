# Rust-FreeRTOS结题报告

我们的项目是利用新兴的Rust语言改写FreeRTOS操作系统。

改写不是抄写。在项目开展的过程中，我们按照FreeRTOS的程序逻辑，针对Rust语言的特点做出了很多富有创造力的设计，我们将在以下部分进行介绍。

最终，我们成功实现了开题时提出的目标，完成了对FreeRTOS中所有的内核模块——**移植（`port`）**模块、**链表**（`list`）模块、**任务调度**（`task`）模块和**队列与信号量**（`queue`）模块——的改写；为了更好地发挥多人协作的作用，我们将以上模块进一步细分，形成了十余个轻量的子模块；截至我们开始撰写结题报告时，我们的总代码量（不包括底层的C语言代码）已经达到了**6000行以上**，git仓库的commits数量**200多次**，小组的每个成员都贡献了大量的代码；此外，我们希望这个项目更加专业，并且能够真正被他人使用，所以我们提供了详细的[文档]()和各个功能的[示例代码]()。

在本报告中，我们首先将对Rust-FreeRTOS项目进行简要介绍，然后重点介绍我们的项目设计，最终介绍我们的测试结果。

我们的小组成员：**樊金昊 左顺 宁雨亭 黄业琦 张俸铭 雷思琦**

## 项目背景

TODO：简要总结开题报告和可行性报告

## 项目设计

### 总体设计概览

#### 细致的模块化设计

原本的FreeRTOS实现中，task模块仅位于`tasks.c`中，queue模块也全部在`queue.c`中实现，这样尽管使代码更加紧凑，但却使代码的可读性大大下降。同时，这也不利于我们小组中多人合作编写代码。

为了使项目结构更加清晰，同时达到每人独立编写一个模块的效果，我们根据功能，对task和queue模块进行了进一步的模块化设计，`task`被分为`kernel`、`task_api`、`task_control`、`task_global`、`task_queue`和`task_timemanager`这六个模块，`queue`也被分为`queue`、`queue_api`、`semaphore`这三个模块。

#### 基于Cargo feature的内核裁剪功能

FreeRTOS中提供了二十余个用于裁剪内核的宏，例如`IncludeTaskDelete`等，我们[利用Cargo的feature功能]，成功实现了FreeRTOS中所有的条件编译，具体技术细节在我们之前写过的[这篇文章](https://github.com/OSH-2019/x-rust-freertos/tree/task/cfg)中，在此不再赘述。条件编译的配置在[Cargo.toml](https://github.com/OSH-2019/x-rust-freertos/blob/880890850098e52a90c335f8e3eb67dfbf38645b/rust_freertos/Cargo.toml#L17)中。

#### 全局变量的处理

FreeRTOS中，处于不同状态的任务队列、内核运行状态等数据都是以全局变量的形式存储的，但是Rust不鼓励使用全局变量，因为它可能造成数据竞争。因为Rust不支持结构体作为全局变量，所以我们使用了[lazy_static](https://docs.rs/lazy_static)包来封装任务链表。此外，我们使用全局mutable变量来存储系统状态，并创建`getter`和`setter`，用`unsafe`统一对其进行访问，例如：

```rust
pub static mut TICK_COUNT: TickType = 0;

#[macro_export]
macro_rules! get_tick_count {
    () => {
        unsafe { crate::task_global::TICK_COUNT }
    };
}

#[macro_export]
macro_rules! set_tick_count {
    ($next_tick_count: expr) => {
        unsafe {
            trace!("TICK_COUNT was set to {}", $next_tick_count);
            crate::task_global::TICK_COUNT = $next_tick_count;
        }
    };
}
```

因为操作系统状态变量只会被内核函数访问，所以此处不会发生数据竞争，可以放心使用`unsafe`。

#### 完善的日志

在上一部分中，我们已经展示了`trace!`函数的使用，在我们的实现中，我们广泛地使用了日志函数，以下是我们某次测试中的一段日志：

```
13:15:11 [TRACE] rust_freertos::task_control: [src/task_control.rs:127] Initialising Task: main, stack size: 512 bytes
13:15:11 [TRACE] rust_freertos::ffi: [src/ffi.rs:40] vTaskSuspendAll() called from ffi!
13:15:11 [TRACE] rust_freertos::kernel: [src/kernel.rs:400] SCHEDULER_SUSPENDED was set to 1
13:15:11 [TRACE] rust_freertos::ffi: [src/ffi.rs:46] xTaskResumeAll() called from ffi!
13:15:11 [TRACE] rust_freertos::kernel: [src/kernel.rs:471] SCHEDULER_SUSPENDED was set to 0
13:15:11 [INFO] task_resume_all() returned false
13:15:11 [TRACE] rust_freertos::task_control: [src/task_control.rs:141] stack_pos for task main is 140500632868624
13:15:11 [TRACE] rust_freertos::task_control: [src/task_control.rs:152] Function ptr of main is at 7FC8DA5016F0
13:15:11 [TRACE] rust_freertos::ffi: [src/ffi.rs:40] vTaskSuspendAll() called from ffi!
13:15:11 [TRACE] rust_freertos::kernel: [src/kernel.rs:400] SCHEDULER_SUSPENDED was set to 1
13:15:11 [TRACE] rust_freertos::ffi: [src/ffi.rs:46] xTaskResumeAll() called from ffi!
13:15:11 [TRACE] rust_freertos::kernel: [src/kernel.rs:471] SCHEDULER_SUSPENDED was set to 0
```

这段日志显示了任务创建的过程。可见，日志让我们的代码执行过程变得可视化。在我们的调试过程中，日志的作用至关重要。

### 硬件接口——Port模块

Port模块是与体系结构相关的，每一个体系结构都有自己的一套port实现，FreeRTOS 8中就提供了面向以下体系结构和编译器的port模块：

![Screen Shot 2019-07-06 at 10.54.34 AM](/Users/fandahao1/Documents/19Spring/OS/x-rust-freertos/docs/concluding.assets/Screen Shot 2019-07-06 at 10.54.34 AM.png)

这意味着，我们是不可能把每一个port都用Rust改写一遍的。但是，所有的port函数都提供了统一的API接口，所以我们决定利用Rust封装这些API接口。有了这些封装，**我们的代码理论上可以在任何FreeRTOS和LLVM支持的平台上运行**。

因为不同体系结构和编译器上Rust和C语言的接口是不同的，所以以上封装是**由程序自动进行的**，我们使用了[Bindgen](https://github.com/rust-lang/rust-bindgen)工具来生成C代码对应的Rust函数，并利用[CC](https://docs.rs/cc)库来编译C代码，并和Rust程序链接起来，以上过程均在[build.rs](https://github.com/OSH-2019/x-rust-freertos/blob/master/rust_freertos/build.rs)中完成。

因为Bindgen生成的Rust函数是`unsafe`函数，所以我们又在[port.rs](https://github.com/OSH-2019/x-rust-freertos/blob/master/rust_freertos/src/port.rs)中对这些函数进行了一层safe封装，这是Rust中的通行做法；此外，对于C语言中调用的Rust函数，我们也利用Rust的[**FFI**](https://github.com/OSH-2019/x-rust-freertos/blob/master/rust_freertos/src/ffi.rs)为他们生成了对应的C函数。这样，**port层的C代码和Rust代码就可以互相调用了**。

### 基本数据结构——链表

TODO：左顺写

### TCB和TaskHandle结构体

TODO：樊金昊写

### 任务控制函数

TODO：挂起、恢复、delay等，黄业琦写（这一部分代码似乎还有编译问题）

### 任务API函数

TODO：张俸铭写，这一部分代码还有编译错误

### 队列与信号量

TODO：宁雨亭和雷神写，可以多写点。

## 测试

TODO：现在task和queue的基本功能已经测试过，主要还需测试以下内容：

* 任务挂起恢复
* 任务API里改变任务优先级的那个函数
* semaphore和mutex

负责这几部分的同学这两天可以写一些测试代码测试一下，就像`src/lib.rs`里的测试一样，具体测试可以模仿[freertos.rs](https://github.com/hashmismatch/freertos.rs/tree/master/qemu_stm32_tests/examples)里的。

如果这几部分测试都比较顺利，我们回头用一些benchmark测一下性能，与C语言的实现对比一下。

## 总结与不足

TODO：最后再写