// this is a curl implementation for HTTPRequestImpl
// if you implement HTTPRequestImpl you can use this library anywhere (where there's internet)

#include "request.h"
#include <stdio.h>
#include <stdlib.h>
#include <curl/curl.h>
#include <string.h>

#define len(x) sizeof(x) / sizeof(x[0])

size_t write_function(void* ptr, size_t size, size_t nmemb, ResponseStream* data) {
    data->stream_part = ptr;
    data->next(*data);

    return size * nmemb;
}

int curl_get(HTTPRequestImpl* req, char* url, HTTPHeader* headers, void (*streamfunc)(ResponseStream)) {
    CURL* curl = (CURL*) req->http_client;

    curl_easy_setopt(curl, CURLOPT_URL, url);

    if (headers != NULL) {
        struct curl_slist* header_list = NULL;
        for (int i = 0; i < len(headers); i++) {
            HTTPHeader currentHeader = headers[i];

            // we need to account for the :, the space and \0 at the end of the string
            char header_string[strlen(currentHeader.key) + strlen(currentHeader.value) + 3];

            // key: value
            sprintf(header_string, "%s: %s", currentHeader.key, currentHeader.value);
            
            header_list = curl_slist_append(header_list, header_string);
        }

        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, header_list);
    }
    

    ResponseStream stream;
    stream.next = streamfunc;

    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_function);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &stream);

    CURLcode err = curl_easy_perform(curl);
    if (!err) {
        long response_code;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &response_code);
        
        return (int) response_code;
    }
    else {
        return -1;
    }
    
    
}

int curl_post(HTTPRequestImpl* req, char* url, HTTPHeader* headers, char* post_data, void (*streamfunc)(ResponseStream)) {
    CURL* curl = (CURL*) req->http_client;

    curl_easy_setopt(curl, CURLOPT_URL, url);

    if (headers != NULL) {
        struct curl_slist* header_list = NULL;
        for (int i = 0; i < len(headers); i++) {
            HTTPHeader currentHeader = headers[i];

            // we need to account for the :, the space and \0 at the end of the string
            char header_string[strlen(currentHeader.key) + strlen(currentHeader.value) + 3];

            // key: value
            sprintf(header_string, "%s: %s", currentHeader.key, currentHeader.value);
            
            header_list = curl_slist_append(header_list, header_string);
        }

        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, header_list);
    }
    

    ResponseStream stream;
    stream.next = streamfunc;

    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_function);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &stream);

    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, post_data);

    CURLcode err = curl_easy_perform(curl);
    if (!err) {
        long response_code;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &response_code);
        
        return (int) response_code;
    }
    else {
        return -1;
    }
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
    return curl_impl;
}

// DO NOT PASS instances of HTTPRequestImpl that were not created with http_init
void http_free(HTTPRequestImpl* req) {
    curl_easy_cleanup(req->http_client);
    req->http_client = NULL;
    free(req);
}