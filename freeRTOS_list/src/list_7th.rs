// #![crate_name = "doc"]

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

/// this should be defined is port.rs
type BaseType = u16;    // unsighed short
type TickType = u16;  
type TCB = TskTCB;   // not declared
type StackType = u16;

/// thing now get better understood here!
/// suppose we have a list vec, we call it `list`.
/// now that we have two list, `lista` and `listb`.
/// we could push the `lista` and `listb` in the `list` given above, meaning that 
/// lista == list[0]
/// listb == list[1]
/// we now have three list item named item1, item2, item3, which are created by the method `ListItem::new`
/// every thim we want insert an item to a list, we should first call `set_list_item_container!` macro, for example:
/// `
/// //insert item1 to the lista, since the lista is the first value in list, the index should be 0 --> ListName::LIST0
/// set_list_item_container!(item1, ListName::LIST0);
/// insert_end!(lista, item1);
/// `
/// and in this way, we could easily find the **actural** container.
/// `
/// let mut index = get_list_item_container!(item1);
/// let mut container = match index {
///     Some(index) => {
///         let i = index as u32;   // ListName --> u32
///         &mut list[i]
///     },
///     None => {
///         panic!("no container found!");    
///     }
/// }
/// `
#[derive(Debug)]
pub struct TskTCB {
    topOfStack: *mut StackType,         // Points to the location of the last item placed on the tasks stack.
    genericListItem: ListItem,     // The list that the state list item of a task is reference from denotes the state of that task (Ready, Blocked, Suspended ). 
    eventListItem: ListItem,       // Used to reference a task from an event list. 
    priority: BaseType,                 
    stack: *mut StackType,                   // // Points to the start of the stack.
    taskName: String,
}

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
    // container: Option<Rc<RefCell<&Vec<Rc<RefCell<ListItem>>>>>>,    // complicated, deprecateed
    owner: Option<Rc<RefCell<TCB>>>,      // the TCB declaration is not defined
}

impl ListItem {
    /// # Description
    /// * constructor
    /// # Argument
    /// * `item_value` - item_value
    /// # Return
    /// * Rc<RefCell<Self>>
    pub fn new(item_value: TickType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListItem {
            item_value: item_value,
            container: None,
            owner: None,
        }))
    }
}
/// # Description
/// set list item's owner
/// # Arguments
/// $item: Rc<RefCell<ListItem>>
/// $owner: Rc<RefCell<TCB>>
/// #Return
/// Nothing
macro_rules! set_list_item_owner {
    ($item:ident, $owner:ident) => ({
        $item.borrow_mut().owner = Some(Rc::clone(&$owner));
    });
}

/// # Description
/// get list item's owner
/// # Arguments
/// $ietm: Rc<Refcell<ListItem>>
/// #Return
/// Option<Rc<RefCell<TCB>>>
macro_rules! get_list_item_owner {
    ($item:ident) => ({
        match $item.borrow().owner {
            Some(owner) => {
                Rc::clone(&owner)
            },
            None => {
                None
            }
        }
    });
}

/// # Description
/// get owner of next entry
/// # Arguments
/// $list: List
/// $item: Rc<RefCell<ListItem>>
/// #Return
/// Option<Rc<RefCell<TCB>>>
macro_rules! get_owner_of_next_entry {
    ($list:ident, $item:ident) => ({
        let index = get_item_index!($list, $item, eq);
        match index {
            Some(index) => {
                match $list[(index + 1) % current_list_length!($list)].borrow().owner {
                    Some(owner) => Rc::clone(&owner),
                    None => None,
                }
            },
            None => None,
        }  
    });
}

/// # Description
/// get owner of head entry
/// # Arguments
/// $list: List
/// #Return
/// Option<Rc<RefCell<TCB>>>
macro_rules! get_owner_of_head_entry {
    ($list:ident) => ({
        if current_list_length!($list) == 0 {
            None
        }else{
            Some(Rc::clone(&$list[0]))
        }  
    });
}
/// # Description
/// * append $item to the $list
/// # Argument
/// * `$list` - list
/// * `$item` - list item
/// # Return
/// * Nothing

macro_rules! list_insert_end {
    ($list:ident, $item:ident) => ({
        {
            
            $list.push(Rc::clone(&$item));
        }
    })
}

/// # Description
/// * get $item's index in $list, based on the given oprator
/// # Argument
/// * `$list` - list
/// * `$item` - list item
/// # Return
/// * Option<u32>
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

/// # Description
/// * insert $item in $list in descending order 
/// # Argument
/// * `$list` - list
/// * `$item` - list item
/// # Return
/// * Nothing
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

/// # Description
/// * set list item container
/// # Argument
/// * `$item` - list item
/// * `$Name::$name` - ListName
/// # Return
/// * Nothing
macro_rules! set_list_item_container {
    ($item:ident, $Name:ident::$name:ident) => ({
        {
            $item.borrow_mut().container = Some($Name::$name);
        }
    })
}

/// # Description
/// * get list item container
/// # Argument
/// * `$item` - list item
/// # Return
/// * Option<ListName>
macro_rules! get_list_item_container {
    ($item:ident) => ({
        {
            $item.borrow().container
        }
    })
}

/// # Description
/// * remove the $item in $list, panic if the $item not in $list
/// # Argument
/// * `$list` - list
/// * `$item` - list item
/// # Return
/// * Nothing
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

/// # Description
/// * set $item's container None
/// # Argument
/// * `$item` - list item
/// # Return
/// * Nothing
macro_rules! list_initialise_item {
    ($item:ident) => ({
        {
            $item.borrow_mut().container = None;
        }
    })
}

/// # Description
/// * make $list empty with no item in it
/// # Argument
/// * `$list` - list
/// # Return
/// * Nothing
macro_rules! list_initialise {
    ($list:ident) => ({
        {
            $list.clear();
        }
    })
}

/// # Description
/// * return true if $list contain $item, otherwise false
/// # Argument
/// * `$list` - list
/// * `$item` - list item
/// # Return
/// * is_contained: bool
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

/// # Description
/// * return true if $list is empty, otherwise false
/// # Argument
/// * `$list` - list
/// # Return
/// * is_empty: bool
macro_rules! list_is_empty {
    ($list:ident) => ({
        {
            $list.is_empty()
        }
    })
}

/// # Description
/// * get current list length
/// # Argument
/// * `$list` - list
/// # Return
/// * len: u32
macro_rules! current_list_length {
    ($list:ident) => ({
        {
            $list.len()
        }
    })
}

/// # Description
/// * get the next item of $list, and the current item is $item. If $item is not in $list, panic!.
/// # Argument
/// * `$list` - list
/// * `$item` - list item
/// # Return
/// * item: &Rc<RefCell<ListItem>>
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

/// # Description
/// * set list item value
/// # Argument
/// * `$item` - list item
/// * `$value` - item_value
/// # Return
/// * No return
macro_rules! set_list_item_value {
    ($item:ident, $value:expr) => ({
        {
            $item.borrow_mut().item_value = $value;
        }
    })
}

/// # Description
/// * get list item value
/// # Argument
/// * `$item` - list item
/// # Return
/// * item_value: TickType
macro_rules! get_list_item_value {
    ($item:ident) => ({
        {
            $item.borrow().item_value
        }
    })
}

/// # Description
/// * get item_value of the head_entry of the list. If the list is empty, panic!
/// # Argument
/// * `$list` - list
/// # Return
/// * item_value: TickType 
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