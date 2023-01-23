
precision mediump float;

varying vec3 v_position;
varying vec3 v_normal;
varying vec3 v_color;
varying mat4 light_dirn;

vec3 light_position = vec3(light_dirn * vec4(vec3(-000.0, -000.0, -100.0),1.0));
vec3 light_color = vec3(0.35, 0.35, 0.35);
vec3 ambient_color = vec3(0.3, 0.3, 0.3);
float shininess = 0.0;

void main()
{
    // Calculate ambient lighting
    vec3 ambient = v_color * 0.30;

    // Calculate diffuse lighting
    vec3 lightDirection = normalize(light_position - v_position);
    float diffuse = max(dot(v_normal, lightDirection), 0.0);
    vec3 diffuseColor = v_color * light_color * diffuse;

    // Calculate specular lighting
    vec3 viewDirection = normalize(-lightDirection);
    vec3 reflectDirection = reflect(-lightDirection, v_normal);
    float specular = pow(max(dot(viewDirection, reflectDirection), 0.01), shininess);
    vec3 specularColor = light_color * specular;

    // Combine ambient, diffuse, and specular lighting
    vec3 finalColor = ambient + diffuseColor + specularColor;

    gl_FragColor = vec4(finalColor, 1.0);
}
