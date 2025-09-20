cmake_minimum_required(VERSION 3.25)
project(GLLearn LANGUAGES C CXX)

set(CMAKE_TOOLCHAIN_FILE "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake" CACHE STRING "")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(Game
    source/main.cpp
    source/renderer/renderer.cpp
    source/renderer/renderer_window.cpp
    source/renderer/renderer_render.cpp
    source/renderer/renderer_helpers.cpp
    source/renderer/shader_program_builder.cpp
    source/glad/src/glad.c
)

target_include_directories(Game PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/source/glad/include"
    "${CMAKE_CURRENT_SOURCE_DIR}/source/include"
)

# Dependencies
# GLFW3
find_package(glfw3 CONFIG REQUIRED)
target_link_libraries(Game PRIVATE glfw)
# GLM
find_package(glm CONFIG REQUIRED)
target_link_libraries(Game PRIVATE glm::glm)
# stb
find_package(Stb REQUIRED)
target_include_directories(Game PRIVATE ${Stb_INCLUDE_DIR})

target_link_libraries(Game PRIVATE
    opengl32
)
