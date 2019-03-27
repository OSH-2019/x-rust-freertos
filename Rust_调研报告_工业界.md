**Rust因其内存管理的安全性和高效性，在近两年内飞速发展，建立起了完善的开发人员社区，在工业界也大展拳脚，在多个领域催生出了安全高效的虚拟产品。**

# Rust in Optimization
**——npm堆栈管理**

npm是Node.js的包管理工具。得益于强大的功能，npm注册表成为世界上最大的软件注册表。但在规模成指数增长的同时，npm同样面临诸多挑战，其中一个是扩展CPU绑定服务(CPU-bound Service)产生的性能瓶颈：npm执行的大多数操作都是网络绑定的，JavaScript能够较好地支持该功能，但是在检查是否允许用户发布包的授权服务时，npm团队发现JS在执行一些CPU绑定任务时会造成性能下降。因此Node.js中这一服务需要重新实现，而npm团队希望借此机会利用Rust代码来提高性能。
 <img src="https://img-blog.csdnimg.cn/20190327183207254.png?watermark/2/text/aHR0cDovL2Jsb2cuY3Nkbi5uZXQvdTAxMDQxNjEwMQ==/font/5a6L5L2T/fontsize/400/fill/I0JBQkFCMA==/dissolve/70/gravity/SouthEast" width="70%" alt=""/>

C或C ++解决方案不再是一个合理的选择。这些语言需要内存管理方面的专业知识，以免出现安全问题，崩溃和内存泄漏（事实上，有专业知识的程序员也难免在百万行代码中犯一些这样的错误）。而Java由于需要将JVM和相关库部署在服务器上，这将产生一系列资源开销，不利于性能的提升。
Rust代码实现的优势：
- 内存安全
- 编译为独立且易于部署的二进制文件
- 总是优于JavaScript

它是一种可扩展且易于部署的解决方案，可以降低资源使用率而不会影响内存安全性。Cargo的依赖管理为系统编程领域带来了现代工具，它独立地为每个项目协调每个依赖项的版本，以便环境中的构建项目不会影响最终的可执行文件。npm的第一个Rust程序在一年半的应用过程中没有引起任何警报。

不过npm团队坦言：在Rust中重写服务确实需要比JavaScript版本和Go版本更长的时间，需要大约一周的时间来熟悉语言并实现程序。 Rust语言的设计预先加载了关于内存使用的决策，以确保内存的安全性。

# Rust in Analysis
**——Skylight代理的优化**

Tilde是一家位于波特兰的创业公司，他们开发的产品Skylight能够将Ruby on Rails框架中应用程序的性能数据转化为易于分析的信息，以便开发人员高效地监控和维护应用程序。Skylight代理在客户开发的rails应用程序中运行，以监控实际性能指标。但用户对这款软件的分析性能要求比较高：第一，他们对此代理使用的内存和CPU开销的容差非常低。由于这款代理用于帮助开发人员分析程序为什么变慢，因此很重要的一点是代理本身不会对应用程序的性能产生影响。第二，大多数Skylight的客户在Heroku上托管他们的应用程序，他们受到256或512 MB内存限制。

在先前的Ruby版本中，Skylight的内存使用量经常超过100MB，此时Heroku会报出内存超过限制的错误并重启代理。
  <img src="https://img-blog.csdnimg.cn/2019032718455486.png?watermark/2/text/aHR0cDovL2Jsb2cuY3Nkbi5uZXQvdTAxMDQxNjEwMQ==/font/5a6L5L2T/fontsize/400/fill/I0JBQkFCMA==/dissolve/70/gravity/SouthEast" width="80%" alt=""/>
而在使用Rus替换Ruby的数据结构并重写代理后，Skylight的内存占用能够保持在8 MB，比Ruby减少了92％，极大地减小了内存开销。

除此之外，Rust在编译时避免了数据竞争的问题。在一个Rust工程师希望使用新的数据结构来优化日志功能、防止消息重复时，Rust编译器报了错，表明这个数据结构中的某一部分不能很好地发挥系统中固有的并发性。修复后，应用程序就能够稳定实现预期的功能。Rust能够在早期开发过程中捕获并提醒程序员解决此类问题。

# Rust in Entertainment
**——大型游戏安全流畅的并发性**

目前许多游戏都是用C++编写的。无论是2D还是3D，大型游戏都有许多大型数据结构来保存游戏渲染图形所需的大量信息。游戏需要缓存这些数据以便快速访问，同时还需要经常更改数据以便响应。如果使用C＃或Java等垃圾收集语言（garbage-collection language）编写，那么游戏体验会受到游戏截图等停顿的明显影响。

Chucklefish团队使用Rust创建了一个利用多个核心而不会崩溃的游戏。他们仍编写了一些额外代码来确保组件和资源的锁定不会与其他系统发生死锁，Rust编译器会在出现这样的潜在风险时发出警告。开发人员不必在处理数据竞争的问题上小心翼翼，可以将之间花在处理游戏的运行逻辑上。

Rust还能够很好地处理跨平台差异。在开发Xbox、S4和Nintendo Switch等不同的主机适用版本时，Rust的包管理器工具Cargo为Chucklefish的构建处理了依赖库的问题。而在过去，添加一个库就需要花费数天的时间才能成功集成到所有支持的平台上。除此之外，Chucklefish开发人员认为，Rust中的类型和API需要更少的自定义包装代码，使用起来也更加自然。

#### 参考文献
[1]  《How Rust is Tilde’s Competitive Advantage》
[2]  《Community makes Rust an easy choice for npm》
[3]  《Chucklefish Taps Rust to Bring Safe Concurrency to Video Games》
[4]  《Building a Simple Webapp in Rust》
