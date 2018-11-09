#if defined _inc_pawnscraper
	#undef _inc_pawnscraper
#endif

#if defined _pawnscraper_included
	#endinput
#endif
#define _pawnscraper_included

native Html:ParseHtmlDocument(document[]);
native Selector:ParseSelector(string[]);
native Element:SelectElementValueById(Html:docid,Selector:selectorid,idx);
native GetElementName(Element:elementid,string[],size = sizeof(string));