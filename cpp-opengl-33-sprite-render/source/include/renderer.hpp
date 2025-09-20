#pragma once
#include <glfw/glfw3.h>
#include <string>
#include <memory>

namespace Nex {
    class Renderer {
    public:
        Renderer() = default;
        ~Renderer();

        // Methods

        // Life-Cycle

        /**
         * @brief Pre-Initialize Step
         */
        int pre_initialize(std::string* log = 0);
        /**
         * @brief Initialize Step
         */
        int initialize(std::string* log = 0);
        /**
         * @brief Post-Initialize Step
         */
        int post_initialize(std::string* log = 0);

        // Window
        /**
         * @brief Create GLFW window
         */
        int create_window(std::string* log = 0);
        GLFWwindow* get_window();

        // Shaders

        // Render
        void draw_sprite(GLuint textureID);
        void clear_color();

        // Helpers

        // TODO: rewrite texture loading to use batching
        // TODO: add texture loading options like filtration and etc
        /**
         * @brief Reads texture from FS by specified path and loads it into GPU
         * @return ID of loaded texture in GPU
         */
        GLuint load_texture(std::string path);

        void cleanup();

        // Properties
    private:
        // Methods

        // Render
        /**
         * @brief Internal initialization step of everything directly related to rendering itself (shaders compilation, VBO, VAO and etc), should be called from @ref Renderer::initialize()
         */
        int initialize_render(std::string* log = 0);
        void draw_call();

        // Properties

        // Render

        GLuint drawShader;

        GLuint VAO;
        GLuint VBO;
        GLuint instanceVBO;

        // Window
        std::shared_ptr<GLFWwindow> window;
    };
}
