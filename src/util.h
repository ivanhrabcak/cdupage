#ifndef _UTIL_H
#define _UTIL_H

#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>

#include "util.h"

// pseudocode of this function will explain it the best:
// if (haystack.contains(needle)) {
//     return haystack.indexOf(needle) + needle.length - 1;
// }
// else {
//     return -1;
// }
int get_substring_pos(char* needle, char* haystack) {
    int n = strlen(haystack);
    int m = strlen(needle);
    int* next = calloc(sizeof(int), m);

    //Compute next
    next[0] = -1;
    for(int i = 0; i < m; i++) {
        int back = next[i];
        while (back >= 0 && needle[i] != needle[back]) back = next[back];
        next[i + 1] = back + 1;
    }

    //Process the string
    int position = 0;
    for(int i = 0; i < n; i++) {
        while (position >= 0 && haystack[i] != needle[position]) position = next[position];
        position++;
        if (position == m) {
            free(next);
            return i;
        }
    }

    free(next);

    return -1;
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

#endif