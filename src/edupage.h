#ifndef _EDUPAGE_H
#define _EDUPAGE_H

#include <stdbool.h>
#include "request.h"

typedef struct Edupage {
    HTTPRequestImpl* req;
    bool is_logged_in;
    char* PHPSESSID;
} Edupage;

Edupage edupage_init(HTTPRequestImpl *impl);
int edupage_login(Edupage* edupage, char* subdomain, char* username, char* password);

#endif