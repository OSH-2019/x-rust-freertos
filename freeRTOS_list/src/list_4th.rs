use std::rc::{Rc, Weak};
use std::cell::{Ref, RefMut, RefCell};

type BaseType = u16;  // unsighed short
type TickType = u16;  
type TCB = tskTCB;
type StackType = u16;

const portMAX_DELAY :TickType = 0xffff;

// umimplemented!
pub struct tskTCB {
    topOfStack: *mut StackType,         // Points to the location of the last item placed on the tasks stack.
    genericListItem: ListItem,     // The list that the state list item of a task is reference from denotes the state of that task (Ready, Blocked, Suspended ). 
    eventListItem: ListItem,       // Used to reference a task from an event list. 
    priority: BaseType,                 
    stack: *mut StackType,                   // // Points to the start of the stack.
    taskName: String,
}

pub struct List {
    numberOfItems: BaseType, 
    index: Option<Rc<RefCell<ListItem>>>,   
    listEnd: Option<Rc<RefCell<ListItem>>>,
}

pub struct ListItem {
    itemValue: TickType,
    next: Option<Rc<RefCell<ListItem>>>,
    previous: Option<Weak<RefCell<ListItem>>>,
    owner: Option<Rc<RefCell<TCB>>>,
    container: Option<Rc<RefCell<List>>>,
}


impl ListItem {
    pub fn new(itemValue: TickType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListItem {
            itemValue: itemValue,
            next: None,
            previous: None,
            owner: None,
            container: None,
        }))
    }
}

impl List {
    pub fn new() -> Self {
        let mut listItem = ListItem::new(portMAX_DELAY);
        listItem.borrow_mut().next = Some(Rc::clone(&listItem));
        listItem.borrow_mut().previous = Some(Rc::downgrade(&listItem));
        let list = List {
            numberOfItems: 0u16,
            index: Some(Rc::clone(&listItem)),
            listEnd: Some(Rc::clone(&listItem)),
        };
        list
    }
    pub fn insertEnd(&mut self, listItem: Rc<RefCell<ListItem>>) {
        
    }
    
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_new() {
        let mut list = List::new();
        let mut listItem = ListItem::new(23u16);
        assert_eq!(list.listEnd.is_some(), true);
        assert_eq!((&(list.listEnd)).unwrap().borrow().itemValue, portMAX_DELAY);
        (list.listEnd).unwrap().borrow_mut().itemValue = 111u16;
        assert_eq!((&(list.listEnd)).unwrap().borrow().itemValue, 111u16);
        
        
    }
}
