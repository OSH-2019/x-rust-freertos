// we need null *raw pointer*, ptr::null()
use std::ptr;
use std::ptr::null;
use std::cmp::PartialEq;

type BaseType = u16;  // unsighed short
type TickType = u16;  
type TCB = tskTCB;
type StackType = u16;

const portMAX_DELAY :TickType = 0xffff;

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
    index: *mut ListItem,   // raw pointer
    listEnd: *mut ListItem,
}

pub struct ListItem {
    itemValue: TickType,
    next: *mut ListItem,
    previous: *mut ListItem,
    owner: *mut TCB,
    container: *mut List,
}


impl List {
    // void vListInitialise( List_t * const pxList )
    pub fn new() -> Self {
        let mut listEnd = ListItem::new(portMAX_DELAY);
        listEnd.next = &mut listEnd;
        listEnd.previous = &mut listEnd;
        let mut list = List {
            numberOfItems: 0, 
            index: ptr::null_mut(),   // raw pointer
            listEnd: &mut listEnd as *mut ListItem,
        };
        list.index = list.listEnd;
        list
    }
    // void vListInsertEnd( List_t * const pxList, ListItem_t * const pxNewListItem )
    pub fn insertEnd(&mut self, item: &mut ListItem) {
        let index: *mut ListItem = self.index;
        item.next = index;
        unsafe {
            item.previous = (*index).previous;
            (*(*index).previous).next = item as *mut ListItem;
            (*index).previous = item as *mut ListItem;   
        }
        item.container = self;
        self.numberOfItems += 1; 
        // println!("insertEnd----------------");
        // unsafe {
        //     println!("value: {}", (*self.index).itemValue);
        //     println!("value: {}", (*(*self.index).next).itemValue);
        //     println!("value: {}", (*(*(*self.index).next).next).itemValue);
        //     println!("value: {}", (*(*(*(*self.index).next).next).next).itemValue);
        // }

    }
    
    // 
    pub fn insert(&mut self, item: &mut ListItem) {
        let mut iter: *mut ListItem = ptr::null_mut();
        let value = item.itemValue;
        unsafe{
            // assert_eq!(((*self.listEnd).itemValue), 0xffff);
            println!("value was {}", (*self.listEnd).itemValue);
        }
        unsafe {
            println!("11111");
            if value == portMAX_DELAY {
                iter =  (*self.listEnd).previous;
            } else {
                println!("11111");
                iter = self.listEnd;
                println!("11111");
                // while (*(*iter).next).itemValue <= value {
                    println!("{}-----", (*iter).itemValue);
                    // iter = (*iter).next;
                // }
            }
            // item.next = (*iter).next;
            // (*(*iter).next).previous = item;
            // item.previous = iter;
            // (*iter).next = item;
            // item.container = self;
        }
        self.numberOfItems += 1;
    }
    

}

impl ListItem {
    pub fn new(value: TickType) -> Self {
        ListItem {
            itemValue: value,
            next: ptr::null_mut(),
            previous: ptr::null_mut(),
            owner: ptr::null_mut(),
            container: ptr::null_mut(),
        }
    }
    // void vListInitialiseItem( ListItem_t * const pxItem )
    pub fn init(&mut self) {
        self.container = ptr::null_mut();
    }
    // UBaseType_t uxListRemove( ListItem_t * const pxItemToRemove )
    pub fn remove(&mut self) -> BaseType{
        let mut this: *mut ListItem = self as *mut ListItem;
        unsafe {
            let mut list: *mut List = (*this).container;
            (*(*this).next).previous = (*this).previous;
            (*(*this).previous).next = (*this).next;
            if (*list).index == this {
                (*list).index = (*this).previous;
            } else {
                println!("mtCOVERAGE_TEST_MARKER()");
            }
            (*list).numberOfItems = (*list).numberOfItems - 1;   
            // (*this).container = ptr::null_mut(); 
            let mut iter: *mut ListItem = (*list).listEnd;
            iter = (*iter).previous;
            println!("{}----", (*iter).itemValue);
            iter = (*iter).previous;
            println!("{}----", (*iter).itemValue);
            iter = (*iter).previous;
            println!("{}----", (*iter).itemValue);
            iter = (*iter).previous;
            println!("{}----", (*iter).itemValue);
            (*list).numberOfItems
        }
        // unsafe {
        //     let mut list: *mut List = (*this).container;
        //     let mut iter: *mut ListItem = (*list).iter;
            
        // }
        
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_one() {
        let mut item1: ListItem = ListItem::new(111u16);
        let mut item2: ListItem = ListItem::new(222u16);
        let mut item3: ListItem = ListItem::new(333u16);
        item1.init();
        item2.init();
        item3.init();
        let mut list: List = List::new();
        list.insertEnd(&mut item1);
        list.insertEnd(&mut item2);
        list.insertEnd(&mut item3);
        assert_eq!(list.numberOfItems, 3);
        let mut i = 0u32;
        let mut iter: *mut ListItem  = list.listEnd;
        unsafe {
            assert_eq!((*iter).itemValue, portMAX_DELAY);
            iter = (*iter).next;
            assert_eq!((*iter).itemValue, 111u16);
            iter = (*iter).next;
            assert_eq!((*iter).itemValue, 222u16);
            iter = (*iter).next;
            assert_eq!((*iter).itemValue, 333u16);
            iter = (*iter).next;
            assert_eq!((*iter).itemValue, portMAX_DELAY);
            iter = (*iter).next;
            assert_eq!((*iter).itemValue, 111u16);
            iter = (*iter).next;
            assert_eq!((*iter).itemValue, 222u16);
            iter = (*iter).next;
            assert_eq!((*iter).itemValue, 333u16);

            iter = (*iter).previous;
            assert_eq!((*iter).itemValue, 222u16);
            iter = (*iter).previous;
            assert_eq!((*iter).itemValue, 111u16);
            iter = (*iter).previous;
            assert_eq!((*iter).itemValue, portMAX_DELAY);
            iter = (*iter).previous;
            assert_eq!((*iter).itemValue, 333u16);
            iter = (*iter).previous;
            assert_eq!((*iter).itemValue, 222u16);
            iter = (*iter).previous;
            assert_eq!((*iter).itemValue, 111u16);
        }

        let mut item4: ListItem = ListItem::new(110u16);
        let mut item5: ListItem = ListItem::new(223u16);
        let mut item6: ListItem = ListItem::new(34u16);
        list.insert(&mut item4);
        list.insert(&mut item5);
        list.insert(&mut item6);
        // let size: BaseType = item1.remove();
        // assert_eq!(size, 2);
        // let mut iter: *mut ListItem  = list.listEnd;
        // unsafe {

            // iter = (*iter).previous;
            // println!("{}", (*iter).itemValue);

        //     // assert_eq!((*iter).itemValue, 222u16);
            // println!("{}", (*iter).itemValue);
            // iter = (*iter).previous;
        //     // assert_eq!((*iter).itemValue, 333u16);
            // println!("{}", (*iter).itemValue);
            // iter = (*iter).previous;
            // assert_eq!((*iter).itemValue, portMAX_DELAY);
        //     iter = (*iter).next;
            // assert_eq!((*iter).itemValue, 222u16);
        // }

        println!("test is over~~");
    }
}

