struct VSInput {
    float3 position: POSITION;
    float4 color: TEXCOORD0;
};

struct VSOutput {
    float4 position: SV_Position;
    float4 color: TEXCOORD0;
};

VSOutput vs_main(VSInput input) {
    VSOutput output;
    output.position = float4(input.position, 1.0);
    output.color = input.color;
    return output;
}

float4 ps_main(VSOutput vs): SV_Target {
    return vs.color;
}