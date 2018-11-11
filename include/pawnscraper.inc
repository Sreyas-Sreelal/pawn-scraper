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
#define INVALID_HTTP_RESPONSE (Response:-1)

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
	if(html[0] != INVALID_HTML_DOC) {
		DeleteHtml(html[0]);
	}
}

stock operator~(Selector:selector[], size) {
	#pragma unused size
	if(selector[0] != INVALID_SELECTOR) {
		DeleteSelector(selector[0]);
	}
}

stock operator~(Response:response[], size) {
	#pragma unused size
	if(response[0] != INVALID_HTTP_RESPONSE) {
		DeleteResponse(response[0]);
	}
}