#include <nex/glad.hpp>
#include <nex/renderer.hpp>

namespace Nex {
    void framebuffer_size_callback(GLFWwindow* window, int width, int height) {
        glViewport(0, 0, width, height);
    }

    int Renderer::create_window(std::string* log) {
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

        GLFWwindow* rawWindow = glfwCreateWindow(800, 800, "CoreCollapse", nullptr, nullptr);

        if (!rawWindow) {
            if (log) {
                log->assign("Window creation failed!\n");
            }

            glfwTerminate();
            return -1;
        }

        Renderer::window = std::shared_ptr<GLFWwindow>(
            rawWindow,
            [](GLFWwindow* w) { 
                glfwDestroyWindow(w); 
            }
        );

        glfwMakeContextCurrent(window.get());
        glfwSetFramebufferSizeCallback(window.get(), framebuffer_size_callback);

        return 0;
    };

    GLFWwindow* Renderer::get_window() {
        return Renderer::window.get();
    }
}