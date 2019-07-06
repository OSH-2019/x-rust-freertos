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

TCB，即任务控制块，是任务调度的基本单元，TaskHandle是任务句柄，用户通过其实现对任务的各种操作，因为TCB和TaskHandle涉及大量指针操作，所以在这一部分我们花费了大量时间，对原有的结构做出了很多修改。

#### 利用Callback机制调用任务函数

这样，C语言代码中实际调用的函数是：

```rust
/* Task call wrapper function. */
extern "C" fn run_wrapper(func_to_run: CVoidPointer) {
    unsafe {
        let func_to_run = Box::from_raw(func_to_run as *mut Box<FnBox() + 'static>);
        func_to_run();
    }
}
```

此外，还必须要在任务堆栈初始化成功后手动`forget`即将被运行的函数，以免Rust的内存管理机制在初始化函数执行结束后将其free掉：

```rust
        match result {
            Ok(_) => {
                /* We MUST forget `f`, otherwise it will be freed at the end of this function.
                 * But we need to call `f` later in `run_wrapper`, which will lead to
                 * some unexpected behavior.
                 */
                mem::forget(f);
            }
            Err(e) => return Err(e),
        }
```

#### TaskHandle类型

在原本的C代码中，TaskHandle是一个指针（实际上是一个指向TCB的指针）：

```c
typedef void * TaskHandle_t;
```

事实上，FreeRTOS利用`void *`实现了类似多态的类型转换，这显然是不安全的，所以Rust不允许这样的类型转换。因为可能会有多个`TaskHandle`同时指向并有权修改同一个TCB，我们采用智能指针对其进行封装：

```rust
#[derive(Clone)]
pub struct TaskHandle(Arc<RwLock<TCB>>);
```

这一定义与链表中的`owner`域定义类似，所有与任务相关的函数，都是以TaskHandle为参数的，这使得Task的使用很灵活。

#### DRY (Don't repeat yourself)

由上一部分可以看出，TaskHandle、List、ListItem的定义都采用了多层只能指针的封装，他们的定义比较复杂。事实上，由于涉及到多次对智能指针的操作，他们的使用也非常复杂。例如，下面是一个判断两个TaskHandle是否指向同一TCB的函数：

```rust

```



### 任务控制函数



### 任务控制函数



#### 任务创建

```rust
pub fn initialise<F>(mut self, func: F) -> Result<TaskHandle, FreeRtosError>
    where
        F: FnOnce() -> () + Send + 'static,
```

我们将任务创建和任务信息初始化的函数合一处理。

首先为任务申请空间，栈空间我们采用字对齐处理，实现如下。

```rust
let px_stack = port::port_malloc(stacksize_as_bytes)?;
```

之后标记栈空间信息。

```rust
let mut top_of_stack = self.stack_pos + self.task_stacksize as StackType - 1;
top_of_stack = top_of_stack & portBYTE_ALIGNMENT_MASK as StackType;
```

申请空间如果失败，则会返回`Err`信息，我们不作处理；如果申请成功，我们则会装入TCB的相关信息和数据。例如：任务名称，任务函数信息，任务函数参数信息，任务优先级等等……

除了这些信息，还有两个任务相关的列表项要被初始化，`state_list_item`以及`event_list_item`用于任务队列调度。

最后我们将初始化完成的任务放置在就绪队列`ready_list`中。

```rust
handle.add_new_task_to_ready_list()?;
```

就此，任务穿件过程完毕。

详见`task_control L182-L279`



#### 添加任务至就绪列表

之前再创建任务的时候也有使用过，我们创建的任务需要加入就绪列表中。

```rust
fn add_new_task_to_ready_list(&self) -> Result<(), FreeRtosError>
```

在这个过程中，为了保证正确性，我们先进入临界区，使用我们已经编写好的`taskENTER_CRITICAL!()`和`taskEXIT_CRITICAL!()`API。

