#include <nex/glad.hpp>
#include <GLFW/glfw3.h>
#include <iostream>
#include <string>
#include <nex/renderer.hpp>
#include <nex/shader_program_builder.hpp>

int main() {
    auto renderer = std::make_shared<Nex::Renderer>();

    std::string error_log;

    // Pre-Initialize
    if (renderer->pre_initialize(&error_log) != 0)
    {
        std::cerr << error_log << std::endl;
        return -1;
    }

    // Initialization
    if (renderer->create_window(&error_log) != 0)
    {
        std::cerr << error_log << std::endl;
        return -1;
    }

    if (renderer->initialize(&error_log) != 0)
    {
        std::cerr << error_log << std::endl;
        return -1;
    }

    // Main Cycle
    GLFWwindow* window = renderer->get_window();

    GLuint textureID = renderer->load_texture("assets/full.png");

    while (!glfwWindowShouldClose(window)) {
        renderer->clear_color();

        renderer->draw_sprite(textureID);

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    // End
    renderer->cleanup();
    glfwTerminate();

    return 0;
}
