#pragma once
#include <vector>
#include <string>
#include <nex/glad.hpp>

namespace Nex {
    class ShaderProgramBuilder {
    public:    
        int add_shader(GLenum type, const std::string shader, std::string* log = 0);
        int add_vertex_shader(const std::string shader, std::string* log = 0);
        int add_fragment_shader(const std::string shader, std::string* log = 0);

        GLuint build();

        ~ShaderProgramBuilder();
    private:
        void cleanup();

        std::vector<GLuint> shaders;
    };
}
