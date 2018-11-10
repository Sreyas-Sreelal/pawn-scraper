#include<a_samp>
#include<a_http>

#include<pawnscraper>

main(){
	new Selector:selector = ParseSelector("a");
	if(selector == Selector:-1){
		printf("Error on creating selector");
		return;
	}
	new Response:response = HttpGet("https://wiki.sa-mp.com");

	new Html:html = ResponseParseHtml(response);
	if(html==Html:-1){
		printf("Error on creating html instance");
		return;
	}
	new str[500];
	for(new i;GetNthElementAttrVal(html,selector,i,"href",str)!=0;++i){
		printf("%s",str);
	}
	DeleteResponse(response);
}
