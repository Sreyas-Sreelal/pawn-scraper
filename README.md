# pawn-scraper
[![Build Status](https://travis-ci.org/Sreyas-Sreelal/pawn-scraper.svg?branch=master)](https://travis-ci.org/Sreyas-Sreelal/pawn-scraper)
[![Build status](https://ci.appveyor.com/api/projects/status/5rq55kukvy8xymly?svg=true)](https://ci.appveyor.com/project/Sreyas-Sreelal/pawn-scraper)
[![sampctl-supported](https://shields.southcla.ws/badge/sampctl-PawnScraper-2f2f2f.svg)](https://github.com/Sreyas-Sreelal/pawn-scraper)
[![GitHub issues](https://img.shields.io/github/issues/Sreyas-Sreelal/pawn-scraper.svg)]() [![GitHub pull requests](https://img.shields.io/github/issues-pr-raw/sreyas-sreelal/pawn-scraper.svg)]() [![GitHub pull license](https://img.shields.io/github/license/sreyas-sreelal/pawn-scraper.svg)]() 


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

* Compile the plugin using nightly compiler
 
  * **Windows**
	
	`cargo +nightly-i686-pc-windows-msvc build --release`
  * **Linux**
	
	`cargo +nightly-i686-unknown-linux-gnu build --release`


## Example Usage
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
The plugin is in primary stage and more tests and features needed to be added.I'm open to any kind of contribution, just open a pull request if you have anything to improve or add new features.This plugin was written inorder to get around with rust and sharpen my skills in it.

## Special thanks
* [Eva](https://github.com/ZOTTCE) for [samp-rust-sdk](https://github.com/ZOTTCE/samp-sdk)
* [Y_Less](https://github.com/Y-Less) for [y_tests](https://github.com/pawn-lang/YSI-Includes) 
* Discord members in [SAMP discord channel](https://discord.me/page/samp)