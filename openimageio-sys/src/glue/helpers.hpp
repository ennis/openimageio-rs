#include <cstdlib>
#include <string>
#include "oiio.h"
#include <OpenImageIO/typedesc.h>

static inline OIIO_TypeDesc wrapTypeDesc(OIIO::TypeDesc typeDesc) {
    return OIIO_TypeDesc {
        typeDesc.basetype,
        typeDesc.aggregate,
        typeDesc.vecsemantics,
        0,
        typeDesc.arraylen
    };
}

static inline OIIO::TypeDesc unwrapTypeDesc(OIIO_TypeDesc typeDesc) {
    return OIIO::TypeDesc {static_cast<OIIO::TypeDesc::BASETYPE>(typeDesc.basetype),
                           static_cast<OIIO::TypeDesc::AGGREGATE >(typeDesc.aggregate),
                           static_cast<OIIO::TypeDesc::VECSEMANTICS >(typeDesc.vecsemantics),
                           typeDesc.arraylen};
}

char* makeCString(const std::string& str);
void freeCString(const char* ptr);

char** makeCharArray(int size);
void setArrayString(char **a, char *s, int n);
void freeCharArray(char **a, int size);

#define OIIO_CAST(ty,v) (reinterpret_cast<OIIO::ty*>(v))
#define OIIO_CAST_CONST(ty,v) (reinterpret_cast<OIIO::ty const*>(v))

