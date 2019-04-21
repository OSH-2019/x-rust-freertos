use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

type BaseType = u16;    // unsighed short
type TickType = u16;  
// type TCB = TskTCB;   // not declared
type StackType = u16;


#[derive(Debug, Copy, Clone)]
pub enum ListName {
    LIST0,
    LIST1,
    LIST2,
    LIST3,
    LIST4,
}

#[derive(Debug)]
pub struct ListItem {
    item_value: TickType,
    container: Option<ListName>,
    // container: Option<Rc<RefCell<&Vec<Rc<RefCell<ListItem>>>>>>,    // complicated
    // owner: Option<Rc<RefCell<TCB>>,
}

impl ListItem {
    pub fn new(item_value: TickType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListItem {
            item_value: item_value,
            container: None,
        }))
    }
}

macro_rules! list_insert_end {
    ($list:ident, $item:ident) => ({
        {
            
            $list.push(Rc::clone(&$item));
        }
    })
}

macro_rules! get_item_index {
    ($list:ident, $item:ident, eq) => ({
        {
            let index = $list.iter().position(|x| x.borrow().item_value == $item.borrow().item_value);
            index
        }
    });
    ($list:ident, $item:ident, gt) => ({
        {
            let index = $list.iter().position(|x| x.borrow().item_value > $item.borrow().item_value);
            index
        }
    });
}

macro_rules! list_insert {
    ($list:ident, $item:ident) => ({
        {
            let index = get_item_index!($list, $item, gt);
            match index {
                Some(index) => $list.insert(index, Rc::clone(&$item)),
                None => list_insert_end!($list, $item),
            }
        }
    })
}

macro_rules! set_list_item_container {
    ($item:ident, $Name:ident::$name:ident) => ({
        {
            $item.borrow_mut().container = Some($Name::$name);
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

macro_rules! list_remove {
    ($list:ident, $item:ident) => ({
        {
            let index = get_item_index!($list, $item, eq);
            match index {
                Some(index) => $list.remove(index),
                None => panic!("attemp to remove an item that actually not exsited"),
            }
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

macro_rules! list_initialise {
    ($list:ident) => ({
        {
            $list.clear();
        }
    })
}

macro_rules! is_contained_within {
    ($list:ident, $item:ident) => ({
        {
            let index = get_item_index!($list, $item, eq);
            match index {
                Some(index) => true,
                None => false,
            }

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

macro_rules! get_next {
    ($list:ident, $item:ident) => ({
        {
            let index = get_item_index!($list, $item, eq);
            match index {
                Some(index) => &$list[(index + 1) % current_list_length!($list)],
                None => panic!("item not found"),
            }    
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

macro_rules! get_list_item_value {
    ($item:ident) => ({
        {
            $item.borrow().item_value
        }
    })
}

macro_rules! get_item_value_of_head_entry {
    ($list:ident) => ({
        {
            if !list_is_empty!($list) {
                $list[0].borrow().item_value
            } else {
                panic!("no head entry");
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        let mut item = ListItem::new(100);
        let mut list1: Vec<Rc<RefCell<ListItem>>> = vec![];
        let mut list = vec![list1];
        let mut list1 = &mut list[0];
        list_insert_end!(list1, item);
        assert_eq!(list1[0].borrow().item_value, 100);
    }
    #[test]
    fn test_some_macros() {
        let mut item1 = ListItem::new(100);
        let mut item2 = ListItem::new(200);
        let mut item3 = ListItem::new(300);
        let mut item4 = ListItem::new(400);
        let mut list1: Vec<Rc<RefCell<ListItem>>> = vec![];
        let mut list = vec![list1];
        let mut list1 = &mut list[0];

        assert_eq!(list_is_empty!(list1), true);

        set_list_item_container!(item1, ListName::LIST0);
        assert_eq!(0, item1.borrow().container.unwrap() as i32);

        list_insert_end!(list1, item1);
        list_insert_end!(list1, item3);
        list_insert!(list1, item2);
        list_insert!(list1, item4);
        assert_eq!(list1[0].borrow().item_value, 100);
        set_list_item_container!(item1, ListName::LIST0);
        assert_eq!(list1[1].borrow().item_value, 200);
        set_list_item_container!(item1, ListName::LIST0);
        assert_eq!(list1[2].borrow().item_value, 300);
        set_list_item_container!(item1, ListName::LIST0);
        assert_eq!(list1[3].borrow().item_value, 400);

        list_remove!(list1, item3);
        assert_eq!(list1[0].borrow().item_value, 100);
        assert_eq!(list1[1].borrow().item_value, 200);
        assert_eq!(list1[2].borrow().item_value, 400);

        assert_eq!(get_item_value_of_head_entry!(list1), 100);
        assert_eq!(get_list_item_value!(item2), 200);

        let mut item = get_next!(list1, item4);
        assert_eq!(get_list_item_value!(item), 100);


    }
}