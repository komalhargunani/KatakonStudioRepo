import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Location/GoToList'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/List/AddGuest/Page_LRS Connect/button_Add Guest'))

WebUI.setText(findTestObject('Object Repository/List/AddGuest/Page_LRS Connect/input_Add Guest_firstName'), 'Komal')

WebUI.setText(findTestObject('Object Repository/List/AddGuest/Page_LRS Connect/input_Add Guest_lastName'), 'H')

WebUI.setText(findTestObject('Object Repository/List/AddGuest/Page_LRS Connect/input_Add Guest_partySize'), '7')

WebUI.setText(findTestObject('Object Repository/List/AddGuest/Page_LRS Connect/input_Add Guest_contact'), '904-800-8378')

WebUI.click(findTestObject('Object Repository/List/AddGuest/Page_LRS Connect/button_Add Guest_1'))

WebUI.getText(findTestObject('List/AddGuest/Page_LRS Connect/Page_LRS Connect/div_2'), FailureHandling.STOP_ON_FAILURE)

WebUI.takeScreenshot()

WebUI.closeBrowser()

