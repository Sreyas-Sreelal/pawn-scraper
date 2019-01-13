#if defined _inc_pawnscraper
	#undef _inc_pawnscraper
#endif

#if defined _pawnscraper_included
	#endinput
#endif
#define _pawnscraper_included

#define PAWN_SCRAPER_VERSION 20
#define INVALID_HTML_DOC (Html:-1)
#define INVALID_SELECTOR (Selector:-1)
#define INVALID_ATTRIBUTE (-2)
#define SCRAPER_ERROR (-1)
#define INVALID_HTTP_RESPONSE (Response:-1)
#define INVALID_HEADER (Header:-1)

public _pawnscraper_version = PAWN_SCRAPER_VERSION;
#pragma unused _pawnscraper_version

/*
ParseHtmlDocument(document[])
	Params
		document[] - string of html document
	Returns
		 - Html document instance id
		 - if failed to parse document INVALID_HTML_DOC is returned
*/
native Html:ParseHtmlDocument(document[]);

/*
ResponseParseHtml(Response:id)
	Params
		id - Http response id returned from HttpGet
	Returns
		 - Html document instance id
		 - if failed to parse document INVALID_HTML_DOC is returned
*/
native Html:ResponseParseHtml(Response:id);

/*
HttpGet(url[],Header:headerid=INVALID_HEADER)
	Params
		url[] - Url of a website
		header - id of header object created using CreateHeader
	Returns
		 - Response id if successful
		 - if failed to INVALID_HTTP_RESPONSE is returned
*/
native Response:HttpGet(url[],Header:headerid=INVALID_HEADER);

/*
HttpGetThreaded(playerid,callback[],url[],Header:headerid=INVALID_HEADER)
	Params
		playerid - id of the player
		callback[] - name of the callback function to handle the response.
		url[] - Url of a website
		header - id of header object created using CreateHeader
*/
native HttpGetThreaded(playerid,callback[],url[],Header:headerid=INVALID_HEADER);

/*
ParseSelector(string[])
	Params
		string[] - CSS selector 
	Returns
		 - Selector instance id if successful
		 - if failed to INVALID_SELECTOR is returned
*/
native Selector:ParseSelector(string[]);

/*
CreateHeader(...)
	Params
		key value pairs
*/
native Header:CreateHeader(...);

/*
GetNthElementName(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string))
	Params
		docid - Html instance id
		selectorid - CSS selector instance id
		idx - the n'th occurence of element in the document (starts from 0)
		string[] - element name is stored
		size - sizeof string
	Returns
		- 1 if successful
		- 0 if failed
*/
native GetNthElementName(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string));

/*
GetNthElementText(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string))
	Params
		docid - Html instance id
		selectorid - CSS selector instance id
		idx - the n'th occurence of element in the document (starts from 0)
		string[] - element name
		size - sizeof string
	Returns
		- 1 if successful
		- 0 if failed
*/
native GetNthElementText(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string));

/*
GetNthElementAttrVal(Html:docid,Selector:selectorid,idx,attribute[],string[],size = sizeof(string))
	Params
		docid - Html instance id
		selectorid - CSS selector instance id
		idx - the n'th occurence of element in the document (starts from 0)
		attribute[] - the attribute of element
		string[] - element name
		size - sizeof string
	Returns
		- 1 if successful
		- 0 if failed
*/
native GetNthElementAttrVal(Html:docid,Selector:selectorid,idx,attribute[],string[],size = sizeof(string));

/*
DeleteHtml(Html:id)
	Params
		id - html instance to be deleted
*/
native DeleteHtml(Html:id);

/*
DeleteSelector(Selector:id)
	Params
		id - selector instance to be deleted
*/
native DeleteSelector(Selector:id);

/*
DeleteResponse(Html:id)
	Params
		id - response instance to be deleted
*/
native DeleteResponse(Response:id);

/*
DeleteHeader(Header:id)
	Params
		id - header instance to be deleted
*/
native DeleteHeader(Header:id);
