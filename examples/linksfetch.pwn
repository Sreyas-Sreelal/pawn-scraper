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
}
