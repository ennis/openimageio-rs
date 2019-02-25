//
// Created by Alexandre on 23/01/2019.
//
#include "helpers.hpp"

char* makeCString(const std::string& str)
{
    auto n = str.size();
    char* ptr = new char[n+1];
    strncpy(ptr, str.c_str(), n+1);
    return ptr;
}

void freeCString(const char *ptr) {
    delete[] ptr;
}

static char** makeCharArray(int size) {
    return (char**)calloc(sizeof(char*), size);
}

static void setArrayString(char **a, char *s, int n) {
    a[n] = s;
}

static void freeCharArray(char **a, int size) {
    int i;
    for (i = 0; i < size; i++)
        free(a[i]);
    free(a);
}
