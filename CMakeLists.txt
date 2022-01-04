cmake_minimum_required(VERSION 3.5)
project(cdupage C)

set(CMAKE_C_STANDARD 11)

file(GLOB cdupage_SRC
    "src/**.h"
    "src/**.c"
)

file(GLOB cdupage_HEADERS
    "src/**.h"
)

add_library(
    cdupage
    ${cdupage_SRC}
)

find_package(CURL REQUIRED)
target_link_libraries(cdupage PRIVATE)

target_include_directories(cdupage
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>)

install(FILES ${cdupage_HEADERS} DESTINATION include/cdupage)
include(GNUInstallDirs)

install(
    TARGETS
        cdupage
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
)