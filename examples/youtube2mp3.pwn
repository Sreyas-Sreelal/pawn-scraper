#include<a_samp>
#include<pawnscraper>
#include<zcmd>

main(){
	
	
}

CMD:yt2mp3(playerid,params[]){
	Yt2Mp3(playerid,params);	
	return 1;
}

Yt2Mp3(playerid,id[]){
	new link[70];
	format(link,sizeof(link),"https://www.easy-youtube-mp3.com/download.php?v=%s",id);

	new Response:response = HttpGet(link);
	if(response == SCRAPER_HTTP_ERROR){
		printf("HTTP ERROR");
		return;
	}

	new Html:html = ResponseParseHtml(response);
	if(html==Html:-1){
		printf("Error on creating html instance");
		DeleteResponse(response);
		return;
	}

	new 
		Selector:link_selector = ParseSelector(".btn.btn-success"),
		Selector:name_selector = ParseSelector("title");

	if(link_selector == INVALID_SELECTOR || name_selector == INVALID_SELECTOR){
		printf("Error on creating selector");
		DeleteResponse(response);
		return;
	}

	new str[500],i,check,name[100];
	
	GetNthElementText(html,name_selector,0,name);
	
	while((check = GetNthElementAttrVal(html,link_selector,i,"href",str))!=0){
		printf("%s",str);
		if(check != INVALID_ATTRIBUTE)
			break;
		++i;
	}
	if(!check){
		SendClientMessage(playerid,-1,"Can't play song right now ");
	}else{
		
		if(name[0] != '\0' )
			SendClientMessage(playerid,-1,name);

		PlayAudioStreamForPlayer(playerid,str);
	}
	if(!DeleteResponse(response)){
		print("[WARNING] Response cache couldn't removed");
	}
}
