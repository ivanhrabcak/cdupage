#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

#include "edupage.h"
#include "request.h"
#include "util.h"

const char* NO_CSRF_TOKEN = "000000000000000000000000000000000000000000000000000000000000000000000000";

Edupage edupage_init(HTTPRequestImpl* impl) {
    Edupage e;
    e.req = impl;
    e.is_logged_in = false;

    return e;
}

// returns -1 on error
int edupage_login(Edupage* edupage, char* subdomain, char* username, char* password) {
    HTTPRequestImpl* req = edupage->req;

    char* url = dsprintf("https://%s.edupage.org/login/index.php", subdomain);

    // the csrf token is 72 characters + \0 at the end;
    // allocated with malloc to free asap
    char *csrf_token = malloc(73 * sizeof(char));
    strcpy(csrf_token, NO_CSRF_TOKEN);

    void parse_csrf_from_stream(ResponseStream stream) {
        char* buf = (char*) stream.stream_part;

        int csrf_substr_pos = get_substring_pos("csrfauth", buf);
        if (csrf_substr_pos != -1) {
            csrf_substr_pos += 10;

            if (csrf_substr_pos + 72 > strlen(buf)) {
                return;
            }

            for (int i = csrf_substr_pos; i < csrf_substr_pos + 72; i++) {
                csrf_token[i - csrf_substr_pos] = buf[i];
            }
        }
    };

    int response_code = http_get(req, url, NULL, &parse_csrf_from_stream);
    if (strcmp(csrf_token, NO_CSRF_TOKEN) == 0 || response_code != 200) {
        return -1;
    }

    free(csrf_token);
    free(url);


    char* parameters = dsprintf("{\"username\":\"%s\",\"password\":\"%s\", \"csrfauth\":\"%s\"}", username, password, csrf_token);
    url = dsprintf("https://%s.edupage.org/login/edubarLogin.php", subdomain);

    HTTPHeader raw_headers[1];
    raw_headers[0] = http_create_header("Content-Type", "application/json");

    HTTPHeaders headers = http_create_header_list(raw_headers, 1); 

    int response_len = 0;

    void test_streamfunc(ResponseStream stream) {
        char* buf = (char*) stream.stream_part;

        printf("%s", buf);
    };

    

    response_code = http_post(req, url, &headers, parameters, &test_streamfunc);
    
    free(url);
}
