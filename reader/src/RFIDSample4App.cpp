#include "common.h"

bool g_bUseWin32EventHandling = false;

static wchar_t hostName[260];
static int readerPort = 0;
static SINGULATION_CONTROL singulationControl;
extern ANTENNA_INFO g_antennaInfo;

void InventoryFilterOption(RFID_HANDLE32 readerHandle);
void Createmenu(RFID_HANDLE32 readerHandle);
void ConfigurationMenu(RFID_HANDLE32 readerHandle);
void InventoryMenu(RFID_HANDLE32 readerHandle);
void AccessMenu(RFID_HANDLE32 readerHandle);

#ifdef linux
int main(int argc, char* argv[])
#else
int _tmain(int argc, wchar_t* argv[])
#endif
{
	if(argc == 1 || argc == 3)
	{
		if(argc == 1)
		{
			wcscpy(hostName, L"localhost");
			readerPort = 0;
		}
		else
		{
#ifdef linux
			g_bUseWin32EventHandling = false; // in lunux, just the callback mechanism is supported.
			char *stopChar;
			mbstowcs((wchar_t *)hostName, argv[1], MAX_PATH);
			readerPort = strtol(argv[2], &stopChar, 10);
#else
			wchar_t *stopChar;
			wcscpy((wchar_t *)hostName, argv[1]);
			readerPort = wcstol(argv[2], &stopChar, 10);
			g_bUseWin32EventHandling = true; // you can set it to true or false in windows, it supports both.
#endif
		}	
	}
	else
	{
		wprintf(L"\nEnter either 0 or 2 arguments\nPress any key to exit");
		getchar();
		exit(0);
	}

	RFID_HANDLE32 readerHandle;
	RFID_STATUS rfidStatus = ConnectReader(&readerHandle, hostName, readerPort);

	if(RFID_API_SUCCESS == rfidStatus)
	{
		TAG_STORAGE_SETTINGS tagStorageSettings;	

		RFID_GetTagStorageSettings(readerHandle,&tagStorageSettings);
		tagStorageSettings.discardTagsOnInventoryStop = TRUE;
		RFID_SetTagStorageSettings(readerHandle,&tagStorageSettings);

		CreateEventThread(readerHandle);
		Createmenu(readerHandle);
	}
	return 0;
}

void Createmenu(RFID_HANDLE32 readerHandle)
{
	int option = 0;
	RFID_STATUS rfidStatus = RFID_API_SUCCESS;
	while(1)
	{
		wprintf(L"\n");
		wprintf(L"\n----Command Menu----");
		wprintf(L"\n1. Capability -- Displays the device capabilities");
		wprintf(L"\n2. Configuration");
		wprintf(L"\n3. Inventory");
		wprintf(L"\n4. Access  - Select Mode of Access");
		wprintf(L"\n5. Exit\n");
		while(1 != scanf("%d",&option))
		{
			wprintf(L"\nEnter a Valid Input:");
			clean_stdin();
		}
		switch(option)
		{
		case 1:
			rfidStatus = ReaderCapability(readerHandle);
			break;
		case 2:
			ConfigurationMenu(readerHandle);
			break;
		case 3:
			InventoryMenu(readerHandle);
			break;
		case 4:
			AccessMenu(readerHandle);
			break;
		case 5:
			KillEventThread();
			if(g_antennaInfo.pAntennaList)
			{
				delete [] g_antennaInfo.pAntennaList;
				g_antennaInfo.pAntennaList = NULL;
			}
			RFID_Disconnect(readerHandle);
			return;
		}
	}
}


