#include <nex/glad.hpp>
#include <nex/renderer.hpp>
#include <nex/shader_program_builder.hpp>

namespace Nex {
    int build_sprite_shader() {
        const char* vertexShaderSource = R"(
            #version 330 core

            layout (location = 0) in vec3 vertex;
            layout (location = 1) in vec2 texCoord;

            out vec2 UV;

            void main() {
                gl_Position = vec4(vertex, 1.0);
                UV = texCoord;
            }
        )";

        const char* fragmentShaderSource = R"(
            #version 330 core

            in vec2 UV;

            uniform sampler2D sprite;

            void main() {
                gl_FragColor = texture(sprite, UV);
            }
        )";

        Nex::ShaderProgramBuilder programBuilder;
        programBuilder.add_vertex_shader(vertexShaderSource);
        programBuilder.add_fragment_shader(fragmentShaderSource);
        return programBuilder.build();
    }

    // Used for preparing VBO and VAO
    std::tuple<GLuint, GLuint, GLuint> build_sprite_vabo() {
        // Quad
        float vertices[] = {
            // X, Y, Z           // U, V
            -0.5f, -0.5f, 1.0f,  1.0f, 1.0f, // left-down
             0.5f, -0.5f, 1.0f,  0.0f, 1.0f, // right-down
            -0.5f,  0.5f, 1.0f,  1.0f, 0.0f, // left-top

             0.5f, -0.5f, 1.0f,  0.0f, 1.0f, // right-down
             0.5f,  0.5f, 1.0f,  0.0f, 0.0f, // right-top
            -0.5f,  0.5f, 1.0f,  1.0f, 0.0f // left-top
        };

        GLuint VAO, VBO, instanceVBO;
        glGenVertexArrays(1, &VAO);
        glGenBuffers(1, &VBO);
        glGenBuffers(1, &instanceVBO);

        glBindVertexArray(VAO);

        // VBO
        glBindBuffer(GL_ARRAY_BUFFER, VBO);
        glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 5 * sizeof(float), (void*)0);
        glEnableVertexAttribArray(0);

        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(float), (void*)(3 * sizeof(float)));
        glEnableVertexAttribArray(1);

        // instanceVBO
        glBindBuffer(GL_ARRAY_BUFFER, instanceVBO);
        glVertexAttribPointer(2, 2, GL_FLOAT, GL_FALSE, 2 * sizeof(float), (void*)0);
        glEnableVertexAttribArray(2);
        glVertexAttribDivisor(2, 1);

        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindVertexArray(0);

        return std::make_tuple(VBO, VAO, instanceVBO);
    }

    int Renderer::initialize_render(std::string* log) {
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        // Build Draw Shader
        drawShader = build_sprite_shader();

        // Build VBO, VAO
        auto [VBO, VAO, instanceVBO] = build_sprite_vabo();
        this->VBO = VBO;
        this->VAO = VAO;
        this->instanceVBO = instanceVBO;

        return 0;
    }

    void Renderer::draw_sprite(GLuint textureID) {
        GLint uniform = glGetUniformLocation(drawShader, "sprite");
        glUseProgram(drawShader);
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, textureID);
        glUniform1i(uniform, 0);

        draw_call();
    }

    void Renderer::draw_call() {
        glUseProgram(drawShader);
        glBindVertexArray(VAO);
        glDrawArrays(GL_TRIANGLES, 0, 6);
    }

    void Renderer::clear_color() {
        glClearColor(0.2f, 0.3f, 0.1f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);
    }

    // Maybe will be added in future as part of dummy methods

    int build_triangle_shader() {
        const char* vertexShaderSource = R"(
            #version 330 core
            layout (location = 0) in vec3 aPos;
            void main() {
                gl_Position = vec4(aPos, 1.0);
            }
        )";
        const char* fragmentShaderSource = R"(
            #version 330 core
            out vec4 FragColor;
            void main() {
                FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f); // orange
            }
        )";

        Nex::ShaderProgramBuilder programBuilder;
        programBuilder.add_vertex_shader(vertexShaderSource);
        programBuilder.add_fragment_shader(fragmentShaderSource);
        return programBuilder.build();
    }

    std::pair<GLuint, GLuint> build_triangle_vabo() {
        // Triangle
        float vertices[] = {
            // X, Y, Z
            -0.5f, -0.5f, 0.0f,
            0.5f, -0.5f, 0.0f,
            0.0f,  0.5f, 0.0f
        };

        GLuint VBO, VAO;
        glGenVertexArrays(1, &VAO);
        glGenBuffers(1, &VBO);

        glBindVertexArray(VAO);
        glBindBuffer(GL_ARRAY_BUFFER, VBO);
        glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
        glEnableVertexAttribArray(0);

        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindVertexArray(0);

        return std::make_pair(VBO, VAO);
    }
}
