#include <nex/glad.hpp>
#include <nex/renderer.hpp>

namespace Nex {
    int Renderer::pre_initialize(std::string* log) {
        if (!glfwInit()) {
            if (log) {
                log->assign("GLFW init failed!\n");
            }
            return -1;
        }

        return 0;
    }

    int Renderer::initialize(std::string* log) {
        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
            if (log) {
                log->assign("GLAD init failed!\n");
            }
            return -1;
        }

        auto renderInitialize = initialize_render(log);
        if (renderInitialize != 0) {
            return renderInitialize;
        }

        return 0;
    }

    void Renderer::cleanup() {
        if (VAO)
            glDeleteVertexArrays(1, &VAO);
        if (VBO)
            glDeleteBuffers(1, &VBO);
        if (drawShader) {
            glDeleteProgram(drawShader);
            drawShader = 0;
        }
    }

    Renderer::~Renderer() {
        cleanup();
    }
}
