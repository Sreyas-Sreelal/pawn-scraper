#include<a_samp>
#include<pawnscraper>

main(){
	
	new Response:response = HttpGet("https://wiki.sa-mp.com");
	if(response == INVALID_HTTP_RESPONSE){
		printf("HTTP ERROR");
		return;
	}

	new Html:html = ResponseParseHtml(response);
	if(html == INVALID_HTML_DOC){
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
	
	DeleteResponse(response);
	DeleteHtml(html);
	DeleteSelector(selector);

}
