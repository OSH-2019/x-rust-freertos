# vim setup for Rust
本教程旨在帮大家安装Vundle，syntastic（同步代码错误检查），Rust.vim（Rust官方的vim支持）和vim-racer（代码自动补全工具）
  
----
## 0. 准备
1. 请检查自己的vim版本是否是vim8或以上版本
2. 请检查是否已安装了git

----
## 1. 安装并使用Vundle
Vim有很多插件管理器，类似Ubuntu的apt。我使用的是Vundle。

安装方法：在命令行输入：
```
git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim
```
----
## 2. 安装各个插件
将本文件夹中的.vimrc文件复制到``` ~/.vimrc ``` 中：

以上内容包括了Vundle默认需要的插件和我们要安装的几个插件的相关配置。复制完成后打开vim，输入命令```:VundleInstall```，会发现vim的窗口被分成了两半，左侧显示的是被安装的插件，窗口下方显示安装的状态。

Vundle的具体使用说明可以在vim中输入命令： ``` :h Vundle ```
  
因为它是在GitHub上下载这些插件，所以该过程会持续很长时间，所以我们打开另一个命令行终端，进行下一步。

----
## 安装Racer
Racer是Rust的自动补全工具。

Note:如果以下过程下载太慢，建议换用[ustc源](https://lug.ustc.edu.cn/wiki/mirrors/help/rust-crates).

1. 安装[nightly rust](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#appendix-g---how-rust-is-made-and-nightly-rust):在命令行输入：
```
rustup toolchain add nightly
```

2. 安装racer：在命令行输入：```cargo +nightly install racer```

----
## 完成
此时：

1. 在Rust代码中按下\<Ctrl-x\>\<Ctrl-o\>时，将弹出自动补全（学名omni-completion），并可以用\<Ctrl-N\>（上）和\<Ctrl-P\>（下）来上下选择，该功能在使用结构体的方法时格外有效；
2. 光标在某个函数上时，按下\<Ctrl-d\>将弹出该函数的文档；
3. 每次打开一个新的Rust文件或用```:w```保存时，vim的最左侧将标示代码出错的位置，最下方将出现报错信息。

----
## References and Further Reading
1. [Vundle](https://github.com/VundleVim/Vundle.vim#quick-start)，[Rust.vim](https://github.com/rust-lang/rust.vim)，[Racer](https://github.com/racer-rust/racer)，[vim-racer](https://github.com/racer-rust/vim-racer)，[syntastic](https://github.com/vim-syntastic/syntastic)的详细使用说明和配置选项请参考其官网（尽量不要看CSDN，因为其中很多内容可能是过时的）。
2. omni-completion需要手动触发、syntastic只有在保存文件时才会进行代码检查，[YouCompleteMe](https://github.com/Valloric/YouCompleteMe#rust-semantic-completion)提供了更强大的补全功能和边写代码边检查错误的功能，但该插件的配置十分困难。
3. [Ultisnips](https://github.com/SirVer/ultisnips)提供了自动插入代码块的功能（用起来挺爽的）。


