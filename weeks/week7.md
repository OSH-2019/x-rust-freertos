## 第七周安排

我们第七周的计划如下：

* 完全实现`list`的所有功能（左顺）
* 完成`task`的类型定义和相关函数原型声明（张俸铭、黄业琦）
* 实现`queue`的基本数据结构和与队列操作有关的相关函数（雷神、宁雨亭）
* 用Rust封装底层与硬件相关的函数（`portmacro.h`）和配置选项 （樊金昊）

组长已经在仓库中为以上四部分分别创建了分支（`list`，`task`，`queue`，`port`），在其中已经建立好了Cargo工程`rust_freertos`，大家在其中修改即可。

在编写完成且测试无误后，请大家在自己的分支提出`Pull request`，并邀请至少两个人对代码进行`review`（这主要是为了提高大家对不同模块的熟悉程度），根据意见修改后就可以merge进`master`分支了。

以上工作应当在**下周二**组会前完成，希望大家早日开工（特别是两人合作的，要尽快分工），周日碰头的时候我们讨论一下写代码过程中遇到的困难。



PS：Rust的宏定义也很强大，它也可以用来替代常见的代码块，[在这里](https://doc.rust-lang.org/rust-by-example/macros.html)有介绍。

PS：大家在声明函数时可以参考[freertos.rs](https://github.com/hashmismatch/freertos.rs/tree/master/src)。

## 代码风格

我们的项目涉及团队合作，代码可读性很重要，所以我们需要良好的代码风格和充分的注释。

我们提交的代码应当通过[Clippy](https://github.com/rust-lang/rust-clippy)的检查并被[rust fmt](https://github.com/rust-lang/rustfmt)格式化。

此外，我们编写的的每一个结构体和函数都应当有[Documentation](https://doc.rust-lang.org/rust-by-example/meta/doc.html)。下面以一个简单的例子介绍注释的格式：（内容使用中英文皆可）

```rust
impl Task {
  /// 返回一个以`name`命名的task （介绍函数的功能）
  /// * Declared by: 函数声明的编写者
  /// * Implemented by: 函数的实现者
  /// * C implementation: task.c xxxx行 （对应C语言函数的位置，由声明者给出，若对原函数有修改，请在此简要说明）
  ///
  /// # Arguments （介绍参数）
  /// 
  /// * `name` - The task's name
  ///
  pub fn new(name: &str) -> Task {
    Task {
        name: name.to_string(),
    }
  }
}
```

大多数编辑器都支持代码块（snippet）补全功能，大家可以将以上注释作为一个代码块，声明函数时直接插入即可。