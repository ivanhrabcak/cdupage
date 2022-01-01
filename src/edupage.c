#include "edupage.h"
#include <stdio.h>
#include "request.h"

Edupage edupage_init(HTTPRequestImpl* impl) {
    Edupage e;
    e.req = impl;
    e.is_logged_in = false;

    return e;
}

void streamfunc(ResponseStream response_stream) {
    printf("%s", response_stream.stream_part);
}

int edupage_login(Edupage* edupage) {
    
}