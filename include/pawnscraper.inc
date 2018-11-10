#if defined _inc_pawnscraper
	#undef _inc_pawnscraper
#endif

#if defined _pawnscraper_included
	#endinput
#endif
#define _pawnscraper_included

#define END_OF_ELEMENTS (0)
#define PS_ERROR (-1)

native Html:ParseHtmlDocument(document[]);
native Selector:ParseSelector(string[]);
native GetNthElementName(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string));
native GetNthElementText(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string));
native Response:HttpGet(url[]);
native DeleteResponse(Response:id);
native Html:ResponseParseHtml(Response:id);
native GetNthElementAttrVal(Html:docid,Selector:selectorid,idx,attribute[],string[],size = sizeof(string));


