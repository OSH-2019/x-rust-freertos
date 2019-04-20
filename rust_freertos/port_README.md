# Port、Config、trace相关函数（宏）

到目前为止，Port、Config、trace相关函数和宏已经改写完毕并在Mac和Linux上基本通过了测试。因为`port.c`用到了Unix的pthread，所以在Windows上可能难以编译。

测试时，只需正常运行`cargo build`和`cargo test`即可。

因为我用到了[bindgen](https://github.com/alexcrichton/cc-rs#compile-time-requirements)和[cc](https://rust-lang.github.io/rust-bindgen/requirements.html)这两个crate，所以如果编译失败，很可能是它们的依赖项没有配置好。

----

关于我封装的内容和作出的修改：

1. 所有以`INCLUDE`开头的宏和值为bool类型的`config`开头的宏都定义在`Cargo.toml`中，可以用`#[cfg(…)]`实现条件编译；值为整型的以`config`开头的宏定义在`config.rs`中。
2. 所有`trace`开头的宏定义在了`trace.rs`中。
3. 在`port.rs`中定义了所有以`port`开头的宏；此外我对以port开头的**函数**名做了**修改**（如pvPortMalloc等），建议大家在写代码前读一下port.rs。
4. `bindings`模块中的代码是由bindgen在build时自动生成的。
4. 在`lib.rs`的测试代码中可以看到以上函数（宏）的简单用法。

