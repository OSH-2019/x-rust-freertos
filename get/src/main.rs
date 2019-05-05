#[macro_use]
extern crate lazy_static;
struct fan {
    name: String
}

impl fan {
    fn get_name(&self) -> &String {
        &self.name
    }
}

lazy_static! {
    static ref ian: fan = fan{name: String::from("fan")};
}

fn get_tcb_from_handle(task: Option<&fan>) -> &fan {
    match task {
        Some(t) => t,
        None => &ian
    }
}

fn get_name(name_handler: Option<&fan>) -> &String {
    // enter critical
    let name_handler: &fan = get_tcb_from_handle(name_handler);
    &name_handler.name
}

fn main() {
    println!("Hello, world!");
}
