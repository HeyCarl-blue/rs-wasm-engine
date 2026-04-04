#version 300 es
precision mediump float;

in vec3 v_normal;

struct DirectionalLight {
    vec3 direction;
    vec3 color;
};

const int MAX_LIGHTS = 8;
uniform DirectionalLight u_lights[MAX_LIGHTS];
uniform int u_light_count;
uniform vec3 u_ambient;
uniform vec3 u_albedo;

out vec4 out_color;

void main() {
    vec3 normal = normalize(v_normal);
    vec3 diffuse = vec3(0.0);

    for (int i = 0; i < u_light_count; i++) {
        float diff = max(dot(normal, -normalize(u_lights[i].direction)), 0.0);
        diffuse += u_lights[i].color * diff;
    }

    out_color = vec4(u_albedo * (u_ambient + diffuse), 1.0);
}
