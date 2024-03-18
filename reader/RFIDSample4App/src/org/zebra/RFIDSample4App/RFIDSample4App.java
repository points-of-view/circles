package org.zebra.RFIDSample4App;

import com.mot.rfid.api3.*;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Hashtable;
import java.util.concurrent.locks.*;

public class RFIDSample4App {
	RFIDReader myReader = null;
        private boolean inventoryComplete = false;
        
        private Lock accessEventLock = new ReentrantLock();
        private Condition accessEventCondVar = accessEventLock.newCondition();
        
        private Lock inventoryStopEventLock = new ReentrantLock();
        private Condition inventoryStopCondVar = inventoryStopEventLock.newCondition();
	
	public static Hashtable<String,Long> tagStore = null;
	
	public static final String API_SUCCESS = "Function Succeeded";
	public static final String PARAM_ERROR = "Parameter Error";
	final String APP_NAME = "J_RFIDSample3";
	
	public boolean isConnected;
	public String hostName = "169.254.156.114";
	public int port = 5084;
	
	String[] memoryBank = new String[] { "Reserved", "EPC", "TID", "USER" };
	
	public boolean isAccessSequenceRunning = false;
	String[] tagState = new String[] { "New", "Gone", "Back", "None" };
	
	// To display tag read count
	public long uniqueTags = 0;
	public long totalTags = 0;
	
	private EventsHandler eventsHandler = new EventsHandler();
	
	// Antennas
	
	public Antennas antennas;
	
	// Access Filter
	public AccessFilter accessFilter = null;
	public boolean isAccessFilterSet = false;

	// Post Filter
	public PostFilter postFilter = null;
	public boolean isPostFilterSet = false;

	// Antenna Info
	public AntennaInfo antennaInfo = null;

	// Filter
	public PreFilters preFilters = null;
	
	public PreFilters.PreFilter preFilter1 = null;
	public PreFilters.PreFilter preFilter2 = null;

	public String preFilterTagPattern1 = null;
	public String preFilterTagPattern2 = null;
	
	public boolean isPreFilterSet1 = false;
	public boolean isPreFilterSet2 = false;
        public int preFilterActionIndex1 = 0;
        public int preFilterActionIndex2 = 0;
    
        public TriggerInfo triggerInfo = null;
	
	public int readerTypeIndex = 1;
	
    // Access
	TagAccess tagAccess = null;
	TagAccess.ReadAccessParams readAccessParams;
	TagAccess.WriteAccessParams writeAccessParams;
	TagAccess.LockAccessParams lockAccessParams;
	TagAccess.KillAccessParams killAccessParams;
	
	// Access filter

	BufferedReader inputReader = new BufferedReader(new InputStreamReader(System.in));
	public int rowId = 0;

