cmake_minimum_required(VERSION 3.5)
project(cdupage C)

set(CMAKE_C_STANDARD 99)

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

set_target_properties(cdupage
    PROPERTIES
    PUBLIC_HEADER ${cdupage_HEADERS})
include(GNUInstallDirs)

set(INSTALL_CONFIGDIR ${CMAKE_INSTALL_LIBDIR}/cmake/MiniSat)

install(
    TARGETS
        cdupage
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    PUBLIC_HEADER DESTINATION include
)