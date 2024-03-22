#include <unistd.h>
#include <inttypes.h>
#include <pthread.h>
#include <semaphore.h>
#include <sys/time.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <locale.h>
#include <wchar.h>

#include <stdarg.h>
#include <list>
#include "rfidapi.h"


RFID_STATUS ConnectReader(RFID_HANDLE32 *readerHandle,wchar_t *hostName,int readerPort);
RFID_STATUS ReaderCapability(RFID_HANDLE32);
RFID_STATUS SingulationControl(RFID_HANDLE32,LPSINGULATION_CONTROL);
RFID_STATUS	RebootReader(RFID_HANDLE32);
RFID_STATUS ConfigureAntennaMode(RFID_HANDLE32);
RFID_STATUS ConfigureReadPointStatus(RFID_HANDLE32);
RFID_STATUS ConfigureGPO(RFID_HANDLE32);
RFID_STATUS ConfigureGPI(RFID_HANDLE32);
RFID_STATUS ConfigureAntenna(RFID_HANDLE32);
RFID_STATUS ConfigureRFMode(RFID_HANDLE32);
RFID_STATUS SimpleInventory(RFID_HANDLE32);
RFID_STATUS PeriodicInventory(RFID_HANDLE32);
RFID_STATUS AddPreFilter(RFID_HANDLE32);
RFID_STATUS RemovePrefilter(RFID_HANDLE32);
RFID_STATUS PerformSingleTagAccess(RFID_HANDLE32);
RFID_STATUS MultipleTagAccess(RFID_HANDLE32);
RFID_STATUS SingleTagAccess(RFID_HANDLE32);
RFID_STATUS PerformMultipleTagAccess(RFID_HANDLE32);
RFID_STATUS ReadAccessSingleTag(RFID_HANDLE32);
RFID_STATUS WriteAccessSingleTag(RFID_HANDLE32);
RFID_STATUS LockAccessSingleTag(RFID_HANDLE32);
RFID_STATUS KillAccessSingleTag(RFID_HANDLE32);
RFID_STATUS ReadAccessMultipleTags(RFID_HANDLE32);
RFID_STATUS WriteAccessMultipleTags(RFID_HANDLE32);
RFID_STATUS LockAccessMultipleTags(RFID_HANDLE32);
RFID_STATUS KillAccessMultipleTags(RFID_HANDLE32);

void * ProcessRfidEventsThread(void * pvarg);
void * AwaitRfidWin32EventsThread(void * pvarg);


void printTagDataWithResults(TAG_DATA *pTagData);
void CreateEventThread(RFID_HANDLE32 readerHandle);
void KillEventThread();
tm SYSTEMTIME2tm(SYSTEMTIME *s);
void GetLocalTime(SYSTEMTIME * pSystemTime);
void clean_stdin(void);

#ifdef linux
#define rfid_swprintf swprintf
#else
#define rfid_swprintf(x, y, z, ...) swprintf(x, z, __VA_ARGS__)
#endif