	TagData[] myTags = null;
	
	
	public RFIDSample4App()
	{
		// Create Reader Object
		myReader = new RFIDReader();
		
		// Hash table to hold the tag data
		tagStore = new Hashtable<String, Long>();
		isAccessSequenceRunning = false;
		
		// Create the Access Filter
		accessFilter = new AccessFilter();
		accessFilter.setAccessFilterMatchPattern(FILTER_MATCH_PATTERN.A);
		accessFilter.TagPatternA = null;
		accessFilter.TagPatternB = null;

		// create the post filter
		postFilter = new PostFilter();

		// Create Antenna Info
		antennaInfo = new AntennaInfo();
		
		// Create Pre-Filter
		preFilters = new PreFilters();
   
		preFilter1 = preFilters.new PreFilter();
	    preFilter2 = preFilters.new PreFilter();
		
		antennas = myReader.Config.Antennas;

		triggerInfo = new TriggerInfo();
		
		triggerInfo.StartTrigger
				.setTriggerType(START_TRIGGER_TYPE.START_TRIGGER_TYPE_IMMEDIATE);
		triggerInfo.StopTrigger
				.setTriggerType(STOP_TRIGGER_TYPE.STOP_TRIGGER_TYPE_IMMEDIATE);

		triggerInfo.TagEventReportInfo
				.setReportNewTagEvent(TAG_EVENT_REPORT_TRIGGER.MODERATED);
		triggerInfo.TagEventReportInfo
				.setNewTagEventModeratedTimeoutMilliseconds((short) 500);

		triggerInfo.TagEventReportInfo
				.setReportTagInvisibleEvent(TAG_EVENT_REPORT_TRIGGER.MODERATED);
		triggerInfo.TagEventReportInfo
				.setTagInvisibleEventModeratedTimeoutMilliseconds((short) 500);

		triggerInfo.TagEventReportInfo
				.setReportTagBackToVisibilityEvent(TAG_EVENT_REPORT_TRIGGER.MODERATED);
		triggerInfo.TagEventReportInfo
				.setTagBackToVisibilityModeratedTimeoutMilliseconds((short) 500);

		triggerInfo.setTagReportTrigger(1);
		
		// Access Params
		
		tagAccess = new TagAccess();
		readAccessParams  = tagAccess.new ReadAccessParams();
		writeAccessParams = tagAccess.new WriteAccessParams();
		lockAccessParams  = tagAccess.new LockAccessParams();
		killAccessParams  = tagAccess.new KillAccessParams();
		
				
		// On Device, connect automatically to the reader
		connectToReader(hostName, port);
			
		
		
		
	}
	
	
	public void Createmenu()
    {
        int option = 0;
        Boolean keepWorking = true;
       
       
        while (keepWorking)
        {
            System.out.println("----Command Menu----");
            System.out.println("1. Start reading");
            System.out.println("2. Configuration");
            System.out.println("5. Exit");
            
            

            try
            {
                option =  Integer.valueOf(inputReader.readLine());

                switch (option)
                {
                    // Inventory
                    case 1:
                    	InventoryMenu();
                        break;
                    // Configuration
                    case 2:
                        ConfigurationMenu();
                        break;
                    // Application Exit
                    case 5:
                    	myReader.disconnect();
                        keepWorking = false;
                        break;
                }
            }
            catch (Exception ex)
            {
            	System.out.println(ex.getMessage());
            }
        }
    }
		
	private void ConfigurationMenu()
	{
          Boolean keepworking = true;
          while (keepworking)
          {
             
             try
             {
             
	             
	                      ConfigureAntenna();
	                      break;
	                  
             }
             catch(NumberFormatException nfe)
             {
            	 System.out.println("Can't configure antenna"+nfe.getMessage());
            	 
             }
          }
		
	}
	
	private void ConfigureAntenna()
	{
	    Integer antennaID;
        Boolean keepworking = true;
        Antennas antennas = myReader.Config.Antennas;
        Antennas.Config antennaConfig;
        Integer option = 0;
        
        while (keepworking)
    	{
    		System.out.println("----Command Menu----");
            System.out.println("1. SetAntennaConfig");
            System.out.println("2. GetAntennaConfig");
            System.out.println("");
            System.out.println("3. Go back");
            try
            {
              option = Integer.valueOf(inputReader.readLine());
              switch (option)
              {
                case 1:
                   {  
                    	System.out.println("Enter AntennaID");
                        antennaID = Integer.valueOf(inputReader.readLine());
                        
                        antennaConfig = antennas.getAntennaConfig(antennaID);
                                          
                        System.out.println("Enter ReceiveSensitivityIndex  value ");
                        antennaConfig.setReceiveSensitivityIndex(Short.valueOf(inputReader.readLine()));
                       
                        System.out.println("Enter TransmitPowerIndex  value ");
                        antennaConfig.setTransmitPowerIndex(Short.valueOf(inputReader.readLine()));

                        System.out.println("Enter TransmitFrequencyIndex value ");
                        antennaConfig.setTransmitFrequencyIndex(Short.valueOf(inputReader.readLine()));
                       
                        antennas.setAntennaConfig(antennaID, antennaConfig);
                        System.out.println("Set Antenna Configuration Successfully");
                                      
                    }
                    break;
                    case 2:
                        
                        System.out.println("Enter AntennaID");
                        antennaID = Integer.valueOf(inputReader.readLine());
                        antennaConfig = antennas.getAntennaConfig(antennaID);

                        System.out.println("ReceiveSensitivityIndex: "+antennaConfig.getReceiveSensitivityIndex());
                        System.out.println("TransmitPowerIndex: "+antennaConfig.getTransmitPowerIndex());
                        System.out.println("TransmitFrequencyIndex: "+antennaConfig.getTransmitFrequencyIndex());

                        break;
                    case 3:
                        keepworking = false;
                        break;
                    default:
                    	 System.out.println("Enter a valid integer in the range 1-3");
                    	break;
                }
            }
            catch (NumberFormatException nfe)
            {
            	System.out.println("Invalid Input format "+nfe.getMessage());
            }
            catch (InvalidUsageException iue)
            {
            	System.out.println("Invalid Usage exception  "+iue.getInfo());
            }
            
            catch (OperationFailureException opEx)
            {
                System.out.println("Antenna Configuration failed.Reason: "+opEx.getVendorMessage());
            }
            catch (Exception ex)
            {
                System.out.println(ex.toString());
           	}
    	}
        	
      }
          
