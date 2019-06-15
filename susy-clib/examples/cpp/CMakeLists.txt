cmake_minimum_required(VERSION 3.5)
include(ExternalProject)
include_directories("${CMAKE_SOURCE_DIR}/../..")
set (CMAKE_CXX_STANDARD 11) # Enfore C++11
add_executable(susy-example main.cpp)

ExternalProject_Add(
    libsusy
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    COMMAND cargo build -p susy-clib      # Note: use --release in a real project
    BINARY_DIR "${CMAKE_SOURCE_DIR}/../../../target"
    INSTALL_COMMAND ""
    LOG_BUILD ON)

add_dependencies(susy-example libsusy)
target_link_libraries(susy-example "${CMAKE_SOURCE_DIR}/../../../target/debug/${CMAKE_SHARED_LIBRARY_PREFIX}susy${CMAKE_SHARED_LIBRARY_SUFFIX}")
