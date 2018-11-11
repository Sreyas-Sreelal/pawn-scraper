#include<a_samp>

#define RUN_TESTS
#include <YSI\y_testing>

#include<pawnscraper>

Test:TestParseHtmlDocument(){
	new Html:doc = ParseHtmlDocument("\
		<!DOCTYPE html>\
		<meta charset=\"utf-8\">\
		<title>Hello, world!</title>\
		<h1 class=\"foo\">Hello, <i>world!</i></h1>\
		");
	ASSERT(doc != INVALID_HTML_DOC);
}

Test:TestParseHtmlDocumentError(){
	new Html:doc = ParseHtmlDocument("invalid html doc here");
	ASSERT(doc != INVALID_HTML_DOC);
}

Test:TestParseSelector(){
	new Selector:selector = ParseSelector("h1 .foo");
	ASSERT(selector != INVALID_SELECTOR);
}

Test:TestParseSelectorError(){
	new Selector:selector = ParseSelector("INVALID <selector> ");
	ASSERT(selector == INVALID_SELECTOR);
}

Test:TestResponseParseHtml(){
	new Response:response = HttpGet("https://www.sa-mp.com");
	new Html:doc = ResponseParseHtml(response);
	ASSERT(doc != INVALID_HTML_DOC);
}

Test:TestResponseParseHtmlError(){
	new Response:invalid_response_id = Response:10;
	new Html:doc = ResponseParseHtml(invalid_response_id);
	ASSERT(doc == INVALID_HTML_DOC);
}

Test:TestHttpGet(){
	new Response:response = HttpGet("https://www.sa-mp.com");
	ASSERT(response != INVALID_HTTP_RESPONSE);
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
}

