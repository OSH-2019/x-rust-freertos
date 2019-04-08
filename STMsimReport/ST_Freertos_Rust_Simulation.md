## 环境配置

### STM32 ARM开发环境

我们选择工业界使用相对来说最为广泛的**ST系列**，我们因此也需要使用他们的系列开发环境。

开发和创建项目的软件选择为：STM32CubeMX

软件截图：

![Screenshot from 2019-04-08 20-13-45](/home/chivier_humber/Documents/OSH_git/STMsimReport/Screenshot from 2019-04-08 20-13-45.png)

我们要再该软件下生成STM32Project

### Eclipse

我们之所以选择Eclipse的原因在于其为我们提供完整的[GNU ARM plugin](http://gnuarmeclipse.livius.net/blog/) with [GNU ARM GCC](https://launchpad.net/gcc-arm-embedded) compiler。与此同时我们还可以使用其中的内嵌的qemu系列插件模拟各种硬件环境。

使用自带的插件管理系统，我们需要安装以下插件：

- Rust相关插件
- [GNU Tools for ARM Embedded Processors](http://launchpad.net/gcc-arm-embedded) (arm-none-eabi-*)
- CPT系列插件
- QEMU系列插件

### 外部*GNU Arm Embedded Toolchain*

这里的Arm Embedded Toolchain和之前的Eclipse中的不太一样，需要单独放再我们设置的的特殊位置，因为这个我们之后可能会对他们**“痛下毒手”**进行一些奇怪的操作，所以为了保证安全性，我们还是需要单独下载至独立的文件目录之下。

[Download](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)

### FreeRtos C-Simulation

这里是GitHub上对FreeRTOS等嵌入式系统的一个模拟[FreeRTOS-GCC-ARM926ejs](https://github.com/jkovacic/FreeRTOS-GCC-ARM926ejs)

`setenv.sh`脚本中已经写好了我们需要的预操作。当然，我们有可能会被qemu卡住。这里相对比较简单了。我们可以依赖我们的Ubuntu中的apt-get install指令偷偷懒。直接安装。

## 项目转化

由于Eclipse的局限性，里面有部分不予以支持的部分，我们的ST项目再里面会有一些无论如何也无法通过编译的`error`此时我们需要借助**Extra Superpower**——[CubeMX2Makefile项目](https://github.com/baoshi/CubeMX2Makefile)

用python运行这个简单的小小项目就可以自动摆平我们程序项目中所有的问题了。

## 交叉编译

### 项目生成后模拟

我们利用FreeRTOS模拟项目和我们已经生成好的bin文件，执行：

```bash
qemu-system-arm -M versatilepb -nographic -m 128 -kernel image.bin
```

即可以实现模拟

### Embedded Rust编译方法

我们需要单独构建libcore，使用的libcore与编译器版本匹配很重要。

```bash
$ rustc -v --version
rustc 1.0.0-nightly (8903c21d6 2015-01-15 22:42:58 +0000)
binary: rustc
commit-hash: 8903c21d618fd25dca61d9bb668c5299d21feac9
commit-date: 2015-01-15 22:42:58 +0000
host: x86_64-apple-darwin
release: 1.0.0-nightly
```

紧接着执行：

```bash
mkdir libcore-thumbv7m
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g rust/src/libcore/lib.rs --out-dir libcore-thumbv7m
```

现在我们已经做好了交叉编译准备了

我们需要借助Rust中的Xargo以实现交叉编译，记得再Cargo.toml中加上他们

这里有一个简单的执行在ST系列开发板的模板：

```bash
#![feature(no_std)]
#![feature(core)]
#![no_std]
#![crate_type="staticlib"]


// **************************************
// These are here just to make the linker happy
// These functions are just used for critical error handling so for now we just loop forever
// For more information see: https://github.com/rust-lang/rust/blob/master/src/doc/trpl/unsafe.md

#![feature(lang_items)]

extern crate core;

#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"] extern fn eh_personality() {}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
  loop { }
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> () {
  loop {}
}

// **************************************
// **************************************

// And now we can write some Rust!


#[no_mangle]
pub fn main() -> () {
  
  // Do stuff here
  loop {}

}
```

编译可以使用：

```bash
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g --emit obj -L libcore-thumbv7m  -o my_rust_file.o my_rust_file.rs
```

这样就可以生成.o文件了

## 转换至Rust内核

我们只需要慢慢修改[**FreeRTOS-GCC-ARM926ejs**](https://github.com/jkovacic/FreeRTOS-GCC-ARM926ejs)项目里面的Makefile部分即可

同事我们也需要将我们用C/C++生成的.o文件一一置换为我们用Rust制作的部分

值得注意的是我们需要将rust default模式设置成为nightly，否则feature系列的宏不能正常使用