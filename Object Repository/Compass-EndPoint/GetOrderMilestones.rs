<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get Order Milestone by API</description>
   <name>GetOrderMilestones</name>
   <tag></tag>
   <elementGuidId>ac3f7fbd-8b2a-46c1-9922-507fa9985716</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://${IS05Compass}/rest/api/kitsuneServices/v1/getStatus?action=milestone&amp;Ids=9149137571313366743</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'flcncapp-is05asf.tsl.telus.com'</defaultValue>
      <description></description>
      <id>0354cd44-e8cf-48a0-bea6-e111d6fbbc99</id>
      <masked>false</masked>
      <name>IS05Compass</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementPropertyValue(response, 'data[0].milestones[0].status', &quot;Cancelled&quot;)
WS.verifyElementPropertyValue(response, 'data[0].milestones[0].tasks[0].name', &quot;Facility Check&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
