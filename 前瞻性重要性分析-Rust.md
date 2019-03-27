# 前瞻性/重要性分析
## Rust
### Rust语言发展
Rust语言使用人数逐步增多，应用场景愈发广泛： 

1. 越来越多的用户体会到Rust语言的高效性：
![Rust高效性](https://blog.rust-lang.org/images/2018-11-RustSurvey/5-How_long_did_it_take_to_be_productive.png )  
超过40%的Rust用户在不到一个月的使用内即感受到Rust语言的高效性，超过70%的用户在不到一年内感受到Rust语言的高效性。   
2. Rust使用规模增大：
![Rust使用情况](https://blog.rust-lang.org/images/2018-11-RustSurvey/6-Size_of_summed_Rust_projects.png)   
2018年，调查显示，约23%的Rust用户使用Rust编写了超过10k行代码，超过70%用户使用Rust编写超过1k行代码。   
此外，随着对Rust项目的整体投资增加，Rust项目将趋向更大规模。Rust的中大型投资（分别超过10k和10万行代码）从2016年的8.9％增长到2017年的16％，2018年增长到23％。    
3. 使用Rust的目标平台趋于多样化：
![目标平台](https://blog.rust-lang.org/images/2018-11-RustSurvey/16-Platforms_targeting.png)     
Linux和Windows为Rust语言的主要目标平台。但在2017年，针对移动端、嵌入式设备的Rust开发大幅增长，交叉编译大大增加。2018年，针对WebAssembly的开发大幅增加，较2017年几乎翻了一番。   
4. 人们对Rust语言的兴趣与日俱增：   
![用户熟悉的编程语言第二名](https://blog.rust-lang.org/images/2018-11-RustSurvey/22-Programming_language_familiarity.png)   
自2016年开始，Rust连续三年成为Stack Overflow开发者调查中“最受欢迎的编程语言” 。同时，根据Rust官网进行的用户调查，2018年，Rust成为用户熟悉的编程语言第二名，仅次于python。 
  
5. 目前已使用Rust开发的项目：   
使用Rust开发的项目日益增加，目前已使用Rust开发的项目主要集中于浏览器、操作系统等.   
 * 浏览器：
     - 并行网页浏览器引擎Servo
     - 用于改进Firefox的Gecko Web浏览器引擎Quantum        
 * 操作系统：
     - Magic Pocket –  Dropbox的文件系统，为Diskotech PB级存储设备提供动力      
     - Redox – 微内核操作系统   
     - Stratis – 用于Fedora 28的文件系统   
     - Railcar – Oracle的一个容器的运行时    
     - Firecracker – 针对无服务器计算的安全、快速的微型虚拟机   

6. Rust目前的挑战：   
  Rust语言目前虽然稳步发展，吸引了更多的用户，应用场景更加广泛，但仍存在许多挑战。根据Rust官网2018年的调查结果，有以下地方仍待改进：需要更好的函数库、更好的IDE、更丰富的工具、改进编译时间等。   
7. Rust用于嵌入式设备开发的优势：  
   * 强大的静态分析：Rust将在在编译时强制执行引脚和外设配置，以保证程序非预期部分不会使用资源。   
   * 灵活的内存管理：可选择动态内存分配、全局分配和动态数据结构，或省略堆全部进行静态分配。
   * 无畏并发：Rust保证线程间不会意外共享状态，可以使用任何并行方式。   
   * 互通性：可将Rust集成到已编写的C代码库中，或使用现有SDK编写Rust应用。
   * 可移植性：编写一次库或驱动，即可应用于各类系统中。    

# 参考文献
Rust Survey 2018 Results  
https://blog.rust-lang.org/2018/11/27/Rust-survey-2018.html   
Rust 2017 Survey Results    
https://blog.rust-lang.org/2017/09/05/Rust-2017-Survey-Results.html    
wikipedia    
https://en.wikipedia.org/wiki/Rust_(programming_language)