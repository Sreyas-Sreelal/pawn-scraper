# pawn-scraper
![Build](https://github.com/sreyas-sreelal/pawn-scraper/actions/workflows/rust.yml/badge.svg)
[![sampctl](https://img.shields.io/badge/sampctl-supported-2f2f2f.svg)](https://github.com/Sreyas-Sreelal/pawn-scraper)
[![GitHub issues](https://img.shields.io/github/issues/Sreyas-Sreelal/pawn-scraper.svg)](https://github.com/Sreyas-Sreelal/pawn-scraper/issues) [![GitHub pull requests](https://img.shields.io/github/issues-pr-raw/sreyas-sreelal/pawn-scraper.svg)](https://github.com/Sreyas-Sreelal/pawn-scraper/pulls) [![GitHub pull license](https://img.shields.io/github/license/sreyas-sreelal/pawn-scraper.svg)](LICENSE)

A powerful scraper plugin that provides interface for utlising html_parsers and css selectors in pawn.
## Installing 

Thanks to [Southclaws](https://www.github.com/southclaws),plugin installation is now much easier with [sampctl](https://www.github.com/southclaws/sampctl)

`sampctl p install Sreyas-Sreelal/pawn-scraper`

#### OR
* Download suitable binary files from releases for your operating system
* Add it your `plugins` folder
* Add `PawnScraper` to server.cfg or  `PawnScraper.so` (for linux)
* Add [pawnscraper.inc](include/pawnscraper.inc) in includes folder

## Building
* Clone the repo

	`git clone https://github.com/Sreyas-Sreelal/pawn-scraper.git`

* Use makefile to compile and test
	* Setup testing environment 

		`make setup`
	* To build release version 

		`make release`
	* Run tests

		`make run`

## API
* ### ParseHtmlDocument(document[])
	* **Params**
		* `document[]` - string of html document
	* **Returns**
		* Html document instance id
		* if failed to parse document `INVALID_HTML_DOC` is returned
	* **Example Usage**
		```Pawn
		new Html:doc = ParseHtmlDocument("\
			<!DOCTYPE html>\
			<meta charset=\"utf-8\">\
			<title>Hello, world!</title>\
			<h1 class=\"foo\">Hello, <i>world!</i></h1>\
			");
		ASSERT(doc != INVALID_HTML_DOC);
		DeleteHtml(doc);
		```
* ### ResponseParseHtml(Response:id)
	* **Params**
		* `id` - Http response id returned from HttpGet
	* **Returns**
		* Html document instance id
		* if failed to parse document `INVALID_HTML_DOC` is returned
	* **Example Usage**
		```Pawn
		new Response:response = HttpGet("https://www.sa-mp.com");
		new Html:doc = ResponseParseHtml(response);
		ASSERT(doc != INVALID_HTML_DOC);
		DeleteHtml(doc);
		```

* ### HttpGet(url[],Header:headerid=INVALID_HEADER)
	* **Params**
		* `url[]` - Url of a website
		* `header` - id of header object created using CreateHeader
	* **Returns**
		* Response id if successful
		* if failed to `INVALID_HTTP_RESPONSE` is returned
    * **Example Usage**
        ```Pawn
        new Response:response = HttpGet("https://www.sa-mp.com");
        ASSERT(response != INVALID_HTTP_RESPONSE);
        DeleteResponse(response);
        ```

* ### HttpGetThreaded(playerid,callback[],url[],Header:headerid=INVALID_HEADER)
	* **Params**
		* `playerid` - id of the player
		* `callback[]` - name of the callback function to handle the response.
		* `url[]` - Url of a website
		* `header` - id of header object created using CreateHeader
    
    * **Example Usage**
        ```Pawn
        HttpGetThreaded(0,"MyHandler","https://sa-mp.com");
        //********
        forward MyHandler(playerid,Response:responseid);
        public MyHandler(playerid,Response:responseid){
            ASSERT(responseid != INVALID_HTTP_RESPONSE);
            DeleteResponse(responseid);
        }
        ```
 
* ### ParseSelector(string[])
	* **Params**
		* `string[]` - CSS selector 
	* **Returns**
		* Selector instance id if successful
		* if failed to `INVALID_SELECTOR` is returned
    * **Example Usage**
        ```Pawn
        new Selector:selector = ParseSelector("h1 .foo");
        ASSERT(selector != INVALID_SELECTOR);
        DeleteSelector(selector);
        ```
 
* ### CreateHeader(...)
	* **Params**
		* key,value pairs of String type
    * **Returns**
		* Header instance id if successful
		* if failed to `INVALID_HEADER` is returned
    * **Example Usage**
        ```Pawn
        new Header:header = CreateHeader(
		    "User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
	    );
        ASSERT(header != INVALID_HEADER);
        new Response:response = HttpGet("https://sa-mp.com/",header);
        ASSERT(response != INVALID_HTTP_RESPONSE);
        ASSERT(DeleteHeader(header) == 1);
        ```
 
* ### GetNthElementName(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string))
	* **Params**
		* `docid` - Html instance id
		* `selectorid` - CSS selector instance id
		* `idx` - the n'th occurence of element in the document (starts from 0)
		* `string[]` - element name is stored
		* `size` - sizeof string
	* **Returns**
		* `1` if successful
		* `0` if failed
    * **Example Usage**
        ```Pawn
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
        ```
        
* ### GetNthElementText(Html:docid,Selector:selectorid,idx,string[],size = sizeof(string))
	* **Params**
		* `docid` - Html instance id
		* `selectorid` - CSS selector instance id
		* `idx` - the n'th occurence of element in the document (starts from 0)
		* `string[]` - element name
		* `size` - sizeof string
	* **Returns**
		* `1` if successful
		* `0` if failed
    * **Example Usage**
        ```Pawn
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
        ```
* ### GetNthElementAttrVal(Html:docid,Selector:selectorid,idx,attribute[],string[],size = sizeof(string))
	* **Params**
		* `docid` - Html instance id
		* `selectorid` - CSS selector instance id
		* `idx` - the n'th occurence of element in the document (starts from 0)
		* `attribute[]` - the attribute of element
		* `string[]` - element name
		* `size` - sizeof string
	* **Returns**
		* `1` if successful
		* `0` if failed
    * **Example Usage**
		```Pawn
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
		```
* ### DeleteHtml(Html:id)
	* **Params**
		* `id` - html instance to be deleted
	* **Returns**
		* `1` if successful
		* `0` if failed

* ### DeleteSelector(Selector:id)
	* **Params**
		* `id` - selector instance to be deleted
	* **Returns**
		* `1` if successful
		* `0` if failed
 
* ### DeleteResponse(Html:id)
	* **Params**
		* `id` - response instance to be deleted
    * **Returns**
		* `1` if successful
		* `0` if failed

* ### DeleteHeader(Header:id)
	* **Params**
		* `id` - header instance to be deleted
	* **Returns**
		* `1` if successful
		* `0` if failed


## Usage
A small example to fetch all links in wiki.sa-mp.com 
```Pawn
new Response:response = HttpGet("https://wiki.sa-mp.com");
if(response == INVALID_HTTP_RESPONSE){
	printf("HTTP ERROR");
	return;
}

new Html:html = ResponseParseHtml(response);
if(html == INVALID_HTML_DOC){
	DeleteResponse(response);
	return;
}

new Selector:selector = ParseSelector("a");
if(selector == INVALID_SELECTOR){
	DeleteResponse(response);
	DeleteHtml(html);
	return;
}

new str[500],i;
while(GetNthElementAttrVal(html,selector,i,"href",str)){
	printf("%s",str);
	++i;
}
//delete created objects after the usage..
DeleteHtml(html);
DeleteResponse(response);
DeleteSelector(selector);
```

The same above with threaded http call would be

```Pawn
HttpGetThreaded(0,"MyHandler","https://wiki.sa-mp.com");
//...
forward MyHandler(playerid,Response:responseid);
public MyHandler(playerid,Response:responseid)
{
	
	if(responseid == INVALID_HTTP_RESPONSE){
		printf("HTTP ERROR");
		return 0;
	}

	new Html:html = ResponseParseHtml(responseid);
	if(html == INVALID_HTML_DOC){
		DeleteResponse(response);
		return 0;
	}

	new Selector:selector = ParseSelector("a");
	if(selector == INVALID_SELECTOR){
		DeleteResponse(response);
		DeleteHtml(html);
		return 0;
	}

	new str[500],i;
	while(GetNthElementAttrVal(html,selector,i,"href",str)){
		printf("%s",str);
		++i;
	}

	DeleteHtml(html);
	Delete(response);
	DeleteSelector(selector);
	return 1;
}


```

More examples can be found in [examples](examples)

## Note
The plugin is in primary stage and more tests and features needed to be added.I'm open to any kind of contribution, just open a pull request if you have anything to improve or add new features.

## Special thanks
* [Eva](https://github.com/ZOTTCE) for [samp-rust-sdk](https://github.com/ZOTTCE/samp-sdk)
* [Y_Less](https://github.com/Y-Less) for [y_tests](https://github.com/pawn-lang/YSI-Includes)
* Discord members in [SAMP discord channel](https://discord.me/page/samp)