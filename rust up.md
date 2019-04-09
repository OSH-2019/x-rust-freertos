# Rust Up

## raw pointer

```rust
use std::ptr;

let num: u32 = 233;
let const_raw_pointer = &num as *const u32;
let mut_raw_pointer = &num as *mut u32;
let raw_null = ptr::null() as *const u32;


unsafe {
    println!("Dereferenced data: {}", *const_raw_pointer);
    *mut_raw_pointer = 20;
    println!("Dereferenced data: {}", *mut_raw_pointer);
    
    println!("Data address: {:p}", &const_raw_pointer);
    println!("Data address: {:p}", &num);
    println!("Data address: {:p}", &raw_null);
}
```

