#include<a_samp>
#include<pawnscraper>

main(){
	
	new Response:response = HttpGet("https://wiki.sa-mp.com");
	if(response == SCRAPER_HTTP_ERROR){
		printf("HTTP ERROR");
		return;
	}

	new Html:html = ResponseParseHtml(response);
	if(html==Html:-1){
		printf("Error on creating html instance");
		return;
	}

	new Selector:selector = ParseSelector("a");
	if(selector == INVALID_SELECTOR){
		printf("Error on creating selector");
		return;
	}

	new str[500],i;
	while(GetNthElementAttrVal(html,selector,i,"href",str)){
		printf("%s",str);
		++i;
	}

	if(!DeleteResponse(response)){
		print("[WARNING] Response cache couldn't removed");
	}
}
