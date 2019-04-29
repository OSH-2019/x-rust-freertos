use std::rc::{Rc, Weak};
use std::cell::{Ref, RefMut, RefCell};
use std::vec::*;
use std::cmp::PartialEq;

type BaseType = u16;  // unsighed short
type TickType = u16;  
// type TCB = TskTCB;
type StackType = u16;

const portMAX_DELAY :TickType = 0xffff;

// umimplemented!
// pub struct TskTCB {
//     topOfStack: *mut StackType,         // Points to the location of the last item placed on the tasks stack.
//     genericListItem: ListItem,     // The list that the state list item of a task is reference from denotes the state of that task (Ready, Blocked, Suspended ). 
//     eventListItem: ListItem,       // Used to reference a task from an event list. 
//     priority: BaseType,                 
//     stack: *mut StackType,                   // // Points to the start of the stack.
//     taskName: String,
// }

#[derive(Debug, Copy, Clone)]
pub enum ListName {
    LIST0,
    LIST1,
    LIST2,
    LIST3,
    LIST4,
}

#[derive(Debug, Copy, Clone)]
pub struct ListItem {
    item_value: TickType,
    container: Option<ListName>,
}

impl ListItem {
    pub fn new(item_value: TickType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListItem {
            item_value: item_value,
            container: None,
        }))
    }
}



macro_rules! set_list_item_container {
    ($container:ident::$o:ident own $item:ident) => ({
        {
            $item.borrow_mut().container = Some($container::$o);
        }
    })
}

macro_rules! get_list_item_container {
    ($item:ident) => ({
        {
            $item.borrow().container
        }
    })
}

macro_rules! set_list_item_value {
    ($item:ident, $value:expr) => ({
        {
            $item.borrow_mut().item_value = $value;
        }
    })
}

macro_rules! get_item_value_of_head_entry {
    ($list:ident) => ({
        {
            $list[0].borrow().item_value
        }
    })
}

macro_rules! get_next {
    ($list:ident, $item:ident) => ({
        {
            let index = $list.iter().position(|x| x.borrow().item_value == $item.borrow().item_value).unwrap();
            &$list[index + 1]
        }
    })
}

macro_rules! list_is_empty {
    ($list:ident) => ({
        {
            $list.is_empty()
        }
    })
}

macro_rules! current_list_length {
    ($list:ident) => ({
        {
            $list.len()
        }
    })
}

macro_rules! is_contained_within {
    ($list:ident, $item:ident) => ({
        {
            let index = $list.iter().position(|x| x.borrow().item_value == $item.borrow().item_value);
            match index {
                Some(index) => true,
                None => false,
            }

        }
    })
}

macro_rules! list_initialise {
    ($list:ident) => ({
        {
            $list.clear();
        }
    })
}

macro_rules! list_initialise_item {
    ($item:ident) => ({
        {
            $item.borrow_mut().container = None;
        }
    })
}

macro_rules! list_insert_end {
    ($list:ident, $item:ident) => ({
        {
            $list.push(Rc::clone(&$item));
        }
    })
}

macro_rules! list_remove {
    ($list:ident, $item:ident) => ({
        {
            let index = $list.iter().position(|x| x.borrow().item_value == $item.borrow().item_value).unwrap();
            $list.remove(index);
        }
    })
}

macro_rules! list_insert {
    ($list:ident, $item:ident) => ({
        {
            let index = $list.iter().position(|x| x.borrow().item_value >= $item.borrow().item_value).unwrap();
            $list.insert(index, Rc::clone(&$item));
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_enum() {
        assert_eq!(ListName::LIST0 as u32, 0);
    }

    #[test]
    fn test_list() {
        let mut list = vec![1, 2, 3];
        let sec = list[1];
        assert_eq!(sec, 2);
    }

    #[test]
    fn test_cycle() {
        let mut list: Vec<Rc<RefCell<ListItem>>> = vec![];
        assert_eq!(list_is_empty!(list), true);
        let mut item1 = ListItem::new(111);
        let mut item2 = ListItem::new(222);
        assert_eq!(item2.borrow().container.is_none(), true);
        let mut item3 = ListItem::new(333);
        item1.borrow_mut().container = Some(ListName::LIST0);
        set_list_item_container!(ListName::LIST2 own item2);
        // assert_eq!(item2.container, Some(ListName::LIST0));
        println!("{:?}", item2.borrow().container);
        println!("{:?}", item2.borrow().container);
        let con = get_list_item_container!(item1);
        assert_eq!(item1.borrow().item_value, 111);
        set_list_item_value!(item1, 1111);
        set_list_item_value!(item1, 2222);
        assert_eq!(item1.borrow().item_value, 2222);
        println!("{:?}", con);
        set_list_item_container!(ListName::LIST2 own item1);
        let con = get_list_item_container!(item1);
        println!("{:?}", con);

        list.push(Rc::clone(&item1));
        list.push(Rc::clone(&item2));
        list.push(Rc::clone(&item3));
        assert_eq!(get_item_value_of_head_entry!(list), 2222);
        let mut next_item = get_next!(list, item1);
        assert_eq!(next_item.borrow().item_value, 222);
        assert_eq!(list[0].borrow().item_value, 2222);
        assert_eq!(list[1].borrow().item_value, 222);
        assert_eq!(list[2].borrow().item_value, 333);
        // list_initialise!(list);
        // assert_eq!(list_is_empty!(list), true);
        assert_eq!(current_list_length!(list), 3);

        assert_eq!(is_contained_within!(list, item3), true);
        list_initialise_item!(item3);
        assert_eq!(item3.borrow().container.is_none(), true);
        let mut item4 = ListItem::new(444);
        list_insert_end!(list, item4);
        assert_eq!(current_list_length!(list), 4);
        list_remove!(list, item3);
        assert_eq!(list[0].borrow().item_value, 2222);
        assert_eq!(list[1].borrow().item_value, 222);
        assert_eq!(list[2].borrow().item_value, 444);
        let temp = Rc::clone(&list[0]);
        set_list_item_value!(temp, 100);

        let mut item5 = ListItem::new(111);
        list_insert!(list, item5);
        assert_eq!(list[0].borrow().item_value, 100);
        assert_eq!(list[1].borrow().item_value, 111);
        assert_eq!(list[2].borrow().item_value, 222);
        assert_eq!(list[3].borrow().item_value, 444);

    }

    #[test]
    fn test_macros() {
        
    }
}