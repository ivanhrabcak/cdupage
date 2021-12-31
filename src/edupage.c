#include <curl/curl.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include "edupage.h"


// CURL response struct and write function

typedef struct Response {
    size_t size;
    char* data;
} Response;

size_t write_data(void *ptr, size_t size, size_t nmemb, Response *data) {
    size_t index = data->size;
    size_t n = (size * nmemb);
    char* tmp;

    data->size += (size * nmemb);

    tmp = realloc(data->data, data->size + 1); /* +1 for '\0' */

    if(tmp) {
        data->data = tmp;
    } else {
        if(data->data) {
            free(data->data);
        }
        fprintf(stderr, "Failed to allocate memory.\n");
        return 0;
    }

    memcpy((data->data + index), ptr, n);
    data->data[data->size] = '\0';

    return size * nmemb;
}

void edupage_init(Edupage* edupage) {
    printf("init\n");
}

int edupage_login(Edupage* edupage) {
    CURL* curl;
    CURLcode result;

    struct Response data;
    data.size = 0;
    
    size_t bufsize = sizeof(uint8_t) * (1024 * 1024) * 20; // 20 MB
    data.data = malloc(bufsize);

    curl = curl_easy_init();
    if (!curl) {
        return 0;
    }

    curl_easy_setopt(curl, CURLOPT_URL, "https://gymlsba.edupage.org/login/index.php");
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &data);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_data);
    
    result = curl_easy_perform(curl);

    if (result != CURLE_OK) {
        return 0;
    }

    printf("First request success\n");
    
    data.size = 0;
    data.data = malloc(bufsize);

    curl_easy_setopt(curl, CURLOPT_URL, "https://www.google.com");
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &data);
    
    result = curl_easy_perform(curl);

    if (result != CURLE_OK) {
        return 0;
    }

    curl_easy_cleanup(curl);
    printf("Second request success\n");
}