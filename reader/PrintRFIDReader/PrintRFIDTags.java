package reader.PrintRFIDReader;

import java.util.Hashtable;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

import com.mot.rfid.api3.InvalidUsageException;
import com.mot.rfid.api3.OperationFailureException;
import com.mot.rfid.api3.RFIDReader;
import com.mot.rfid.api3.RfidEventsListener;
import com.mot.rfid.api3.RfidReadEvents;
import com.mot.rfid.api3.RfidStatusEvents;
import com.mot.rfid.api3.STATUS_EVENT_TYPE;
import com.mot.rfid.api3.TagData;
import com.mot.rfid.api3.TagDataArray;

public class PrintRFIDTags {
	RFIDReader myReader = null;
	private boolean inventoryComplete = false;
	private Lock accessEventLock = new ReentrantLock();
    private Condition accessEventCondVar = accessEventLock.newCondition();
	private Lock inventoryStopEventLock = new ReentrantLock();
	private Condition inventoryStopCondVar = inventoryStopEventLock.newCondition();

	public static Hashtable<String, Long> tagStore = null;
	public String hostName = "169.254.156.114";
	public int port = 5084;
	private EventsHandler eventsHandler = new EventsHandler();

	TagData[] myTags = null;

	private void StartReading() {
		while (true) {
			try {
				SimpleInventory();
				break;
			} catch (InterruptedException ie) {
				System.out.println("Inventory interruped prematurely." + ie.getMessage());

			} catch (InvalidUsageException iue) {
				System.out.println("Invalid usage.Reason: " + iue.getMessage());
			} catch (OperationFailureException opex) {
				System.out.println("Failed to start inventory.Reason: " + opex.getMessage());

			}
		}
	}

	public PrintRFIDTags() throws InvalidUsageException, OperationFailureException {
		myReader = new RFIDReader();

		// Hash table to hold the tag data
		tagStore = new Hashtable<String, Long>();
		connectToReader(hostName, port);
	}

	private void SimpleInventory() throws InterruptedException, InvalidUsageException, OperationFailureException {

		tagStore.clear();

		myReader.Actions.Inventory.perform();

		try {
			inventoryStopEventLock.lock();
			if (!inventoryComplete) {
				inventoryStopCondVar.await();
				inventoryComplete = false;
			}

		} finally {
			inventoryStopEventLock.unlock();
		}
	}

	public RFIDReader getMyReader() {
		return myReader;
	}

	void updateTags(Boolean isAccess) {
		TagDataArray oTagDataArray = myReader.Actions.getReadTagsEx(1000);
		myTags = oTagDataArray.getTags();

		if (myTags != null) {
			if (!isAccess) {
				for (int index = 0; index < oTagDataArray.getLength(); index++) {
					TagData tag = myTags[index];
					String key = String.valueOf(tag.getTagID());
					String antennaId = String.valueOf(tag.getAntennaID());
					String peakRSSI = String.valueOf(tag.getPeakRSSI());
					System.out.println(key + "|" + antennaId + "|" + peakRSSI);
				}

			} else {
				for (int index = 0; index < myTags.length; index++) {
					TagData tag = myTags[index];
					if (tag.getMemoryBankData() != null)
						System.out.println("TagID " + tag.getTagID() + tag.getMemoryBank().toString() + "  "
								+ tag.getMemoryBankData());
					else
						System.out.println("TagID " + tag.getTagID() + "Access Status:  " + tag.getOpStatus().toString());

				}
			}

		}

	}

	public class EventsHandler implements RfidEventsListener {
		public EventsHandler() {

		}

		public void eventReadNotify(RfidReadEvents rre) {
			updateTags(false);
		}

		public void eventStatusNotify(RfidStatusEvents rse) {
			System.out.println(rse.StatusEventData.getStatusEventType().toString());

			STATUS_EVENT_TYPE statusType = rse.StatusEventData.getStatusEventType();
			if (statusType == STATUS_EVENT_TYPE.ACCESS_STOP_EVENT) {
				try {
					accessEventLock.lock();
					accessEventCondVar.signalAll();
				} finally {
					accessEventLock.unlock();

				}

			} else if (statusType == STATUS_EVENT_TYPE.INVENTORY_STOP_EVENT) {
				try {
					inventoryStopEventLock.lock();
					inventoryComplete = true;
					inventoryStopCondVar.signalAll();

				} finally {
					inventoryStopEventLock.unlock();
				}

			} else if (statusType == STATUS_EVENT_TYPE.BUFFER_FULL_WARNING_EVENT
					|| statusType == STATUS_EVENT_TYPE.BUFFER_FULL_EVENT) {
				System.out.println(statusType.toString());
			}

		}
	}

	public void connectToReader(String readerHostName, int readerPort) throws InvalidUsageException, OperationFailureException {
		hostName = readerHostName;
		port = readerPort;
		myReader.setHostName(hostName);
		myReader.setPort(port);

//		try {
			myReader.connect();
			myReader.Events.setInventoryStartEvent(true);
			myReader.Events.setTagReadEvent(true);
			myReader.Events.addEventsListener(eventsHandler);
			System.out.println(eventsHandler);
			StartReading();
			System.out.println("StartReading");

//		} catch (InvalidUsageException ex) {
//			System.out.println("invalidusage");
//		} catch (OperationFailureException ex) {
//			System.out.println("Operationfailure");
//		}

	}

	public static void main(String[] args) throws InterruptedException, InvalidUsageException, OperationFailureException {
		@SuppressWarnings("unused")
		PrintRFIDTags rfidBase;
		rfidBase = new PrintRFIDTags();
	}
}
