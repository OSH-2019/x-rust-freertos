# Android+

几个介绍Dalvik虚拟机和Android Runtime（简称ART）的视频：
  
[Google I/O 2008 - Dalvik Virtual Machine Internals](https://www.youtube.com/watch?v=ptjedOZEXPM)（比较基础，有用）
  
[Google I/O 2014 - The ART runtime](https://www.youtube.com/watch?v=EBlTzQsUoOw)(有点用）
  
[Deep Dive into the ART Runtime (Android Dev Summit '18)](https://www.youtube.com/watch?v=vU7Rhcl9x5o)（还没看，可能没啥用）
  
一本介绍安卓内部机理的书：[Embedded Android](http://www.staroceans.org/kernel-and-driver/%5BEmbedded.Android(2013.3)%5D.Karim.Yaghmour.pdf).

----
## Linux拥有而Android没有的功能（接口？）
尚不清楚

----
## 实现Android+的意义
尚不清楚

----
## 如何实现
安卓系统的结构（2013年时）如下图所示
  
![Android architecture](https://github.com/fandahao17/OS-Project/blob/master/Investigations/img/AndroidArchitecture.jpeg)
  
可以看出，安卓虚拟机（Dalvik/ART）的系统调用是通过利用JNI接口调用Linux的.so动态链接库实现的。
  
所以，我们需要**修改安卓虚拟机源码，使其调用Linux内核中它原本未调用的接口**，并向上层(*android.\**)提供封装，就可以使其具备完整的Linux功能了。
  
我认为，在这之后兴起的ART是对Dalvik的优化，本质上它们都是安卓虚拟机，都通过PNI进行系统调用，所以**我们的修改对两者都是有效的**。

----
## 挑战
1. 安卓的Linux内核似乎是[修改过的](https://source.android.com/devices/architecture/kernel/android-common)。
  
2. 安卓虚拟机是安卓系统的核心，对其进行修改很可能牵一发而动全身，工作量有可能出现上下层不兼容的问题。
  
3. 市面上系统介绍安卓内核的资料不多，而且多是针对旧版安卓的（如安卓2.\*，安卓4.\*）
