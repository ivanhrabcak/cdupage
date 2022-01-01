#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

#include "edupage.h"
#include "request.h"

const char* NO_CSRF_TOKEN = "000000000000000000000000000000000000000000000000000000000000000000000000";

Edupage edupage_init(HTTPRequestImpl* impl) {
    Edupage e;
    e.req = impl;
    e.is_logged_in = false;

    return e;
}

// pseudocode of this function will explain it the best:
// if (haystack.contains(needle)) {
//     return haystack.indexOf(needle) + needle.length - 1;
// }
// else {
//     return -1;
// }
int get_substring_pos(char* needle, char* haystack) {
    int needle_position = -1;

    int needle_len = strlen(needle);
    int needle_ind = 0;

    for (int i = 0; i < strlen(haystack); i++) {
        char current_char = haystack[i];

        if (current_char == needle[needle_ind]) {
            needle_ind++;

            if (needle_ind == needle_len) {
                needle_position = i;
                break;
            }
        }
        else {
            needle_ind = 0;
        }
    }

    return needle_position;
}

// returns -1 on error
int edupage_login(Edupage* edupage, char* subdomain, char* username, char* password) {
    HTTPRequestImpl* req = edupage->req;

    char url[50];
    sprintf(url, "https://%s.edupage.org/login/index.php", subdomain);

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

    // we can hardcode the json instead of generating it, this won't waste our memory
    // this has a disadvantage - it is pretty dangerous as there is little padding in the
    // parameters string (to not waste memory)
    char parameters[512];
    sprintf(parameters, "{\"username\":\"%s\",\"password\":\"%s\", \"csrfauth\":\"%s\"}", username, password, csrf_token);

    sprintf(url, "https://%s.edupage.org/login/edubarLogin.php", subdomain);

    HTTPHeader raw_headers[1];
    raw_headers[0] = http_create_header("Content-Type", "application/json");

    HTTPHeaders headers = http_create_header_list(raw_headers, 1); 

    int response_len = 0;

    void test_streamfunc(ResponseStream stream) {
        char* buf = (char*) stream.stream_part;

        printf("%s", buf);
    };

    

    response_code = http_post(req, url, &headers, parameters, &test_streamfunc);
    
}