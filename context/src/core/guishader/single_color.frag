#version 450

layout (set = 0, binding = 0) uniform Color {
	vec3 val;
} color;

layout (location = 0) out vec4 out_color;

void main()
{
	out_color = vec4(color.val, 1.0);
}