	private void InventoryMenu() throws IOException
	{
        Boolean keepworking = true;
        while (keepworking)
        {
            
            try
            {
            	SimpleInventory();
            	break;
            }
            catch(NumberFormatException nfe)
            { 
            	 System.out.println("Invalid format"+nfe.getMessage());
            }
    	    catch(InterruptedException ie)
    		{
    			System.out.println("Inventory interruped prematurely."+ie.getMessage());
    			
    		}
            catch (InvalidUsageException iue)
    		{
    		    System.out.println("Invalid usage.Reason: "+iue.getMessage());
    		}
    		catch(OperationFailureException opex)
    		{
    			System.out.println("Failed to start inventory.Reason: "+opex.getMessage());
    			
    		}
    		
        }
		  
	}
	
	private void SimpleInventory() throws InterruptedException,InvalidUsageException,OperationFailureException
	{
			
		   tagStore.clear();
                   
                  			
		   myReader.Actions.Inventory.perform();
                   
                   System.out.println("Press Enter to stop inventory");
                   
                   try
                   { 
                      inputReader.readLine();
                   }
                   catch(IOException ioex)
                   {
                       System.out.println("IO Exception.Stopping inventory");
                   }
	       	   finally
                   {
                       myReader.Actions.Inventory.stop();
                      
                   }
                   
                   try
                   {
                       inventoryStopEventLock.lock();
                       if(!inventoryComplete)
                       {
                        inventoryStopCondVar.await();
                        inventoryComplete = false;
                       }
                       
                   }
		   finally
                   {
                       inventoryStopEventLock.unlock();
                   }
	 }

	public RFIDReader getMyReader() {
		return myReader;
	}
	
	void updateTags(Boolean isAccess)
	{
		TagDataArray oTagDataArray = myReader.Actions.getReadTagsEx(1000);
		myTags = oTagDataArray.getTags();
		
		if (myTags != null)
		{
				 if(!isAccess)
				 {
					 for (int index = 0; index < oTagDataArray.getLength(); index++) 
					 {
						 TagData tag = myTags[index];
						 String key = tag.getTagID();
						 String antennaId = String.valueOf(tag.getAntennaID());
						 String peakRSSI = String.valueOf(tag.getPeakRSSI());
						// if (!tagStore.containsKey(key))
						// {
						//	tagStore.put(key,totalTags);
							postInfoMessage("ReadTag: "+ key + "|" + antennaId + "|" + peakRSSI); 
							//uniqueTags++;
						 // }
						 totalTags++;
					 }
				
				 }
				 else
				 {
					 for (int index = 0; index < myTags.length; index++)
					 {
						 TagData tag = myTags[index];
						 if(tag.getMemoryBankData() != null)
						    postInfoMessage("TagID "+tag.getTagID()+tag.getMemoryBank().toString()+"  "+tag.getMemoryBankData());
						 else
							 postInfoMessage("TagID "+tag.getTagID()+"Access Status:  "+tag.getOpStatus().toString()); 
						   	 
					 }
				 }
				
				
			
			
		}
		
	}
	
