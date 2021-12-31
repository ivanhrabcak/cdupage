#include "edupage.h"

Edupage edupage_init(HTTPRequestImpl* impl) {
    Edupage e;
    e.req = impl;
    e.is_logged_in = false;

    return e;
}

int edupage_login(Edupage* edupage) {
    
}