我们添加新任务同时，我们将一些辅助维护的全局变量也加以维护。

例如：`current_number_of_tasks`

之后调用`list API`完成添加。

详见`task_control L527-L571`。



#### 任务删除

```rust
pub fn task_delete(task_to_delete: Option<TaskHandle>)
```

首先先使用`get_handle_from_option`转换数据类型，方便之后处理。

与之前添加任务一样，删除任务时，我们也需要进入临界区进行操作。

进入临界区之后步骤如下：

1. 将任务从就序列表中删除，如果成功删除，则重置优先级

2. 判断任务是否在等待事件，如果是，则删除任务对应的`event_list_item`

3. 如果删除的任务是正在运行的任务，则需要多执行一次任务切换过程


由于删除任务我们需要删除任务控制块以及任务堆栈所占用的空间，但是任务正在运行的话，显然任务控制块和任务堆栈不能直接释放，我们需要设置标记，将任务移动到`task_waiting_termination`列表。之后在一一释放内存。

详见`task_control L815-L889`。



#### 任务挂起

```rust
pub fn suspend_task(task_to_suspend: TaskHandle)
```

同样，先转换变量类型，再进入临界区。

步骤如下：

1. 将任务从就序列表或者延迟列表中移除
2. 判断任务是否在等待事件，如果是，则删除任务对应的`event_list_item`

至此可以离开临界区，因为之后添加任务至挂起列表操作是不需要再临界取中执行的。

接着，我们计算还要多长时间执行下一个任务，也就是任务的解锁时间，防止有任务参考了刚才被挂起的任务，我们使用`reset_next_task_unblock_time()`函数进行处理。

还存在一个特例——需要挂起的任务是正在执行的任务。如果这种情况发生，我们需要特殊处理，再任务调度器没有异常的情况下，我们调用函数`portYIELD_WITHIN_API!()`进行强制切换。切换完之后还不算结束，因为我们的全局变量`current_tcb`指向我们正在执行的任务，当他被挂起之后，我们需要再找一个其他的任务放在里面。

假如没有其他被挂起的任务，我们调用`task_switch_context()`获取下一个要执行的任务。

详见`task_control L913-L975`。



#### 任务恢复

```rust
pub fn resume_task(task_to_resume: TaskHandle)
```

首先我们有两种情况比较特殊，不能恢复：

1. 要我们恢复的任务为NULL
2. 要恢复的就是当前正在执行的任务

如果不是这两中情况，则可以进入临界区进行操作：

1. 先判定任务是否已经被挂起，调用函数`task_is_tasksuspended()`
2. 将要恢复的任务从挂起列表中移除
3. 将要恢复的任务放进就序列表中
4. 如果要恢复的任务优先级高于当前正在执行的任务，调用`taskYIELD_IF_USING_PREEMPTION!()`进行任务切换

详见`task_control L1031-L1065`。



#### 任务挂起判定

```rust
pub fn task_is_tasksuspended(xtask: &TaskHandle) -> bool
```

简单地说，是利用`list API`对于挂起列表进行查询。

详见`task_control L977-L1006`。



#### 任务延迟函数

```rust
pub fn task_delay(ticks_to_delay: TickType)
```

参数为我们需要延迟的节拍数，延迟节拍数小于0时，相当于直接执行了`port_YIELD`进行任务切换。

如果延迟节拍数大于0，我们先挂起任务调度器，利用函数`add_current_task_to_delayed_list()`将我们要延迟的任务移动到`delay_list`中，之后再恢复任务调度器。如果此时任务调度器没有调度任务，我们手动调用`portYIELD_WITHIN_API!()`进行调度。

详见`task_timemanager L55-L77`。



#### 将任务添加至延迟队列中

```rust
pub fn add_current_task_to_delayed_list(ticks_to_wait: TickType, can_block_indefinitely: bool)
```

详见`task_control L650-L750`。

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