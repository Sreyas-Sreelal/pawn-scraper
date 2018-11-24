#if defined _inc_pawnscraper
	#undef _inc_pawnscraper
#endif

#if defined _pawnscraper_included
	#endinput
#endif
#define _pawnscraper_included

#define INVALID_HTML_DOC (Html:-1)
#define INVALID_SELECTOR (Selector:-1)
#define INVALID_ATTRIBUTE (-2)
#define SCRAPER_ERROR (-1)
#define INVALID_HTTP_RESPONSE (Response:-1)
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
ResponseParseHtml(document[])
	Params
		id - Http response id returned from HttpGet
	Returns
		 - Html document instance id
		 - if failed to parse document INVALID_HTML_DOC is returned
*/
native Html:ResponseParseHtml(Response:id);

/*
HttpGet(url[])
	Params
		url[] - Url of a website
	Returns
		 - Response id if successful
		 - if failed to INVALID_HTTP_RESPONSE is returned
*/
native Response:HttpGet(url[]);

/*
HttpGetThreaded(playerid,callback[],url[])
	Params
		playerid - id of the player
		callback[] - name of the callback function to handle the response.
		url[] - Url of a website
*/
native HttpGetThreaded(playerid,callback[],url[]);

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

//Internal functions that deletes created objects from plugin memory when they get out of scope 

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