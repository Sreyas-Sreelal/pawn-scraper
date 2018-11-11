#if defined _inc_pawnscraper
	#undef _inc_pawnscraper
#endif

#if defined _pawnscraper_included
	#endinput
#endif
#define _pawnscraper_included

#define END_OF_ELEMENTS (0)
#define INVALID_HTML_DOC (Html:-1)
#define INVALID_SELECTOR (Selector:-1)
#define INVALID_ATTRIBUTE (-2)
#define SCRAPER_ERROR (-1)
#define SCRAPER_HTTP_ERROR (Response:-1)

native Html:ParseHtmlDocument(document[]);
native Html:ResponseParseHtml(Response:id);
native Response:HttpGet(url[]);
native Selector:ParseSelector(string[]);
native GetNthElementName(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string));
native GetNthElementText(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string));
native GetNthElementAttrVal(Html:docid,Selector:selectorid,idx,attribute[],string[],size = sizeof(string));
native DeleteHtml(Html:id);
native DeleteSelector(Selector:id);
native DeleteResponse(Response:id);

stock operator~(Html:html[], size) {
	#pragma unused size
	DeleteHtml(html[0]);
}

stock operator~(Selector:selector[], size) {
	#pragma unused size
	DeleteSelector(selector[0]);
}

stock operator~(Response:response[], size) {
	#pragma unused size
	DeleteResponse(response[0]);
}