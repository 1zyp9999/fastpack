cmake_minimum_required(VERSION 3.15)
project(ConanExample VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(fmt REQUIRED)
find_package(nlohmann_json REQUIRED)

add_executable(conan_example src/main.cpp)

target_link_libraries(conan_example PRIVATE fmt::fmt nlohmann_json::nlohmann_json)

install(TARGETS conan_example DESTINATION bin)