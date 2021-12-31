#ifndef _REQUEST_H
#define _REQUEST_H

typedef struct ResponseStream ResponseStream;

// we use streams to make sure this library can run on 
// hardware with little memory 
// (if we have streams we don't have to store the whole response at once)
typedef struct ResponseStream {
    char* stream_part;
    void (*next) (ResponseStream);
} ResponseStream;

typedef struct HTTPHeader {
    char* key;
    char* value;
} HTTPHeader;

typedef struct HTTPRequestImpl HTTPRequestImpl;

typedef struct HTTPRequestImpl {
    //                             url,   headers,               output stream function
    int (*get) (HTTPRequestImpl*, char*, HTTPHeader*, void (*streamfunc)(ResponseStream));
    //                             url,   headers,   post data,  output stream function
    int (*post) (HTTPRequestImpl*, char*, HTTPHeader*, char*, void (*streamfunc)(ResponseStream));
    
    // additional field for storing a http client 
    // (curl instance or any other client from a http library)
    void* http_client;
} HTTPRequestImpl;

// constructor
HTTPRequestImpl* http_init();

// convinience macros

#define http_get(req, url, headers, streamfunc) req->get(req, url, headers, streamfunc)
#define http_post(req, url, headers, post_data, streamfunc) req->post(req, url, headers, post_data, streamfunc);

// destructor
void http_free(HTTPRequestImpl* req);

#endif