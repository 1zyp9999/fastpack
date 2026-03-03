cmake_minimum_required(VERSION 3.10)
project(ExampleApp VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(example_app src/main.cpp)

install(TARGETS example_app DESTINATION bin)
install(FILES README.md DESTINATION share/example_app)