// WGSL language definition for highlight.js
// Based on https://github.com/highlightjs/highlightjs-wgsl
// Copyright 2024 Google LLC - Apache License 2.0

import type { HLJSApi, Language } from "highlight.js";

export default function wgsl(hljs: HLJSApi): Language {
  return {
    name: "WGSL",
    keywords: {
      keyword:
        "break continue continuing discard else for if loop return while switch case default " +
        "alias const const_assert diagnostic enable fn let override requires struct var " +
        "function private workgroup uniform storage " +
        "read read_write write " +
        "center centroid flat linear perspective sample " +
        "frag_depth front_facing global_invocation_id instance_index local_invocation_id " +
        "local_invocation_index num_workgroups position sample_index sample_mask " +
        "vertex_index workgroup_id " +
        "rgba8unorm rgba8snorm rgba8uint rgba8sint rgba16uint rgba16sint rgba16float " +
        "r32uint r32sint r32float rg32uint rg32sint rg32float rgba32uint rgba32sint " +
        "rgba32float bgra8unorm " +
        "align binding builtin compute const diagnostic fragment group id interpolate " +
        "invariant location must_use size vertex workgroup_size",

      type:
        "bool f32 f16 i32 i16 u32 u16 " +
        "vec2 vec2f vec2i vec2u vec3 vec3f vec3i vec3u vec4 vec4f vec4i vec4u " +
        "mat2x2 mat2x2f mat2x3 mat2x3f mat2x4 mat2x4f " +
        "mat3x2 mat3x2f mat3x3 mat3x3f mat3x4 mat3x4f " +
        "mat4x2 mat4x2f mat4x3 mat4x3f mat4x4 mat4x4f " +
        "texture_1d texture_2d texture_2d_array texture_3d texture_cube " +
        "texture_cube_array texture_multisampled_2d texture_storage_3d " +
        "texture_storage_1d texture_storage_2d texture_storage_2d_array " +
        "texture_depth_2d texture_depth_2d_array texture_depth_cube " +
        "texture_depth_cube_array sampler sampler_comparison " +
        "array atomic ptr",

      built_in:
        "bitcast all any select arrayLength " +
        "abs acos acosh asin asinh atan atanh atan2 ceil clamp cos cosh " +
        "countLeadingZeros countOneBits countTrailingZeros cross degrees " +
        "determinant distance dot exp exp2 extractBits faceForward " +
        "firstLeadingBit firstTrailingBit floor fma fract frexp inverseBits " +
        "inverseSqrt ldexp length log log2 max min mix modf normalize pow " +
        "quantizeToF16 radians reflect refract reverseBits round saturate " +
        "sign sin sinh smoothstep sqrt step tan tanh transpose trunc " +
        "dpdx dpdxCoarse dpdxFine dpdy dpdyCoarse dpdyFine fwidth " +
        "fwidthCoarse fwidthFine textureDimensions textureGather " +
        "textureLoad textureNumLayers textureNumLevels textureNumSamples " +
        "textureSample textureSampleBias textureSampleCompare " +
        "textureSampleLevel textureStore atomicLoad atomicStore atomicAdd " +
        "atomicSub atomicMax atomicMin atomicAnd atomicOr atomicXor " +
        "atomicExchange atomicCompareExchangeWeak pack4x8snorm pack4x8unorm " +
        "pack2x16snorm pack2x16unorm pack2x16float unpack4x8snorm " +
        "unpack4x8unorm unpack2x16snorm unpack2x16unorm unpack2x16float " +
        "storageBarrier textureBarrier workgroupBarrier workgroupUniformLoad",

      literal: "true false",
    },
    illegal: '"',
    contains: [
      hljs.C_LINE_COMMENT_MODE,
      hljs.C_BLOCK_COMMENT_MODE,
      hljs.C_NUMBER_MODE,
    ],
  };
}
