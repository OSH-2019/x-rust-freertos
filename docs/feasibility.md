# 可行性报告

* [项目介绍](#项目介绍)
* [理论依据](#理论依据)
  * [适用于系统编程的Rust语言特性](#适用于系统编程的Rust语言特性)
    * [条件编译](#条件编译)
    * [Unsafe Rust](#Unsafe-Rust)
  * [FreeRTOS架构分析](#FreeRTOS架构分析)
    * [FreeRTOS的编码风格](#FreeRTOS的编码风格)
    * [与任务相关的函数](#与任务相关的函数)
    * [队列与信号量](#队列与信号量)
* [技术依据](#技术依据)
  * [利用QEMU进行FreeRTOS仿真](#利用QEMU进行FreeRTOS仿真)
  * [一个尝试-任务链表的Rust实现](一个尝试：任务链表的Rust实现)
* [项目设计](项目设计)
  * [使用的FreeRTOS版本和硬件平台](#使用的FreeRTOS版本和硬件平台)
  * [如何处理C语言指针？](#如何处理C语言指针？)
  * [如何处理C语言的宏定义？](#如何处理C语言的宏定义？)
  * [怎样进行模块化设计？（对FreeRTOS的函数结构作出多大改变）](#怎样进行模块化设计？（对FreeRTOS的函数结构作出多大改变）)
  * [初期计划](#初期计划)
* [创新点](创新点)
* [参考文献](#参考文献)

## 项目介绍

我们选择的课题是使用Rust语言改写FreeRTOS操作系统。

众所周知，在物联网不断发展的今天，我们身边的嵌入式设备不断增多，它们中的很多都使用了**实时操作系统**。然而，嵌入式设备和实时操作系统也暴露出了很多安全问题。对这些安全问题，C语言因其本身的缺陷难辞其咎。

**Rust**语言被开发的主要目的就是在保有与C语言同样速度的情况下保证安全。它是一门非常适于系统编程的语言，从长期来看，具有取代C/C++的趋势。利用Rust编写实时操作系统，可以充分发挥它相对C语言的高层抽象和内存安全保证。

因此，我们计划使用Rust语言改写目前最流行的实时操作系统——FreeRTOS。在这份报告中，我们将首先介绍项目的理论依据——Rust语言适用于系统编程的特性及对FreeRTOS源码的分析和改写思路；随后，我们将介绍使用QEMU进行硬件仿真和测试的方法，以及我们初步编写的FreeRTOS最核心的数据结构——任务链表；最后，我们将给出项目的总体设计和创新点。

## 理论依据

### 适用于系统编程的Rust语言特性

#### 条件编译

在`FreeRTOS`的`FreeRTOSConfig.h`中，有很多用于配置的[预定义](#FreeRTOS中的预定义)。在主程序代码中，这些配置选项广泛地被用于条件编译。Rust通过`cfg`属性，也对条件编译提供了支持。

Rust的这些配置在`Cargo.toml`中提供，例如：

```toml
[features]
# no features by default
default = []
 
# The “secure-password” feature depends on the bcrypt package.
secure-password = ["bcrypt"]
 
# A feature with no dependencies is used mainly for conditional 	 compilation, like `#[cfg(feature = "go-faster")]`.
go-faster = []
```
在代码中，可以采用`cfg`属性修饰函数：

```rust
// This function is only included when compiling for a unixish OS with a 32-bit architecture
#[cfg(all(unix, target_pointer_width = "32"))]
fn on_32bit_unix() {
  // ...
}
```

也可以使用以下类似`#ifdef`的方式进行条件编译：

```rust
let machine_kind = if cfg!(unix) {
  "unix"
} else if cfg!(windows) {
  "windows"
} else {
  "unknown"
};

println!("I'm running on a {} machine!", machine_kind);
```

----

#### Unsafe Rust

在进行底层系统编程时，我们必须要使用Rust 的`unsafe`关键字，使用该关键字可以使我们实现以下功能：

* 解引用原生指针，这对于和C语言和汇编语言程序交互很有用。
* 调用一个不安全的函数或方法（例如C语言函数）。
* 访问和修改静态变量，后者是Rust中的"全局变量"。

使用`Unsafe Rust`会在一定程度上减少Rust编译器的安全检查，但牺牲的稍许安全性却可以换来灵活性的巨大提高。在与硬件抽象（定义于`portmacros.h`中）进行交互时，我们可能需要使用这个特性。

----

### FreeRTOS架构分析

#### FreeRTOS的编码风格

##### FreeRTOS中的预定义

- config.h中的define语句为相关配置参数（类似树莓派裁剪的menuconfig）
- INCLUDE开始的宏：函数的使能、除能。 当INCLUDE_func值为1时，表明func函数（及其相关函数）API接口可以使用 （task.c中多处条件编译相关）
- config开始的宏：表明一些参数状态 如：configASSERT(x)会在内核中的关键点被调用，如果参数为0说明有错误发生；configCHECK_FOR_STACK_OVERFLOW会指示当前状态堆栈是否溢出

##### 编码风格

没使用C99的语法和新特性 使用了stdint.h 当编译器没有这个头文件时，include文件夹里有stdint.readme，重命名就行

**命名规则**
u-unsigned
l-long
s-short
c-char
非stdint.h定义的类型变量前缀用x
非stdint.h定义的无符号类型变量前缀用u(ux)
size_t的变量前缀也用x
枚举变量-e
指针-p
char-c
char* - pc

**-函数**
static-prv
API函数 有无返回值的加v，表示void
API函数前面会告诉自己在哪个文件里，如vTaskDelete在task.c里

-宏 名字包含文件夹，config 其他部分大写，空格用下划线隔开

-数据类型 有4个移植层的变量，根据一些宏不同的值表示为不同的类型

-注释
不使用//这种单行注释的方式 函数的每一行注释前面都要加* ，应该是为了增加易读性 include文件有一定的顺序，若顺序错了会报错
{符号占单独一行
每个函数实现完有分割线

----

#### 与任务相关的函数

任务（`task`）是FreeRTOS最核心的成分和最重要的组成单元，FreeRTOS中一半的代码都涉及任务处理和调度。这一部分将对FreeRTOS与任务相关的函数进行介绍。

##### FreeRTOS任务运行和调度流程

- main函数初始化硬件外设
- 调用xTaskCreate()创建开始任务（执行一次，创建其他应用任务的信号量、队列等内核对象
- 开启任务调度器
- 开始任务会完成其他任务的创建(start_task函数)

##### FreeRTOS任务系统

FreeRTOS中的每个任务都有自己的运行环境，不依赖于系统中的其他任务或者调度器。RTOS调度器则需要确保一个任务开始执行时其上下文环境和任务上一次退出时相同（通过堆栈的存取操作）。 FreeRTOS中的任务存在4个状态：

- 运行态：任务正在运行
- 就绪态：任务正在排队等待，一旦有资源可以进入运行态
- 阻塞态：任务正在等待某个外部事件（有延时限制）
- 挂起态：任务不能进入运行态，除非被Resume退出挂起态

![在这里插入图片描述](https://camo.githubusercontent.com/cac7200eae129a9da89aec601eeeafdb71c90cd3/68747470733a2f2f696d672d626c6f672e6373646e696d672e636e2f32303139303430373135313631303930332e706e673f)

每个任务都会分配一个相应的优先级，取值范围从0~(configMAX_PRIORITIES-1)，RTOS调度器会在就绪队列中根据优先级进行相应的调度

FreeRTOS通过TCB（任务控制块）这个结构体集合一个任务的属性。在任务被创建时，会同时给它分配一个tskTaskControlBlock。任务堆栈与TCB受函数的统一调配，保证任务调度的正常。 FreeRTOS同样支持时间片轮转的调度方式。

##### 任务相关API函数

- 创建与删除：xTaskCreate(),xTaskDelete() 新创建的任务默认为就绪态，创建任务的主要工作是申请堆栈空间、传递任务的参数、设置优先级、返回任务状态（追踪信息等）。 删除任务时，xTaskDelete()会根据传递的任务句柄进行删除。由内核分配给任务的内存会在此过程中释放掉，但用户分配的任务内存需要用户自行释放。
- 任务挂起和恢复：vTaskSuspend(),vTaskResume() vTaskSuspend()将任务设置为挂起态，该任务将不会进入运行态，直到调用vTaskResume()。后者会将任务设置为就绪态。 这两个函数都是通过任务句柄来修改任务状态的参数值来实现的。
- SVC中断服务函数、空闲任务函数
- 任务相关API函数举例 uxTaskPriorityGet() 获取任务的优先级 vTaskGetInfo() 获取任务状态 xTaskGetHandle() 获取任务句柄 xTaskGetTickCount() 获取调度器计数器的值

```
在《FreeRTOS源码详解与应用开发》的第8、9章详细介绍了FreeRTOS任务相关函数的代码实现
在《ARM Cortex - M3权威指南》中有对Cortex-M处理器的架构介绍（包括寄存器用途、指令等）
```

##### 任务模块中Rust改写难点

- 调度器在切换不同的任务时，需要访问和更改任务优先级、任务状态等参数值。而Rust要求当一个任务的状态正在被修改时，其它函数将无法获取任务的状态值，可能会出现阻塞异常。
- ownership所有权和lifetime生命期的问题。为保证指针或任务参数的生命期，需要多处使用引用传递参数的形式。在某些数据结构上可能会产生异常。
- 任务挂起和恢复的实现方式 当任务被挂起时，任务状态会被调度器设置为挂起态，需要等到一个Resume()将它恢复到就绪态。此时可能存在TaskSuspend()、TaskResume()同时访问任务状态，这一过程可能会导致访问权限问题。
- 一些中断函数（例如PendSV）采用汇编代码编写，其中也涉及到了C函数宏和参数。在Rust与底层汇编交互时需要设计合理的API。

----

#### 队列与信号量

##### FreeRTOS队列

没有操作系统的时候，两个应用程序传递消息的方式是通过全局变量的方式传递
在操作系统中，用队列来实现任务与任务、任务与中断之间的消息传递

队列里储存有限的、大小固定的数据项目，交流的数据保存在队列中，叫队列项目。

队列能保存的最大数据项目的数量叫做队列长度。创建队列会指定数据项目的大小和队列的长度，也称消息队列。

------

##### 简单了解队列

1. 数据储存
   队列采用的缓存机制实现FIFO或者LIFO都可以
   往队列里发送数据的时候，数据被复制，表示队列里存的是数据的原始值而不是指向它的指针，原数据可以不必须一直保持可见
   虽然值传递需要一些空间，但是一但消息发送到队列则原始数据的缓冲区就可以被回收，进而重复使用。
   如果需要仍然可以把指针作为值传进去，跟引用传递的意思类似。传递指针是有必要性的，因为比如在网络应用环境中，网络数据量非常大，采用数据复制不现实。
2. 多任务访问 队列不属于某个特定的任务，任何任务都可以读或取。
3. 出队阻塞 去队列里读取数据时，若队列为空 由出队阻塞时间决定等多久， 参数范围：0(不等) ~ `port-MAX_DELAY`(一直等)
4. 入队阻塞
   同理

在一个任务从队列中读取了一个消息以后，可以清空或者不清空，不请空就还可以给别人读

------

struct QueueDefinition 包含指向头和要下一个空闲区域的指针
里面包含了一个union，当这个结构体作为不同的东西(队列或者信号量)的时候，里面的数据类型不同
还包含了等待(接受or发送)列表
当前队列项数量
创建时允许的最大队列长度
创建队列时每个队列项最大长度
还有与队列上锁相关的变量，当队列上锁时，`cRxLock`和`cTxLock`分别统计出队和入队的队列项数量，当没有上锁时，两个值均为queueUNLOCKED

------

```
xQueueCreate()`和`xQueueCreateStatic()`都是宏，实际上创建函数的是`xQueueGenericCreate()`和`xQueueGenericCreateStatic()
```

创建成功返回句柄，失败返回NULL

Static需要用户自己分配储存区

队列类型有六种： 普通消息队列、 队列集、 互斥信号量、 计数型信号量、 二值信号量、 递归互斥信号量， 默认是第一种

xQueueGenericCreate()详解 configASSERT来保证给出的队列长度>0 调用`pvPortMalloc()`分配内存 这里申请到的内存大小是队列结构体和队列中消息储存区的总大小 然后初始化队列

总体：分配空间+调用初始化函数

prvInitialiseNewQueue()详解
有一个小技巧： 首先一进来先定义了一个 `( void ) ucQueueType;`
其实这个`ucQueueType`是传进来的一个参数 `const uint8_t ucQueueType`
在`configUSE_TRACE_FACILITY`被设置成1的时候会运行 `pxNewQueue->ucQueueType = ucQueueType;` 但是当`configUSE_TRACE_FACILITY`不等于1时，由于不会编译使用它(指传进来的参数)的那一部分，所以编译器会报unused parameters，如果这时候来一个 `( void ) ucQueueType;`
就不会有讨厌的warning了

还有即使传入参数的列表项为0，但是也不能直接设置`PChead`为NULL，因为那个代表这个queue被用作互斥锁

同时使用到了`pdTRUE`和`pdFALSE`宏

总体：改pcHead+对初始化队列结构体的成员变量赋值+调用队列复位函数

xQueueGenericReset()详解

先初始化队列相关成员变量
由于复位以后队列是空的,所以由于出队而保持阻塞的任务继续阻塞，而由于人队而保持阻塞的要解除阻塞,从列表中移除

总体：初始化成员变量+决定是否创建新的队列+初始化队列的链表xTasksWaitingToSend和xTasksWaitingToReceive

创建完成的初始队列，前面是队列结构体，储存关于这个队列的信息，而后面是消息储存区，就是实际上队列项所在的位置

------

向队列发送消息
可以分为任务级入队函数和中断级入队函数(FormISR都是中断级的)

可以前向入队和后向入队，还有带覆写功能的(覆写当前pcWriteTo指针指向的队列项目)
我们目前使用的的版本是通过一个 `xCopyPosition == queueOVERWRITE`来判断是否覆写

中断就是中断服务函数

API任务级入队函数也是宏，调用`xQueueGenericSend()`
只能用于任务函数，不能用于中断服务函数
返回值：
`pdPass`成功向队列发送消息
`errQUEUE_FULL`则队列已满

API中断服务函数也还是宏，调用`xQueueGenericSendFromISR()`
中断级的函数不需要阻塞时间的参数
并且有一个参数可以选择是否在退出中断服务函数之后马上进行任务切换。

返回值：
`pdTRUE`成功
`errQUEUE_FULL`失败

**任务级通用入队函数详解**
首先确定队列是不是满的
未满或者覆写入队，则可以将消息入队
选择前向入队或者覆写是直接把数据拷贝到现在u.pcReadFrom指向的数据项目，如果是后向入队则将消息复制进pcWritrTo指向的队列元素，不管是哪种方式，复制后指针都要移动
若又任务由于请求消息阻塞，则把该任务从等待列表移到就绪列表，如果调度器上锁,那么这些任务就会挂到xPendingReadyList上。
如果取消任务的优先级高还要标记需要进行任务切换。(源代码的注释里说可以在critical section里直接切换,kernel会自己搞定的)
用时间结构体和超时结构体来计算阻塞时间 队列已满切阻塞时间不为0的情况下：
先给队列上锁(给队列上锁不能操作事件链表，解锁时会补上操作) —— 更新超时结构体，检查阻塞时间是否到了 ———— 没到 —— 检查队列是否是满的 —— 满的 ——
—— 1. 将任务从就绪列表中移除，加入`TaskWaitingToSend`和延时列表
—— 2. 若阻塞时间无限且`INCLUDE_vTaskSuspend==1`，则添加任务到`xSuspendedTaskList`上 队列没满 —— 重新试一次
(若阻塞时间到了直接跳到这一步)
解锁队列
恢复任务调度器

源码的注释里解释说，解锁队列意味着队列事件可以影响事件列表，现在就有可能出现中断，并把这个任务从事件列表里删除，但这是调度器还是被挂起的，所以任务会去到`PendingReady`列表(这里源码的注释好像出现了小错误，list打成了last)，而不是真正的就绪列表

恢复调度器后，任务又从`PendingReady`列表去到了就绪列表，所以在一个任务yield之前就已经在就绪列表里是可行的，在这种情况下，yield不需要进行上下文切换，除非有一个更高优先级的任务在`PendingReady`队列里
(看了这一段，我个人理解，马上要执行的任务在`PendingReady`列表中)

**中断级通用入队函数**
跟任务级类似
有一处不同：在这个函数中用cTxLock来记录了队列上锁期间像队列发送了数据(后面有讲处理)
还有就是有关任务切换的处理

------

队列上锁解锁
prvLockQueue()
本质上是个宏 具体内容非常简单，就是改`cRxLock`和`cTxLock`为`queueLOCKED_UNMODIFIED`

prvUnlockQueue()
必须要在调度器被挂起的时候调用才能调用这个函数 lock记录了加入和删除的项，在上锁期间，项可以被删除或者加入，但是相应的事件列表不会更新
所以说实际上队列里面已经被传入了新的消息，但是由于没有更新列表，所以那些原来在等着接受消息的任务不知道已经有消息了，还在一直等着
如果`xTasksWaitingToReceive`不为空，则将任务从此列表中移除，如果优先级高，则进行任务切换，实际上的任务切换是在`xTaskIncrementTick()`实现的
处理完一条就`cTxLock-1`，直至完成然后设置成`queueUNLOCKED`
用同样类似的方法处理`cRxLock`

------

出队

同样也还是任务和中断，还分读取消息后删不删队列项

又是两个宏
读取数据是进行数据复制，所以用户需要提供数组或者缓冲区保存数据
也有阻塞时间的参数，但是不用提供一个数据项的大小 死等时，要求`INCLUDE_vTaskSuspend`必须为1
返回值：
`pdTRUE`成功
`pdFALSE`失败

小知识：.h中在函数声明的后常常会看见`PRIVILEGED_FUNCTIONS`这个参数是告诉MPU把这个函数储存在专门的内存里

**任务级通用函数**
xQueueGenericReceive()
xJustPeek参数决定是否删除队列项
我们的版本中xQueuePeek和xQueueReceive仍然分开实现

Peek里面还用了一个变量来暂存ReadFrom以保证读取前后指针位置无变化，还检查是否还有其他任务要这个数据

**中断级函数**
xQueueReceiveFromISR 介绍了为什么要搞成不同的函数(指中断级和任务级)的原因，是因为优先级的问题，所以特别为中断写了专用的函数
在ISR中不能阻塞，所以需要检查数据是否是可用的
操作cRxLock

----

##### 信号量

信号量一般用来进行资源管理和任务同步
可以分类为二值信号量、计数型信号量、互斥信号量、递归互斥信号量

中断服务函数一定要快进快出，不能有太多的代码，否则影响中断的实时性。裸机编写的时候，一般是在中断中打标记(信号量)，在其他地方根据标记执行具体过程

**二值信号量**
用于互斥访问或者同步
互斥信号量有优先级继承机制,二值没有
因此二值适用同步
高优先级任务优先获得信号量

二值信号量 —— 只有一个队列项的队列(要么满要么空)

二值信号量创建函数
vSemaphoreCreateBinary()
我们的版本里并没有这个函数
实际上的创建过程是由xQueueGenericBinary()来实现的 与创建队列的区别：在union里有不一样，队列长度固定为1以及队列项大小为0 队列类型为`queueQUEUE_TYPE_BINARY_SEMAPHORE`

返回值:
NULL失败
其他值成功

我们这个版本里没有用来创建二值信号量的函数
但是有一个宏叫做`queueQUEUE_TYPE_BINARY_SEMAPHORE`，对应Queue结构体中`uint8_t ucQueueType`。

队列满和空通过`uxMessagesWaiting`来判断

在我们现在使用的版本中没有通用的释放信号量的函数

实际上通过`xQueueSemaphoreTake()`来获取信号量

**计数型信号量**
xQueueCreateCountingSemaphore()
可传入计数信号量最大计数值和初始量
返回值：
NULL-失败
句柄-成功

------

使用二值信号量时会遇到一个非常常见的问题：
优先级翻转
实时系统中不允许这种现象，会破坏任务的预期顺序
示例：L优先级最低却先拿到了信号量，dangH剥夺CPU使用权，相当于L和H同一优先级，而当M剥夺了L的CPU使用权时，相当于M的优先级高于H，出现优先级翻转

------

**互斥信号量**
其实是拥有优先级继承的二值信号量

我们的版本中使用了 `xQueueCreateMutex()` 与之前的不同的是还进行了互斥信号量的初始化`prvInitialiseMutex()`

返回值：
NULL-失败
句柄-成功

prvInitialiseMutex()内容
对队列中一些成员进行了必要的赋值(跟优先级继承有关系) 定义了三个宏专为互斥信号量准备，
xMutexHolder - pcTail
uxQueueType - pcHead
queueQUEUE_IS_MUTEX - NULL
用于表示互斥信号量时，将pcHead指向NULL来表示pcTail保存着互斥队列的所有者，pcMutexHolder指向拥有互斥信号量的任务块
重命名是为了增强可读性
函数的最后调用了一次xQueueGenericSend，说明互斥信号量默认有效

**释放互斥信号量**

我们的版本没有通用函数SemaphoreGive()，暂时没有找到其释放函数

用xTaskPriorityDisinherit()来进行优先级继承

xTaskPriorityDisinherit()内容
若存在优先级，则当前任务的优先级和任务的基优先级不同 若没有其他的互斥锁，则改变其优先级
如果一个mutex是一个被held的状态，那么一定不是从中断给出的，如果mutex是从它的holding task给出的，那么那个task一定正在运行。
于是把holding task从就绪列表中移除
使用新的优先级将任务重新添加到就绪列表(解除优先级??因为实际上是以BasePriority来新加入的)

xMutexHolder表示拥有此互斥信号量任务控制块，所以先判断是否已经被其他任务获取

任务可能会获取多个信号量，用uxMutexesHeld来记录获取到的互斥信号量个数，任务每释放一次，就要-1

判断是否是任务获取的最后一个信号量，如果还获取了其他互斥信号量，那么就不能处理优先级继承，优先级继承的处理必须是在释放最后一个互斥信号量的时候(貌似说一个任务有优先级，那么就一定获取了互斥的信号量)

优先级继承就是将任务从当前优先级降到基优先级，所以先从就绪列表中移除，当任务恢复到原来优先级后再加入就绪列表(为啥呢?????)

如果任务继承来的优先级对应的就绪表中没有其他任务，那么取消这个优先级的就绪态(????)

当获取信号量的顺序和释放信号量的顺序不同的时候要进行上下文的切换

**获取互斥信号量**
xQueueSemaphoreTake()内容
用prvCopyDataFromQueue，用数据复制的方式从队列中提取数据
获取了互斥信号量后uxMessagesWaiting-1，将数据删除掉
获取信号量成功以后，pxMutexHolder要标记互斥信号量所有者，调用pvTaskIncrementMutexHeldCount，就是使任务控制块中uxMutexHeld加一，并且返回是当前任务的控制块

如果运行到，发现互斥信号量被其他任务占用，如果当前任务的优先级比正在拥有互斥信号量的任务优先级高，则把拥有互斥信号量的低优先级任务调整为与当前任务相同的优先级

**递归互斥信号量**
递归信号量很神奇，已经获取了互斥信号量的任务还能再次获取这个递归互斥信号量(互斥信号量就不行)，并且次数不限，获取的次数要跟释放的次数一样

递归互斥信号量也有优先级继承的机制，所以用完递归互斥记得释放，同互斥一样，此信号量也不能用在中断服务函数中，因为优先级继承的存在，中断服务函数不能设置阻塞时间)
需要某个宏为1以使用递归互斥信号量

**释放递归互斥信号量**
xQueueGiveMutexRecursive()注意点

检查递归互斥信号量是不是当前任务获取的，必须是递归互斥信号量的拥有者才能释放
uxRecursiveCallCount这个是用来记录递归调用了几次，所以-1，其他的时候都是-1，只有在最后一次释放的时候调用xQueueGenericSend完成释放过程

释放成功返回`pdPASS`
失败返回`pdFALL`

**获取递归互斥信号量**
xQueueTakeMutexRecursive()注意点
判断是不是递归互斥信号量拥有者，如果是，说明是重复获取，那么简单,只需要uxRecursiveCallCount+1就可以了，如果是第一次获取，那么调用xQueueSemaphoreTake来获取信号量，记得还是要把uxRecursiveCallCount+1

##### 用Rust实现的困难

现提出几个待解决的问题

1. 如何用Rust实现C中的宏定义，因为很多操作都是实际上用通用函数来操作，而API函数只是一个宏定义而已
   或许可以用trait
2. PRIVILEGED_FUNCTIONS背后与硬件交互的逻辑是什么，用Rust如何实现，或者是否需要实现
3. 如何用Rust实现互斥信息量的优先级继承
4. 由于Rust对指针的限制，比如要向队列里传递大数据的指针时，如何做到多个任务都能使用这份数据
5. 能否用Rust实现条件编译？或者用其他的方式实现
6. 如何用Rust实现阻塞
7. 在上述的函数中包含Static的是自己指定空间的，Rust如何实现
8. Rust中的中断服务函数如何实现
9. 在C中一个任务可能属于多个列表，并且可能在不同的列表之间转换，由于指针的灵活性很容易实现，Rust如何实现

补充：在我们使用的FreeRTOS版本中，现在很多API都已经汇总成了通用函数，我们在用Rust实现的时候，不一定需要完全按照原来的方式实现，可以吸取新版本的一些好的设计思想

## 技术依据

### 利用QEMU进行FreeRTOS仿真

为了测试改写系统的正确性，我们需要使用QEMU对硬件进行仿真。

#### 环境配置

##### STM32 ARM开发环境

我们选择工业界使用相对来说最为广泛的**ST系列**，我们因此也需要使用他们的系列开发环境。

开发和创建项目的软件选择为：`STM32CubeMX`

软件截图：

![Screenshot from 2019-04-08 20-13-45](https://github.com/OSH-2019/x-rust-freertos/blob/Chivier_HYQ/STMsimReport/Screenshot%20from%202019-04-08%2020-13-45.png)

我们要在该软件下生成STM32Project。

##### Eclipse

我们之所以选择Eclipse的原因在于其为我们提供完整的[GNU ARM plugin](http://gnuarmeclipse.livius.net/blog/) with [GNU ARM GCC](https://launchpad.net/gcc-arm-embedded) compiler。与此同时我们还可以使用其中的内嵌的qemu系列插件模拟各种硬件环境。

使用自带的插件管理系统，我们需要安装以下插件：

- Rust相关插件
- [GNU Tools for ARM Embedded Processors](http://launchpad.net/gcc-arm-embedded) (arm-none-eabi-*)
- CPT系列插件
- QEMU系列插件

##### 外部*GNU Arm Embedded Toolchain*

这里的Arm Embedded Toolchain和之前的Eclipse中的不太一样，需要单独放再我们设置的的特殊位置，因为这个我们之后可能会对他们**“痛下毒手”**进行一些奇怪的操作，所以为了保证安全性，我们还是需要单独下载至独立的文件目录之下。

[Download](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)

##### FreeRtos C-Simulation

这里是GitHub上对FreeRTOS等嵌入式系统的一个模拟[FreeRTOS-GCC-ARM926ejs](https://github.com/jkovacic/FreeRTOS-GCC-ARM926ejs)

`setenv.sh`脚本中已经写好了我们需要的预操作。当然，我们有可能会被qemu卡住。这里相对比较简单了。我们可以依赖我们的Ubuntu中的apt-get install指令偷偷懒。直接安装。

#### 项目转化

由于Eclipse的局限性，里面有部分不予以支持的部分，我们的ST项目再里面会有一些无论如何也无法通过编译的`error`此时我们需要借助**Extra Superpower**——[CubeMX2Makefile项目](https://github.com/baoshi/CubeMX2Makefile)

用python运行这个简单的小小项目就可以自动摆平我们程序项目中所有的问题了。

----

#### 交叉编译

##### 项目生成后模拟

我们利用FreeRTOS模拟项目和我们已经生成好的bin文件，执行：

```bash
qemu-system-arm -M versatilepb -nographic -m 128 -kernel image.bin
```

即可以实现模拟

##### Embedded Rust编译方法

我们需要单独构建libcore，使用的libcore与编译器版本匹配很重要。

```bash
$ rustc -v --version
rustc 1.0.0-nightly (8903c21d6 2015-01-15 22:42:58 +0000)
binary: rustc
commit-hash: 8903c21d618fd25dca61d9bb668c5299d21feac9
commit-date: 2015-01-15 22:42:58 +0000
host: x86_64-apple-darwin
release: 1.0.0-nightly
```

紧接着执行：

```bash
mkdir libcore-thumbv7m
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g rust/src/libcore/lib.rs --out-dir libcore-thumbv7m
```

现在我们已经做好了交叉编译准备了

我们需要借助Rust中的Xargo以实现交叉编译，记得再Cargo.toml中加上他们

这里有一个简单的执行在ST系列开发板的模板：

```rust
#![feature(no_std)]
#![feature(core)]
#![no_std]
#![crate_type="staticlib"]


// **************************************
// These are here just to make the linker happy
// These functions are just used for critical error handling so for now we just loop forever
// For more information see: https://github.com/rust-lang/rust/blob/master/src/doc/trpl/unsafe.md

#![feature(lang_items)]

extern crate core;

#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"] extern fn eh_personality() {}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
  loop { }
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> () {
  loop {}
}

// **************************************
// **************************************

// And now we can write some Rust!


#[no_mangle]
pub fn main() -> () {
  
  // Do stuff here
  loop {}

}
```

编译可以使用：

```bash
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g --emit obj -L libcore-thumbv7m  -o my_rust_file.o my_rust_file.rs
```

这样就可以生成.o文件了

#### 转换至Rust内核

我们只需要慢慢修改[**FreeRTOS-GCC-ARM926ejs**](https://github.com/jkovacic/FreeRTOS-GCC-ARM926ejs)项目里面的Makefile部分即可

同时我们也需要将我们用C/C++生成的.o文件一一置换为我们用Rust制作的部分

值得注意的是我们需要将rust default模式设置成为nightly，否则feature系列的宏不能正常使用

----

###一个尝试-任务链表的Rust实现

FreeRTOS的任务管理是基于一个双向链表进行的，在原始实现中，它的结构如下：

![FreeRTOS Ready List](http://aosabook.org/images/freertos/freertos-figures-full-ready-list.png)

这份文档只针对通常意义下的双向链表的实现，要实现`FreeRTOS`中的`list.c`，只需要修改几个数据域即可。

文件的目录结构如下：

```
src
|- main.rs
|- lib.rs
|- list.rs
```

其中`lib.rs`内容如下：

```rust
pub mod list;
```

下文的代码全部写在`list.rs`中。

----

#### 设置数据结构

我们设置的链表带有一个头结点，然后是相应的子节点，它的结构可以声明如下：

```rust
use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
```

在这里，使用了`RefCell`类型，是因为待会会使用到它提供的`borrow`和`borrow_mut`方法。而`Rc`的使用是为了共享变量的`ownership`。

----

#### List 方法实现

首先，最先实现的就是`new`方法了。该方法主要起一个初始化的工作。它的的实现如下：

```rust
impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }
}
```

接下来实现`push_front`方法。实现细节如下：

```rust
pub fn push_front(&mut self, elem: T) {
    let new_head = Node::new(elem);
    // Takes the value out of the option, leaving a None in its place.
    match self.head.take() {    
        Some(old_head) => {
            // borrow_mut 作用于RefCell，获取里面的借用并且将其转型为mut
            old_head.borrow_mut().prev = Some(new_head.clone());
            new_head.borrow_mut().next = Some(old_head);
            self.head = Some(new_head);
        }
        None => {
            self.tail = Some(new_head.clone());
            self.head = Some(new_head);
        }
    }
}
```

接着就是实现`pop_front`方法。具体如下：

```rust
pub fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|old_head| {
        match old_head.borrow_mut().next.take() {
            Some(new_head) => {
                new_head.borrow_mut().prev.take();
                self.head = Some(new_head);
            }
            None => {
                self.tail.take();
            }
        }
        // all we want to do is to get the elem
        // Option<Rc<RefCell<Node<T>>>>  ----->   Node.elem
        Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
    })
}
```

然后就是实现`peek_front`方法：

```rust
pub fn peek_front(&self) -> Option<Ref<T>> {
    // Returns None if the pointer is null, 
    // or else returns a reference to the value wrapped in Some.
    self.head.as_ref().map(|node| {
        Ref::map(node.borrow(), |node| &node.elem)
    })
}
```

内存的释放：

```rust
impl<T> Drop for List<T> {
    // This will decrement the strong reference count. 
    // If the strong reference count reaches zero 
    // then the only other references (if any) are Weak, so we drop the inner value.
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
```

其他的相关方法附在[最终代码](https://github.com/OSH-2019/x-rust-freertos/blob/dev/双向链表的实现.md)中。

----

#### 遇到的问题

- 在尝试编写 `FreeRTOS`中`list.c`的过程中，我们发现`List`和`ListItem`之间似乎存在着相互的引用，这意味着两者的生命周期必须一样，这似乎有些矛盾，因为`List`的生命周期显然要比`ListItem`的生命周期长。我们的初步想法是重新构建`ListItem`中的某些`成员`，让其不直接指向`List`。这一部分待定。
- 在编写`rust`的程序时，最为头疼的可能就是其变量的**生命周期**了。这一个特性基本上是`rust`独有的，可能理解起来会有难度。暂时接触的比较少，需要进一步加强练习。
- 尽管我们已经仔细阅读过`官方文档`，但是仍然对其`标准库`不太熟悉，时常会有`invent the wheel`的行为，这增加了我们的负担。需要我们进一步了解和学习`rust`提供的标准库。

## 项目设计

### 使用的FreeRTOS版本和硬件平台

因为参考资料比较丰富，我们决定基于FreeRTOS v9版本，硬件平台是STM32，我们将在QEMU仿真器上测试我们的程序。

### 如何处理C语言指针？

这可能将是困扰我们最多的问题，在可行的情况下，我们将尽可能使用Rust内存安全的**引用**来处理指针。在万不得已的情况下，例如和C语言交互或是我们需要的功能和Rust语法特性矛盾时，我们将使用`Raw Pointer`，后者是Rust的`unsafe`功能。

### 如何处理C语言的宏定义？

FreeRTOS中的宏定义可以分为三类：

* 用于配置的宏定义
* 一些函数的封装
* 封装一些常见的代码块

对于第一种宏定义，我们可以使用`cfg`来进行配置并进行条件编译；对于第二种宏定义，我们将直接定义另一个函数来代替它；对于第三种宏定义，我们将尽可能使用函数将其替换。

### 怎样进行模块化设计？（对FreeRTOS的函数结构作出多大改变）

我们将主要参考[freertos.rs](https://github.com/hashmismatch/freertos.rs)的模块设计，使用更适合Rust风格的类型声明和函数定义方式。我们不忌惮对FreeRTOS的结构进行修改，但在必要的时候，我们将采用`unsafe`关键字来调用某些C语言函数。例如，我们将用Rust封装作为Rust和硬件接口的`portmacro.h`和负责内存分配的`heap.h`。

### 初期计划

我们决定采用自底向上的方式改写FreeRTOS。在初期，我们将首先封装FreeRTOS的硬件抽象，并编写Task和Queue的相关函数框架（声明），同时编写底层数据结构——queue和list。通过测试后，我们将实现任务管理和信号量的相关函数。

## 创新点

我们的工作是一个偏向工程的项目，因此我们在理论上没有多少创新，我们的创新集中在做到了前人没有完成的事情。在我们之前，有人用Rust封装了FreeRTOS的API，有人试图用Rust重写FreeRTOS但没有写完。如果我们能用Rust实现完整的FreeRTOS，就是实现了创新。

## 参考文献

* Real Time Engineers ltd.《FreeRTOS源码详解与应用开发》.
* Joseph Yiu.《ARM Cortex - M3权威指南》
* Jean J. Labrosse.嵌入式实时操作系统.
* [FreeRTOS - The architecture of open source applications](http://aosabook.org/en/freertos.html)
* [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#unsafe-rust)

- <https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html>
- <https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html>
- <https://doc.rust-lang.org/std/collections/struct.LinkedList.html>
- <https://github.com/alexchandel/rust-rtos>
- <https://github.com/beschaef/rtos>
- <https://rust-unofficial.github.io/too-many-lists/fourth-final.html>
- <https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute>