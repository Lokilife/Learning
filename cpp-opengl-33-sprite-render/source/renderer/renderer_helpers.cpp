#include <nex/glad.hpp>
#include <iostream>
#include <nex/renderer.hpp>
#define STB_IMAGE_IMPLEMENTATION
#include <stb_image.h>

namespace Nex {
    GLuint Renderer::load_texture(std::string path) {
        GLuint textureID = 0;

        int width, height, channels;
        unsigned char* data = stbi_load(path.c_str(), &width, &height, &channels, 0);

        std::cout << "Loaded texture with " << channels << " channels;\n";

        if (data) {
            GLenum format = channels == 4 ? GL_RGBA : GL_RGB;

            glGenTextures(1, &textureID);

            glBindTexture(GL_TEXTURE_2D, textureID);
            glTexImage2D(GL_TEXTURE_2D, 0, format, width, height, 0, format, GL_UNSIGNED_BYTE, data);
            glGenerateMipmap(GL_TEXTURE_2D);

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
        }
        else {
            // TODO: error handling
        }
        stbi_image_free(data);

        return textureID;
    }
}
