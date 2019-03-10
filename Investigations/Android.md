# Investigate -- Ray 3.10

-OS -> 2.Libraries&Runtime -> Application Framework -> Application

1.Linux2.6为内核&驱动程序
	
2.

Dalvik
	clean
	-common dex files\(libraries\)
	-application-specific dex files
	private dirty
	-application "live" dex structures
	-application heap
	shared dirty\(Zydote\)
	-library "live" dex structures
	-shared copy-on-write heap\(mostly not written\)
NO JIT\(just-in-time compiler\), leads to dirty memory, lots of native code\(??system provides libs, JNI available\)

## 深入解析
Java虚拟机标准字节码Bytecode  基于栈的虚拟机
Dalvik .dex 每一个进程是Linux里的一个Process 基于寄存器的虚拟机 基于硬件实现更大优化

### Android内核与Linux内核区别
1.Android Binder基于OpenBinder框架的驱动，提供Android平台的进程间通信
Linux系统上层通信主要是D-bus，使用消息总线的方式来进行IPC

2.电池管理 PM
针对嵌入式设备做了优化
利用锁和定时器切换系统状态，控制设备在不同状态下功耗

3.低内存管理器 Low Memory Killer
比Linux里OOM Out of Memery更灵活，可根据需要杀死进程释放需要的内存。
关键函数Lowmem_shrinker\(\)
一个模块在初始化时调用register_shrinke注册Lowmem_shrinker，会被vm在内存紧张的情况下调用。函数具体操作：寻找一个最合适的进程杀死，从而释放它占用的内存。

4.匿名共享内存 Ashmem
为进程间提供大块内存，同时为内核提供回收和管理这个内存的机制。
如果你个程序尝试访问内存Kernel释放的共享内存块，将会收到错误提示，然后重新分配内存并重载数据。

5.Android PMEM
向用户提供连续的物理内存空间，DSP等设备只能工作在连续物理内存。
驱动提供mmamp、open、release和ioctl等接口

6.Android Logger
抓取Android系统的各种日志，Linux没有

7.Android Alarm
提供了定时器用于把设备从睡眠状态唤醒，同时提供了即使设备在睡眠时也会运行的会时钟基准

8.USB Gadget驱动
基于标准Linux USB gadget驱动框架的设备驱动，Android的USB驱动是基于gadget框架的

9.Android Ram Console
为提供调试功能，Android允许将调试日志信息写入一个被称为RAM Console的设备里，它是一个基于RAM的Buffer

10.Android timed device
提供对设备定时控制功能，目前仅支持vibrator和LED设备\(这本书也挺老的了\)

11.Yaff2文件系统
MTD NAND Flash文件系统
Yaff2是一个快速稳定的应用于NAND和NOR Flash的跨平台嵌入式设备文件系统，同其他Flash文件系统比，使用更小内存保存其运行状态
垃圾回收简单快速，性能好
在大容量NAND Flash上性能尤为明显，适合大容量Flash储存