# Rust-FreeRTOS

This is our final project for [Operating Systems (H) 2019 Spring](https://osh-2019.github.io) in [USTC](http://ustc.edu.cn).

Our team members:

* Fan Jinhao ([fandahao17](https://github.com/fandahao17))
* Zuo Shun ([zsStrike](https://github.com/zsStrike))
* Ning Yuting ([nnnyt](https://github.com/nnnyt))
* Lei Siqi ([Roosevelt93](https://github.com/Roosevelt93))
* Huang Yeqi ([Chivier](https://github.com/Chivier))
* Zhang Fengming ([fming-Z](https://github.com/fming-Z))

## Our ambition

Improve the safety of FreeRTOS while maintaining its efficiency with the help of Rust programming language.

## We have already implemented…

* A unified **FFI** allowing the OS to seamlessly communicate with the C code in portable layer.
* A **doubly-linked list** using smart pointers. (It's really hard in Rust)
* A fully-functional **task scheduler** based on task priority.
* A fixed-length **queue**, with **semaphores and mutexes** built upon it.
* A set of **optionally-compiled functionalities** to help you DIY the kernel.