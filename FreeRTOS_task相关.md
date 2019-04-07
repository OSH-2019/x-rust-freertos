# FreeRTOS task相关
### FreeRTOS中的预定义
- config.h中的define语句为相关配置参数（类似树莓派裁剪的menuconfig）
- INCLUDE开始的宏：函数的使能、除能。
当INCLUDE_func值为1时，表明func函数（及其相关函数）API接口可以使用
（task.c中多处条件编译相关）
- config开始的宏：表明一些参数状态
如：configASSERT(x)会在内核中的关键点被调用，如果参数为0说明有错误发生；configCHECK_FOR_STACK_OVERFLOW会指示当前状态堆栈是否溢出

### FreeRTOS任务运行和调度流程
- main函数初始化硬件外设
- 调用xTaskCreate()创建开始任务（执行一次，创建其他应用任务的信号量、队列等内核对象
- 开启任务调度器
- 开始任务会完成其他任务的创建(start_task函数)

### FreeRTOS任务系统
FreeRTOS中的每个任务都有自己的运行环境，不依赖于系统中的其他任务或者调度器。RTOS调度器则需要确保一个任务开始执行时其上下文环境和任务上一次退出时相同（通过堆栈的存取操作）。
FreeRTOS中的任务存在4个状态：
- 运行态：任务正在运行
- 就绪态：任务正在排队等待，一旦有资源可以进入运行态
- 阻塞态：任务正在等待某个外部事件（有延时限制）
- 挂起态：任务不能进入运行态，除非被Resume退出挂起态
![在这里插入图片描述](https://img-blog.csdnimg.cn/20190407151610903.png?)

每个任务都会分配一个相应的优先级，取值范围从0~(configMAX_PRIORITIES-1)，RTOS调度器会在就绪队列中根据优先级进行相应的调度

FreeRTOS通过TCB（任务控制块）这个结构体集合一个任务的属性。在任务被创建时，会同时给它分配一个tskTaskControlBlock。任务堆栈与TCB受函数的统一调配，保证任务调度的正常。
FreeRTOS同样支持时间片轮转的调度方式。


### 任务相关API函数
- 创建与删除：xTaskCreate(),xTaskDelete()
新创建的任务默认为就绪态，创建任务的主要工作是申请堆栈空间、传递任务的参数、设置优先级、返回任务状态（追踪信息等）。
删除任务时，xTaskDelete()会根据传递的任务句柄进行删除。由内核分配给任务的内存会在此过程中释放掉，但用户分配的任务内存需要用户自行释放。
- 任务挂起和恢复：vTaskSuspend(),vTaskResume()
vTaskSuspend()将任务设置为挂起态，该任务将不会进入运行态，直到调用vTaskResume()。后者会将任务设置为就绪态。
这两个函数都是通过任务句柄来修改任务状态的参数值来实现的。
- SVC中断服务函数、空闲任务函数
- 任务相关API函数举例
uxTaskPriorityGet() 获取任务的优先级
vTaskGetInfo() 获取任务状态
xTaskGetHandle() 获取任务句柄
xTaskGetTickCount() 获取调度器计数器的值

```
在《FreeRTOS源码详解与应用开发》的第8、9章详细介绍了FreeRTOS任务相关函数的代码实现
在《ARM Cortex - M3权威指南》中有对Cortex-M处理器的架构介绍（包括寄存器用途、指令等）
```

