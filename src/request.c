// this is a curl implementation for HTTPRequestImpl
// if you implement HTTPRequestImpl you can use this library anywhere (where there's internet)

#include "request.h"
#include <stdio.h>
#include <stdlib.h>
#include <curl/curl.h>


int curl_get(HTTPRequestImpl* req, char* url, HTTPHeader* headers, void (*streamfunc)(ResponseStream)) {
    printf("get");
}

int curl_post(HTTPRequestImpl* req, char* url, HTTPHeader* headers, char* post_data, void (*streamfunc)(ResponseStream)) {
    printf("post");
}


HTTPRequestImpl* http_init() {
    HTTPRequestImpl *curl_impl = malloc(sizeof(HTTPRequestImpl));

    CURL* curl = curl_easy_init();
    if (!curl) {
        // fuck
        return NULL;
    }

    curl_impl->get = &curl_get;
    curl_impl->post = &curl_post;
    curl_impl->http_client = curl;

    // returning address of a variable 
    // from a local function is ok here
    // because it was allocated with malloc -> it needs to be freed
    return &curl_impl;
}

// DO NOT PASS instances of HTTPRequestImpl that were not created with http_init
void http_free(HTTPRequestImpl* req) {
    curl_easy_cleanup(req->http_client);
    free(req);
}