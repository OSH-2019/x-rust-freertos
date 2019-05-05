#[macro_use]
extern crate lazy_static;
// lazy_static的使用方式，参见https://github.com/rust-lang-nursery/lazy-static.rs
struct TCB {
    name: String
}

impl TCB {
    fn get_name(&self) -> &String {
        &self.name
    }
}

lazy_static! {
    static ref CurrentTCB: TCB = TCB{name: String::from("Fan")};
}

fn get_tcb_from_handle(task: Option<&TCB>) -> &TCB {
    match task {
        Some(t) => t,
        None => &CurrentTCB
    }
}

fn get_name(name_handler: Option<&TCB>) -> &String {
    // 用Option来实现TaskHandle
    let name_handler: &TCB = get_tcb_from_handle(name_handler);

    // One possible return value
    &name_handler.name;

    // Use method of TCB
    name_handler.get_name()
}

fn set_name(name_handler: TCB, new_name: String) -> TCB {
    // Set时，传入TCB本身而不传入引用，返回修改后的新TCB
    TCB {
        name: new_name
    }
}

fn main() {
    let mut tian = TCB{ name: String::from("Tian") };
    // 获取的是tian指向的TCB的名字
    println!("{}", get_name(Some(&tian)));
    // 获取的是CurrentTCB的名字
    println!("{}", get_name(None));

    // 使用赋值的方式进行修改
    tian = set_name(tian, String::from("Tiantian"));
    println!("{}", get_name(Some(&tian)));
}
