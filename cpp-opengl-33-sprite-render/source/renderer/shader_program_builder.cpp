#include <nex/shader_program_builder.hpp>

namespace Nex {
    // Public API
    int ShaderProgramBuilder::add_shader(GLenum type, const std::string source, std::string* log) {
        const char* src = source.c_str();

        unsigned int shader = glCreateShader(type);
        glShaderSource(shader, 1, &src, NULL);
        glCompileShader(shader);

        // TODO: add check for success compilation

        shaders.push_back(shader);

        return 0;
    }

    int ShaderProgramBuilder::add_vertex_shader(const std::string shader, std::string* log) {
        return add_shader(GL_VERTEX_SHADER, shader, log);
    }

    int ShaderProgramBuilder::add_fragment_shader(const std::string shader, std::string* log) {
        return add_shader(GL_FRAGMENT_SHADER, shader, log);
    }

    GLuint ShaderProgramBuilder::build() {
        GLuint program = glCreateProgram();

        for (auto shader : shaders) {
            glAttachShader(program, shader);
        }

        glLinkProgram(program);

        // TODO: add check for success linking

        cleanup();

        return program;
    }

    ShaderProgramBuilder::~ShaderProgramBuilder() {
        cleanup();
    }

    // Private API

    void ShaderProgramBuilder::cleanup() {
        for (auto shader : shaders) {
            glDeleteShader(shader);
        }
        shaders.clear();
    }
}
