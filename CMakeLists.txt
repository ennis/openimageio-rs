cmake_minimum_required(VERSION 3.8)
project(openimageio_rs)

set(CMAKE_CXX_STANDARD 11)

file(GLOB SOURCE_FILES
        openimageio-sys/src/glue/helpers.cpp
        openimageio-sys/src/glue/imageinput.cpp
        openimageio-sys/src/glue/imageoutput.cpp
        openimageio-sys/src/glue/imagespec.cpp
        openimageio-sys/src/glue/imagecache.cpp)

add_executable(openimageio_rs ${SOURCE_FILES})
target_include_directories(openimageio_rs PRIVATE "C:\\Users\\Alexandre\\Developpement\\vcpkg\\installed\\x64-windows\\include\\")