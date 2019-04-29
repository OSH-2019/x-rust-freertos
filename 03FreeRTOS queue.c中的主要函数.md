# queue.c文件中的重要函数

## xQueueCreate

```c
QueueHandle_t xQueueCreate(
							  UBaseType_t uxQueueLength,
							  UBaseType_t uxItemSize
						  );
```

创建一个队列。

## xQueueCreateStatic

```c
 QueueHandle_t xQueueCreateStatic(
							  UBaseType_t uxQueueLength,
							  UBaseType_t uxItemSize,
							  uint8_t *pucQueueStorageBuffer,
							  StaticQueue_t *pxQueueBuffer
						  );
```

使用静态方式创建一个队列。

## xQueueSendToToFront

```c
 BaseType_t xQueueSendToToFront(
								   QueueHandle_t	xQueue,
								   const void		*pvItemToQueue,
								   TickType_t		xTicksToWait
							   );
```

将一个元素送到队首。**队列中的元素通过复制而不是引用**。

## xQueueSendToBack

```c
 BaseType_t xQueueSendToBack(
								   QueueHandle_t	xQueue,
								   const void		*pvItemToQueue,
								   TickType_t		xTicksToWait
							   );
```

将一个元素送到队尾。

## xQueueSend

```c
BaseType_t xQueueSend(
							  QueueHandle_t xQueue,
							  const void * pvItemToQueue,
							  TickType_t xTicksToWait
						 );
```

插入一个元素到队列中。

## xQueueOverwrite

```c
 BaseType_t xQueueOverwrite(
							  QueueHandle_t xQueue,
							  const void * pvItemToQueue
						 );
```

插入一个元素到队列中，如果队列已经满，则覆盖。

## xQueueGenericSend

```c
 BaseType_t xQueueGenericSend(
									QueueHandle_t xQueue,
									const void * pvItemToQueue,
									TickType_t xTicksToWait
									BaseType_t xCopyPosition
								);
```

和`xQueueSend`效果相同，但是不是推荐的API。推荐使用`xQueueSend(), xQueueSendToFront()` and `xQueueSendToBack()`这三个函数。

## xQueuePeek

```c
 BaseType_t xQueuePeek(
							 QueueHandle_t xQueue,
							 void * const pvBuffer,
							 TickType_t xTicksToWait
						 );
```

获取一个队列中中的元素但是不删除其在队列中的位置。

## xQueueReceive

```c
 BaseType_t xQueueReceive(
								 QueueHandle_t xQueue,
								 void *pvBuffer,
								 TickType_t xTicksToWait
							);
```

获取一个队列中的元素，成功访问后就删除该元素在队列中的位置。

## uxQueueMessagesWaiting

```c
UBaseType_t uxQueueMessagesWaiting( const QueueHandle_t xQueue );
```

返回储存在队列中信息的数目。

## uxQueueSpacesAvailable

```c
UBaseType_t uxQueueSpacesAvailable( const QueueHandle_t xQueue );
```

返回队列中的可用空间。

## vQueueDelete

```c
 BaseType_t xQueueSendToFrontFromISR(
										 QueueHandle_t xQueue,
										 const void *pvItemToQueue,
										 BaseType_t *pxHigherPriorityTaskWoken
									  );
```

删除队列。

> 上面的函数定义是为了在任务间传输数据，下面的函数用于`co-routines`（联合任务）

## xQueueCreateSet

```c
QueueSetHandle_t xQueueCreateSet( const UBaseType_t uxEventQueueLength ) PRIVILEGED_FUNCTION;
```

创建一个集合，以便机器能够让任务堵塞或者等待。

## xQueueAddToSet

```c
BaseType_t xQueueAddToSet( QueueSetMemberHandle_t xQueueOrSemaphore, QueueSetHandle_t xQueueSet ) PRIVILEGED_FUNCTION;
```

添加信号量到队列中。

## xQueueRemoveFromSet

```c
BaseType_t xQueueRemoveFromSet( QueueSetMemberHandle_t xQueueOrSemaphore, QueueSetHandle_t xQueueSet ) PRIVILEGED_FUNCTION;
```

从一个队列中移除一个信号量。

## xQueueSelectFromSet

```c
QueueSetMemberHandle_t xQueueSelectFromSet( QueueSetHandle_t xQueueSet, const TickType_t xTicksToWait ) PRIVILEGED_FUNCTION;
```

选择一个信号量。

> 非公共的API：

```c
void vQueueWaitForMessageRestricted( QueueHandle_t xQueue, TickType_t xTicksToWait, const BaseType_t xWaitIndefinitely ) PRIVILEGED_FUNCTION;
BaseType_t xQueueGenericReset( QueueHandle_t xQueue, BaseType_t xNewQueue ) PRIVILEGED_FUNCTION;
void vQueueSetQueueNumber( QueueHandle_t xQueue, UBaseType_t uxQueueNumber ) PRIVILEGED_FUNCTION;
UBaseType_t uxQueueGetQueueNumber( QueueHandle_t xQueue ) PRIVILEGED_FUNCTION;
uint8_t ucQueueGetQueueType( QueueHandle_t xQueue ) PRIVILEGED_FUNCTION;
```

