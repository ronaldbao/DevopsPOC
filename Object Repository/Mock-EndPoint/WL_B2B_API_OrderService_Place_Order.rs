<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>WL_B2B_API_OrderService_Place_Order</name>
   <tag></tag>
   <elementGuidId>6f6a25be-8902-4480-9a47-54ba5b06bd75</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;Envelope xmlns=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
    &lt;Body>
        &lt;placeOrderRequest xmlns=&quot;http://www.telus.com/b2bapi/OrderService/schema&quot;>
            &lt;order>
                &lt;refNumber>123&lt;/refNumber>
                &lt;customer>
                    &lt;addressPrimary>
                        &lt;doorNo>123&lt;/doorNo>
                        &lt;building>567&lt;/building>
                        &lt;street>Kingsway&lt;/street>
                        &lt;city>Burnaby&lt;/city>
                        &lt;country>Canada&lt;/country>
                        &lt;phoneMobile>6046006730&lt;/phoneMobile>
                        &lt;phoneLandLine>6043456545&lt;/phoneLandLine>
                        &lt;email>test@test.com&lt;/email>
                    &lt;/addressPrimary>
                    &lt;addressSecondary>
                        &lt;doorNo>123&lt;/doorNo>
                        &lt;building>567&lt;/building>
                        &lt;street>Kingsway&lt;/street>
                        &lt;city>Burnaby&lt;/city>
                        &lt;country>Canada&lt;/country>
                        &lt;phoneMobile>6046006730&lt;/phoneMobile>
                        &lt;phoneLandLine>6043456545&lt;/phoneLandLine>
                        &lt;email>test@test.com&lt;/email>
                    &lt;/addressSecondary>
                    &lt;name>
                        &lt;fName>David&lt;/fName>
                        &lt;mName>M&lt;/mName>
                        &lt;lName>Lee&lt;/lName>
                    &lt;/name>
                &lt;/customer>
                &lt;dateSubmitted>2019-03-10T12:00:00-05:00&lt;/dateSubmitted>
                &lt;orderDate>2019-03-10T12:00:00-05:00&lt;/orderDate>
                &lt;items>
                    &lt;type>Snacks&lt;/type>
                    &lt;name>pizza&lt;/name>
                    &lt;quantity>1&lt;/quantity>
                &lt;/items>
            &lt;/order>
        &lt;/placeOrderRequest>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>placeOrder</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress>http://192.168.56.1:7001/telus-provisioning-nc-data-interfacess-ed-ws-1.0/b2bapi/OrderService.wsdl</wsdlAddress>
</WebServiceRequestEntity>
