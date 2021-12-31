#include <stdbool.h>
#include "request.h"

#ifndef _EDUPAGE_H
#define _EDUPAGE_H

typedef struct Edupage {
    HTTPRequestImpl* req;
    bool is_logged_in;
} Edupage;


Edupage edupage_init(HTTPRequestImpl *impl);
int edupage_login(Edupage* edupage);

#endif