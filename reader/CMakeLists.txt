cmake_minimum_required(VERSION 3.20)

project(zebra-reader)

add_executable(${PROJECT_NAME}
	src/RFIDSample4App.cpp
	src/common.cpp
)

set(VENDOR "${CMAKE_CURRENT_SOURCE_DIR}/vendor/zebra")
set(CMAKE_SKIP_RPATH TRUE)

include_directories("${VENDOR}/lib/include/")
include_directories("${CMAKE_CURRENT_SOURCE_DIR}/src")

# Search system libs
find_library(ssh2 NAMES ssh2 REQUIRED)
find_library(pthread NAMES pthread REQUIRED)
find_library(z NAMES z REQUIRED)
find_library(rt NAMES rt REQUIRED)
find_library(xml2 NAMES xml2 REQUIRED)

# Search provided SDK libs
set(SDK_LIBS "${VENDOR}/lib/x86_64")
find_library(ltk NAMES libltk.so PATHS ${SDK_LIBS} REQUIRED)
find_library(rfidapi32 NAMES librfidapi32.so PATHS ${SDK_LIBS} REQUIRED)

add_compile_definitions(UNICODE)

target_link_libraries(${PROJECT_NAME} ${ssh2} ${ltk} ${pthread} ${z} ${rt} ${xml2} ${rfidapi32})
