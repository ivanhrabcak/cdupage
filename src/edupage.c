#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>
#include <stdarg.h>

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
    if(!haystack[0] && needle[0]) return -1;
    if(!needle[0]) return 0;

    int needle_position = -1;
    int needle_ind = 0;
    int backtrack_ind = -1;
    int i = -1;
    char current_char;

    while(current_char = haystack[++i]) {
        if(current_char == needle[needle_ind] || current_char == needle[needle_ind = 0]) {
            if(needle_ind && backtrack_ind == -1 && current_char == needle[0]) backtrack_ind = i;
            if(!needle_ind++) needle_position = i;
            if(!needle[needle_ind]) break;
        } else if(i >= backtrack_ind && backtrack_ind != -1) {
            i = backtrack_ind - 1;
            backtrack_ind = -1;
        }
    }

    return needle_ind && !needle[needle_ind] ? needle_position + needle_ind : -1;
}

/**
 * @brief Dynamicly allocates memory for string to be formatted.
 * @param format Format string followed by optional format specifiers.
 * @return Formatted string.
 **/
char* dsprintf(char* format, ...) {
    va_list args;
    va_start(args, format);

    int len = vsnprintf(NULL, 0, format, args);
    char* str = malloc(len + 1);
    vsnprintf(str, len + 1, format, args);

    va_end(args);

    return str;
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
