#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/pawnscraper.inc"

main() { }

Test:ParseDocument() {
	new Html:html = ParseHtmlDocument("\
		<!DOCTYPE html>\
		<meta charset=\"utf-8\">\
		<title>Hello, world!</title>\
		<h1 class=\"foo\">Hello, <i>world!</i></h1>\
	");
	ASSERT(html >= Html:0);
}
Test:ParseSelector() {
	new Selector:selector = ParseSelector("li");
	ASSERT(selector >= Selector:0);
}

