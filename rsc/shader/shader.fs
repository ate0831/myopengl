#version 140

//3Dオブジェクトのデータ
struct Material {
    vec3 specular;      //鏡面反射の強さ
    float shininess;    //発光の強さ
};

//照明のデータ
struct Light {
    vec3 direction;     //照明の光が指すベクトル
    vec3 ambient;       //環境光の強さ
    vec3 diffuse;       //拡散光の強さ
    vec3 specular;      //鏡面反射の強さ
};

in float Alpha;
in vec3 FragPosition;
in vec3 Normal;
in vec2 TexCoords;

uniform sampler2D uScreenTexture;       //今回描画するテクスチャのデータ
uniform vec3 uViewPosition;              //カメラの座標データ
uniform Material uMaterial;             //3Dオブジェクトの証明に関する属性データ
uniform Light uLight;                   //照明のデータ

void main()
{
    //ambient
    vec3 ambient = uLight.ambient * texture(uScreenTexture, TexCoords).rgb;

    //diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(-uLight.direction);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = uLight.diffuse * diff * texture(uScreenTexture, TexCoords).rgb;

    //specular
    vec3 viewDir = normalize(uViewPosition - FragPosition);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), uMaterial.shininess);
    vec3 specular = uLight.specular * spec * uMaterial.specular;

    vec3 result = ambient + diffuse + specular;

    gl_FragColor = vec4(result, Alpha);
}