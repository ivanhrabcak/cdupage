#include <curl/curl.h>
#include <stdbool.h>

#ifndef _EDUPAGE_H
#define _EDUPAGE_H

typedef struct Edupage {
    CURL *curl;
    bool is_logged_in;
} Edupage;


void edupage_init(Edupage* edupage);
int edupage_login(Edupage* edupage);

#endif