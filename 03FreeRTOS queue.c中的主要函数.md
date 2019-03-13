# queue.c文件中的重要函数

## xQueueCreate

创建一个队列。

## xQueueCreateStatic

使用静态方式创建一个队列。

## xQueueSendToToFront

将一个元素送到队首。

## xQueueSendToBack

将一个元素送到队尾。

## xQueueSend

插入一个元素到队列中。

## xQueueOverwrite

插入一个元素到队列中，如果队列已经满，则覆盖。

## xQueueGenericSend

和`xQueueSend`效果相同，但是是推荐的API。

## xQueuePeek

获取一个队列中中的元素但是不删除其在队列中的位置。

## xQueueReceive

获取一个队列中的元素，成功访问后就删除该元素在队列中的位置。

## uxQueueMessagesWaiting

返回储存在队列中信息的数目。

## uxQueueSpacesAvailable

返回队列中的可用空间。

## vQueueDelete

删除队列。

> 上面的函数定义是为了在任务间传输数据，下面的函数用于`co-routines`（联合任务）

TBD...

