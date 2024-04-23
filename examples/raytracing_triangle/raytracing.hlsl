struct Viewport {
    float left;
    float top;
    float right;
    float bottom;
};

struct RayGenConstantBuffer {
    Viewport viewport;
    Viewport stencil;
};

RaytracingAccelerationStructure scene: register(t0, space0);
RWTexture2D<float4> render_target: register(u0);
ConstantBuffer<RayGenConstantBuffer> ray_gen: register(b0);

typedef BuiltInTriangleIntersectionAttributes Attributes;

struct RayPayload {
    float4 color;
};

bool is_inside_viewport(float2 p, Viewport viewport) {
    return (p.x >= viewport.left && p.x <= viewport.right) && (p.y >= viewport.top && p.y <= viewport.bottom);
}

[shader("raygeneration")]
void ray_gen_shader() {
    const float2 d = (float2)DispatchRaysIndex() / (float2)DispatchRaysDimensions();
    float3 ray_dir = float3(0, 0, 1);
    float3 origin = float3(
        lerp(ray_gen.viewport.left, ray_gen.viewport.right, d.x),
        lerp(ray_gen.viewport.top, ray_gen.viewport.bottom, d.y),
        0.0f
    );
    if (is_inside_viewport(origin.xy, ray_gen.stencil)) {
        RayDesc ray;
        ray.Origin = origin;
        ray.Direction = ray_dir;
        ray.TMin = 0.001;
        ray.TMax = 10000.0;
        RayPayload payload = { float4(0, 0, 0, 0) };
        TraceRay(scene, RAY_FLAG_CULL_BACK_FACING_TRIANGLES, ~0, 0, 1, 0, ray, payload);
        render_target[DispatchRaysIndex().xy] = payload.color;
    } else {
        render_target[DispatchRaysIndex().xy] = float4(d, 0, 1);
    }
}

[shader("closesthit")]
void closest_hit_shader(inout RayPayload payload, in Attributes attr) {
    const float3 barycentrics = float3(1 - attr.barycentrics.x - attr.barycentrics.y, attr.barycentrics.x, attr.barycentrics.y);
    payload.color = float4(barycentrics, 1);
}

[shader("miss")]
void miss_shader(inout RayPayload payload) {
    payload.color = float4(0, 0, 0, 1);
}

