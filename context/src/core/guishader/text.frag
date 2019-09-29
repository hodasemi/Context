#version 450

layout (location = 0) in vec2 uv;

layout (set = 0, binding = 0) uniform sampler2D image;

layout (set = 1, binding = 0) uniform TextColor {
	vec3 color;
} text_color;

layout (location = 0) out vec4 color;

void main()
{
	color = texture(image, uv).r * vec4(text_color.color, 1.0f);
}
