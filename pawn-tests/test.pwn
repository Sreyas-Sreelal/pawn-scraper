#include<a_samp>

#define RUN_TESTS
#include <YSI_Core\y_testing>

#include "../include/pawnscraper"

Test:TestHeader(){
	new Header:header = CreateHeader("Name","Bill");
	ASSERT(header != INVALID_HEADER);
	ASSERT(DeleteHeader(header) == 1);
}

Test:TestInvalidHeader(){
	new Header:header = CreateHeader("Name","Bill","Address");
	ASSERT(header == INVALID_HEADER);
	ASSERT(DeleteHeader(header) == 0);
}

Test:TestHtppGetWithHeader(){
	new Header:header = CreateHeader(
		"User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
	);
	ASSERT(header != INVALID_HEADER);
	new Response:response = HttpGet("https://sa-mp.com/",header);
	ASSERT(response != INVALID_HTTP_RESPONSE);
	ASSERT(DeleteHeader(header) == 1);
}

Test:TestHeaderThreaded(){
	new Header:header = CreateHeader(
		"User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
	);
	ASSERT(header != INVALID_HEADER);
	HttpGetThreaded(0,"OnHttpGetRequest","https://open.mp",header);
	ASSERT(DeleteHeader(header) == 1);
}

Test:TestHttpGetThreaded(){
	HttpGetThreaded(0,"OnHttpHeaderRequest","https://open.mp");
}

forward OnHttpHeaderRequest(playerid,Response:responseid);
public OnHttpHeaderRequest(playerid,Response:responseid){
	printf("*** Test OnHttpHeaderRequest\n");
	ASSERT(responseid != INVALID_HTTP_RESPONSE);
	DeleteResponse(responseid);
	print("\nPASS!");
}

forward OnHttpGetRequest(playerid,Response:responseid);
public OnHttpGetRequest(playerid,Response:responseid){
	printf("*** Test OnHttpGetRequest\n");
	ASSERT(responseid != INVALID_HTTP_RESPONSE);
	DeleteResponse(responseid);
	print("\nPASS!");
}

Test:TestHttpGetThreadedInvalid(){
	HttpGetThreaded(0,"OnInvalidHttpGetRequest","https://sa-mp..com");
}

forward OnInvalidHttpGetRequest(playerid,Response:responseid);
public OnInvalidHttpGetRequest(playerid,Response:responseid){
	printf("*** Test OnInvalidHttpGetRequest\n");
	ASSERT(responseid == INVALID_HTTP_RESPONSE);
	print("\nPASS!");
}

Test:TestParseHtmlDocument(){
	new Html:doc = ParseHtmlDocument("\
		<!DOCTYPE html>\
		<meta charset=\"utf-8\">\
		<title>Hello, world!</title>\
		<h1 class=\"foo\">Hello, <i>world!</i></h1>\
		");
	ASSERT(doc != INVALID_HTML_DOC);
	DeleteHtml(doc);
}

Test:TestParseSelector(){
	new Selector:selector = ParseSelector("h1 .foo");
	ASSERT(selector != INVALID_SELECTOR);
	DeleteSelector(selector);
}

Test:TestParseSelectorError(){
	new Selector:selector = ParseSelector("INVALID <selector> ");
	ASSERT(selector == INVALID_SELECTOR);
}

Test:TestResponseParseHtml(){
	new Response:response = HttpGet("https://www.sa-mp.com");
	new Html:doc = ResponseParseHtml(response);
	ASSERT(doc != INVALID_HTML_DOC);
	DeleteHtml(doc);
}

Test:TestResponseParseHtmlError(){
	new Response:invalid_response_id = Response:10;
	new Html:doc = ResponseParseHtml(invalid_response_id);
	ASSERT(doc == INVALID_HTML_DOC);
}

Test:TestHttpGet(){
	new Response:response = HttpGet("https://www.sa-mp.com");
	ASSERT(response != INVALID_HTTP_RESPONSE);
	DeleteResponse(response);
}

Test:TestHttpGetError(){
	new Response:response = HttpGet("invalidurl..com");
	ASSERT(response == INVALID_HTTP_RESPONSE);
}

Test:TestGetNthElementName(){
	new Html:doc = ParseHtmlDocument("\
		<!DOCTYPE html>\
		<meta charset=\"utf-8\">\
		<title>Hello, world!</title>\
		<h1 class=\"foo\">Hello, <i>world!</i></h1>\
		");
	ASSERT(doc != INVALID_HTML_DOC);
	
	new Selector:selector = ParseSelector("i");
	ASSERT(selector != INVALID_SELECTOR);
	
	new i= -1,element_name[10];
	while(GetNthElementName(doc,selector,++i,element_name)!=0){
		ASSERT(strcmp(element_name,"i") == 0);
	}

	DeleteSelector(selector);
	DeleteHtml(doc);

}

Test:TestGetNthElementText(){
	new Html:doc = ParseHtmlDocument("\
		<!DOCTYPE html>\
		<meta charset=\"utf-8\">\
		<title>Hello, world!</title>\
		<h1 class=\"foo\">Hello, <i>world!</i></h1>\
		");
	ASSERT(doc != INVALID_HTML_DOC);
	
	new Selector:selector = ParseSelector("h1.foo");
	ASSERT(selector != INVALID_SELECTOR);
	
	new element_text[20];
	ASSERT(GetNthElementText(doc,selector,0,element_text) == 1);

	new check = strcmp(element_text,("Hello, world!"));
	ASSERT(check == 0);

	DeleteSelector(selector);
	DeleteHtml(doc);
}

Test:TestGetNthElementAttrVal(){
	new Html:doc = ParseHtmlDocument("\
		<!DOCTYPE html>\
		<meta charset=\"utf-8\">\
		<title>Hello, world!</title>\
		<h1 class=\"foo\">Hello, <i>world!</i></h1>\
		");
	ASSERT(doc != INVALID_HTML_DOC);
	
	new Selector:selector = ParseSelector("h1");
	ASSERT(selector != INVALID_SELECTOR);
	
	new element_attribute[20];
	ASSERT(GetNthElementAttrVal(doc,selector,0,"class",element_attribute) == 1);

	new check = strcmp(element_attribute,("foo"));
	ASSERT(check == 0);

	DeleteSelector(selector);
	DeleteHtml(doc);
}