void ConfigurationMenu(RFID_HANDLE32 readerHandle)
{
	RFID_STATUS rfidStatus = RFID_API_SUCCESS;

	while(1)
	{
		int option = 0;
		wprintf(L"\n");
		wprintf(L"\n----Command Menu----");
		wprintf(L"\n1. Get Singulation Control");
		wprintf(L"\n2. GPO");
		wprintf(L"\n3. GP1");
		wprintf(L"\n4. Antenna Config");
		wprintf(L"\n5. RF Mode ");
		wprintf(L"\n6. Back  to main menu\n");
		while(1 != scanf("%d",&option))
		{
			wprintf(L"\nEnter a Valid Input:");
			clean_stdin();
		}
		switch(option)
		{
		case 1:
			rfidStatus = SingulationControl(readerHandle,&singulationControl);
			break;
		case 2:
			rfidStatus = ConfigureGPO(readerHandle);
			break;
		case 3:
			rfidStatus = ConfigureGPI(readerHandle);
			break;
		case 4:
			rfidStatus = ConfigureAntenna(readerHandle);
			break;
		case 5:
			rfidStatus = ConfigureRFMode(readerHandle);
			break;
		case 6:
			return;
		}
		if(option > 0 && option < 6)
		{
			if(rfidStatus != RFID_API_SUCCESS)
			{
				ERROR_INFO ErrorInfo;
				RFID_GetLastErrorInfo(readerHandle,&ErrorInfo);
				wprintf(L"\nOperation Failed. Reason : %ls ",ErrorInfo.statusDesc);
			}
		}
	}
}

void InventoryMenu(RFID_HANDLE32 readerHandle)
{
	
	while(1)
	{
		int option = 0;
		wprintf(L"\n");
		wprintf(L"\n----Command Menu----");
		wprintf(L"\n1. Simple");
		wprintf(L"\n2. Periodic Inventory");
		wprintf(L"\n3. Pre- filter");
		wprintf(L"\n4. Back  to main menu\n");
		while(1 != scanf("%d",&option))
		{
			wprintf(L"\nEnter a Valid Input:");
			clean_stdin();
		}
			
		switch(option)
		{
		case 1:
			SimpleInventory(readerHandle);
			break;
		case 2:
			PeriodicInventory(readerHandle);
			break;
		case 3:
			InventoryFilterOption(readerHandle);
			break;
		case 4:
			return;
		}
	}
	
}
void AccessMenu(RFID_HANDLE32 readerHandle)
{
	int option = 0;
	while(1)
	{
		wprintf(L"\n");
		wprintf(L"\n----Command Menu----");
		wprintf(L"\n1. Access Operation with Specific EPC-ID ");
		wprintf(L"\n2. Access Operation with Access-Filters");
		wprintf(L"\n3. Back  to main menu\n");
	    while(1 != scanf("%d",&option))
		{
			wprintf(L"\nEnter a Valid Input:");
			clean_stdin();
		}
		switch(option)
		{
		case 1:
			PerformSingleTagAccess(readerHandle);
			break;	
		case 2:
			MultipleTagAccess(readerHandle);
			break;	
		case 3:
			return;
		}
	}
}

void InventoryFilterOption(RFID_HANDLE32 readerHandle)
{   
	while(1)
	{

		RFID_STATUS rfidStatus = RFID_API_SUCCESS;
		int option = 0;
		wprintf(L"\n");
		wprintf(L"\n----Command Menu----");
		wprintf(L"\n1. Add Pre-Filter [only 2 filters are allowed]");
		wprintf(L"\n2. Remove PreFilter");
		wprintf(L"\n3. Exit to Inventory-Menu\n");
		while(1 != scanf("%d",&option))
		{
			wprintf(L"\nEnter a Valid Input:");
			clean_stdin();
		}
		switch(option)
		{
		case 1:
			rfidStatus = AddPreFilter(readerHandle);	
			break;
		case 2:
			rfidStatus = RemovePrefilter(readerHandle);
			break;
		case 3:
			return;
			
		}
		if(rfidStatus != RFID_API_SUCCESS)
		{
			ERROR_INFO ErrorInfo;
			RFID_GetLastErrorInfo(readerHandle,&ErrorInfo);
			wprintf(L"Operation failed . Reason : %ls",ErrorInfo.statusDesc);
		}
	}
	
	
}



