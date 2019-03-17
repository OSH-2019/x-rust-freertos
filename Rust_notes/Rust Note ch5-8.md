##**上次忘记提到的**：

1. 在函数中间（而非末尾）[返回](https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type)时，应当使用return。

2. [Tuple](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)的元素读取和利用tuple赋值。

3. https://play.rust-lang.org

    

##5. Structs

### 5.1 

1. 结构体中每个元素是一个field
2. Struct里的[ownership](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields)。

### 5.2

1. [trait](https://doc.rust-lang.org/book/ch10-02-traits.html)的概念
2. *{:?}*（一行）和*{:#?}*（多行）可以用于打印debug信息

 ### 5.3

1. > Having a method that takes ownership of the instance by using just `self` as the first parameter is rare; this technique is usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation.

2. Rust会[自动引用和解引用](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator): 上次提到的字符串不用\*，而整型需要用\*

3. Method: 是对instance使用的，相当于Java中的instance method

   Associated function：是对struct使用的，相当于Java中的static method

4. 实际编程中&号很容易丢！（所幸Rust会给出编译错误）

## 6. Enums

1. Anonymous struct?

2. 讨论：Option相对Null来说优点究竟是什么？(是一个wrapper)

3. 讨论：enum的意义是什么？（可以能出现的的情况是有限且可预测的，便于handle：*Pattern matching is exhaustive.*）（实际上实现了不同类型元素放在一起）

4. **if let**其实就是pattern match版的if else

5. > When enum values have data inside them, you can use `match` or `if let`to extract and use those values, depending on how many cases you need to handle.

## 7. Packages, Crates and Modules

### 7.1

binary crate 和 library crate

### 7.2

1. module类似于树形文件系统

2. [Pub](https://doc.rust-lang.org/book/ch07-02-modules-and-use-to-control-scope-and-privacy.html#modules-as-the-privacy-boundary)

3. [Use](https://doc.rust-lang.org/book/ch07-02-modules-and-use-to-control-scope-and-privacy.html#the-use-keyword-to-bring-paths-into-a-scope)类似于python中的import？其[习惯性用法](https://doc.rust-lang.org/book/ch07-02-modules-and-use-to-control-scope-and-privacy.html#idiomatic-use-paths-for-functions-vs-other-items)需要重点看

4. 在不同文件中使用module时的声明方式.

5. > Rust provides ways to organize your packages into crates, your crates into modules, and to refer to items defined in one module from another by specifying absolute or relative paths. 

## 8. Common collections.

### Vector

1. [copy和move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move) drop？
2. Creating: 在声明时，必须保证编译器知道data type

## Strings

略过

## HashMap

1. Zip?
2. 插入的方式很[有特点](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value) （传统的方式：get->manipulate->insert)
3. Pratice