	void postStatusNotification(String statusMsg, String vendorMsg)
	{
		System.out.println("Status: "+statusMsg+" Vendor Message: "+vendorMsg);
	}
	
    static void postInfoMessage(String msg)
    {
    	System.out.println(msg);
    }
   
    public class EventsHandler implements RfidEventsListener
    {
    	public EventsHandler()
    	{
    		
    	}
    	
    	public void eventReadNotify(RfidReadEvents rre) {
			 
    		updateTags(false);
		}
    	
    	
    	
    	
    	public void eventStatusNotify(RfidStatusEvents rse)
    	{
    		postInfoMessage(rse.StatusEventData.getStatusEventType().toString());
    		
    		STATUS_EVENT_TYPE statusType = rse.StatusEventData.getStatusEventType();
    		if (statusType == STATUS_EVENT_TYPE.ACCESS_STOP_EVENT)
    		{
    			try
    			{
    			  accessEventLock.lock();
    			  accessEventCondVar.signalAll();
    			}
    			finally
    			{
    				accessEventLock.unlock();
    				
    			}
    			
    		}
                 else if(statusType == STATUS_EVENT_TYPE.INVENTORY_STOP_EVENT)
                 {
                     try
                     {
                         inventoryStopEventLock.lock();
                         inventoryComplete = true;
                         inventoryStopCondVar.signalAll();
                         
                     }
                     finally
                     {
                         inventoryStopEventLock.unlock();
                     }
                             
                 }
    		else if(statusType == STATUS_EVENT_TYPE.BUFFER_FULL_WARNING_EVENT || statusType == STATUS_EVENT_TYPE.BUFFER_FULL_EVENT)
    		{
    			postInfoMessage(statusType.toString());
    		}
    		
	   	}
    }
   
    	
	public boolean connectToReader(String readerHostName, int readerPort)
	{
		
		boolean retVal = false;
		hostName = readerHostName;
		port = readerPort;
		myReader.setHostName(hostName);
		myReader.setPort(port);
		
		try {
			myReader.connect();

			myReader.Events.setInventoryStartEvent(true);
			myReader.Events.setInventoryStopEvent(true);
			myReader.Events.setAccessStartEvent(true);
			myReader.Events.setAccessStopEvent(true);
			myReader.Events.setAntennaEvent(true);
			myReader.Events.setGPIEvent(true);
			myReader.Events.setBufferFullEvent(true);
			myReader.Events.setBufferFullWarningEvent(true);
			myReader.Events.setReaderDisconnectEvent(true);
			myReader.Events.setReaderExceptionEvent(true);
			myReader.Events.setTagReadEvent(true);
			myReader.Events.setAttachTagDataWithReadEvent(false);
                        
                        TagStorageSettings tagStorageSettings = myReader.Config.getTagStorageSettings();
                        tagStorageSettings.discardTagsOnInventoryStop(true);
                        myReader.Config.setTagStorageSettings(tagStorageSettings);

			myReader.Events.addEventsListener(eventsHandler);
			
			retVal = true;
			isConnected = true;
			postInfoMessage("Connected to " + hostName);
			postStatusNotification(API_SUCCESS, null);
			myReader.Config.setTraceLevel(TRACE_LEVEL.TRACE_LEVEL_ERROR);
			
			Createmenu();

		} catch (InvalidUsageException ex)
                {
			System.out.println("invalidusage");
		    postStatusNotification(PARAM_ERROR, ex.getVendorMessage());
		} catch (OperationFailureException ex) {
			System.out.println("Operationfailure");
			postStatusNotification(ex.getStatusDescription(),
					ex.getVendorMessage());
		}
		
		
		return retVal;
		
	}
	
	public static void main(String[] args) throws InterruptedException {
		@SuppressWarnings("unused")
		RFIDSample4App rfidBase; 
		rfidBase = new RFIDSample4App();
	}
	
  }
