# pawn-scraper
A powerful scraper plugin that provides interface for utlising html_parsers and css selectors in pawn.

## Building
* Clone the repo

`git clone https://github.com/Sreyas-Sreelal/pawn-scraper.git`
* Compile the plugin using nightly compiler
 
  * **Windows**
	
	`cargo +nightly-i686-pc-windows-msvc build --release`
  * **Linux**
	
	`cargo +nightly-i686-unknown-linux-gnu build --release`

* Add [pawnscraper.inc](includes/pawnscraper.inc) in includes folder

## Example Usage
A small example to fetch all links in wiki.sa-mp.com 
```Pawn
new Response:response = HttpGet("https://wiki.sa-mp.com");
if(response == SCRAPER_HTTP_ERROR){
	printf("HTTP ERROR");
	return;
}

new Html:html = ResponseParseHtml(response);
if(html == INVALID_HTML_DOC){
	return;
}

new Selector:selector = ParseSelector("a");
if(selector == INVALID_SELECTOR){
	return;
}

new str[500],i;
while(GetNthElementAttrVal(html,selector,i,"href",str)){
	printf("%s",str);
	++i;
}


```
More examples can be found in [examples](examples)