#include "FreeRTOSConfig.h"
#include "portmacro.h"
#include "portable.h"

#define pdTASK_CODE TaskFunction_t
#define pdFALSE ( ( BaseType_t ) 0 )
#define pdTRUE ( ( BaseType_t ) 1 )

typedef void * xTaskHandle;
xTaskHandle xTaskGetCurrentTaskHandle();
BaseType_t xTaskIncrementTick();
void vTaskSwitchContext();
void vTaskSuspendAll();
BaseType_t xTaskResumeAll();
