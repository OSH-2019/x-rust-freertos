# 02 list.c 中的主要函数

## vListInitialise

```c
void vListInitialise( List_t * const pxList );
```

列表的初始化。

## vListInitialiseItem

```c
void vListInitialiseItem( ListItem_t * const pxItem );
```

列表项的初始化。

## vListInsert

```c
void vListInsert( List_t * const pxList, 
                 ListItem_t * const pxNewListItem );
```

列表项插入。会按照列表项的`itemvalue`升序排列。

## vListInsertEnd

```c
void vListInsertEnd( List_t * const pxList,
                     ListItem_t * const pxNewListItem );
```

列表项插入到最后。

## uxListRemove

```c
UBaseType_t uxListRemove( ListItem_t * const pxItemToRemove );
```

从列表中移除一个列表项。

