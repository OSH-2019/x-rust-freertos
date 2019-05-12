# 关于条件编译

之前我们在使用条件编译时遇到的问题较多，并且我今天也发现之前我们条件编译的用法存在问题，所以在此说明一下。

----

Cargo的条件编译配置是在Cargo.toml中的`[features]`部分中进行的，本示例的该部分如下所示：

```toml
[features]
default = ["include_1"]

include_1 = []
include_2 = []
```

在该部分中，我们定义了`include_1`和`include_2`两个feature，default部分代表编译时默认会启用的feature（还可以在编译时用命令行参数指定feature）。

关于以上内容，可以参见[Cargo文档](<https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section>)。

----

**这里是重点！！！**

根据StackOverflow上的[回答](<https://stackoverflow.com/questions/48970713/rust-is-not-honoring-a-custom-cfg-feature-flag-for-conditional-compilation>)，我们应当采用如下方式来进行类似C语言的#ifdef操作。

``` rust
#[cfg(feature = "include_1")]
println!("include_1 is defined.");
```

而**不再**采用原来的方式：

``` rust
// 这种方式不对
#[cfg(include_1)]
println!("include_1 is defined.");
```

----

当#ifdef修饰的是一个赋值语句或是一个块（形如`{…}`）时，

``` rust
#[cfg(feature = "include_1")]
a = String::from("This will cause an error");
```

采用以上方式会报错：

``` 
error[E0658]: attributes on expressions are experimental. (see issue #15701)
 --> src/main.rs:9:5
  |
9 |     #[cfg(feature = "include_1")]
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: removing an expression is not supported in this position
 --> src/main.rs:9:5
  |
9 |     #[cfg(feature = "include_1")]
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

其对应的issue #15701在[这里](https://github.com/rust-lang/rust/issues/15701)。

宁雨亭提供了以下替代方法解决该问题：

```rust
{
      // NOTE: 此时用的是#!而不再是#
      // #!修饰的是inner attribute，作用于其所在的整个块
      // #修饰的是outer attribue，仅作用于后面的语句
      #![cfg(feature = "include_1")]
      a = String::from("This is correct");
      println!("{}", a);
}
```

----

此外，应当注意的是，`feature = "include_1"`不代表仅启用了"include_1"这一个feature。只要"include_1"这个feature被启用了，上式就成立。

例如，如果我们修改`[features]`部分，使"include_1"和"include_2"同时被启用，

``` toml
[features]
default = ["include_1", "include_2"]

include_1 = []
include_2 = []
```

以下语句将会被编译：

``` rust
#[cfg(all(feature = "include_1", feature = "include_2"))]
println!("Both include_1 and include_2 is defined");
```

此时运行`src/main.rs`的结果为（我已将会出现error那部分代码注释掉）：

```
include_1 is defined.
This is correct
Both include_1 and include_2 is defined
```

