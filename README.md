# pawn-scraper
A powerful plugin to that helps to do web scraping in pawn.

## Example
A small example to fetch all links in wiki.sa-mp.com 
```Pawn
new Response:response = HttpGet("https://wiki.sa-mp.com");
if(response == SCRAPER_HTTP_ERROR){
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
    return;
}

new str[500],i;
while(GetNthElementAttrVal(html,selector,i,"href",str)){
    printf("%s",str);
    ++i;
}

DeleteResponse(response);

```
More examples can be found in [examples](examples)