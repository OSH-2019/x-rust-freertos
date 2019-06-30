# Rust-FreeRTOS结题报告

TODO：小组成员介绍和项目简介。

## 项目背景

TODO：简要总结开题报告和可行性报告

## 项目设计

### 总体设计概览

TODO：樊金昊来写，大家补充

### 硬件接口——Port模块

TODO：樊金昊写

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