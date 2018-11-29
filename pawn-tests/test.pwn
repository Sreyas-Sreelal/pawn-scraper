#include<a_samp>

#define RUN_TESTS
#include <YSI\y_testing>

#include "../include/pawnscraper"

Test:TestHttpGetThreaded(){
	HttpGetThreaded(0,"OnHttpGetRequest","https://sa-mp.com");
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
