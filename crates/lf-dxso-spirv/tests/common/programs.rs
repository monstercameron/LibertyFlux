//! The two realistic hand-written programs, shared by `programs.rs` and
//! the `spirv-val` corpus. Written from scratch in the public token
//! format; they do not come from the game.

use super::*;

/// `vs_3_0`:
/// ```text
/// dcl_position v0; dcl_normal v1; dcl_texcoord0 v2; dcl_blendindices v3
/// dcl_position o0; dcl_texcoord0 o1.xy; dcl_texcoord1 o2; dcl_color0 o3; dcl_fog o4.x
/// defi i0, 4, 0, 1, 0
/// def c95, 3.0, 0.0, 1.0, 0.5
/// mul r0, v3, c95.x            ; bone index * 3 registers
/// mova a0, r0
/// m4x3 r1.xyz, v0, c100[a0.x]  ; skinning
/// mov r1.w, c95.z
/// m4x4 o0, r1, c0              ; world-view-projection
/// m3x3 r2.xyz, v1, c4
/// nrm r3.xyz, r2
/// mov r4, c95.y
/// loop aL, i0                  ; four lights
///   dp3 r5.x, r3, c20[aL]
///   max r5.x, r5.x, c95.y
///   mad r4.xyz, r5.x, c30[aL], r4
/// endloop
/// setp_gt p0.x, r4.x, c95.z
/// (p0.x) mov r4.xyz, c95.z
/// mov o3, r4
/// mov o1.xy, v2
/// dp4 r6.x, r1, c8
/// mul o4.x, r6.x, c9.x
/// mov o2, r3
/// ```
pub fn skinned_lit_vertex_shader() -> Vec<u32> {
    let mut a = Asm::vs();
    a.dcl(usage::POSITION, 0, vd(0))
        .dcl(usage::NORMAL, 0, vd(1))
        .dcl(usage::TEXCOORD, 0, vd(2))
        .dcl(usage::BLENDINDICES, 0, vd(3))
        .dcl(usage::POSITION, 0, od(0))
        .dcl(usage::TEXCOORD, 0, od(1).m("xy"))
        .dcl(usage::TEXCOORD, 1, od(2))
        .dcl(usage::COLOR, 0, od(3))
        .dcl(usage::FOG, 0, od(4).m("x"));
    a.defi(0, [4, 0, 1, 0]);
    a.def(95, [3.0, 0.0, 1.0, 0.5]);
    a.op2(op::MUL, rd(0), v(3), c(95).sw("x"));
    a.op1(op::MOVA, d(reg::ADDR, 0).m("x"), r(0).sw("x"));
    a.op2(op::M4X3, rd(1).m("xyz"), v(0), c(100).rel(A0X));
    a.mov(rd(1).m("w"), c(95).sw("z"));
    a.op2(op::M4X4, od(0), r(1), c(0));
    a.op2(op::M3X3, rd(2).m("xyz"), v(1), c(4));
    a.op1(op::NRM, rd(3).m("xyz"), r(2));
    a.mov(rd(4), c(95).sw("y"));
    a.flow(op::LOOP, 0, &[al(), i(0)]);
    a.op2(op::DP3, rd(5).m("x"), r(3), c(20).rel(AL));
    a.op2(op::MAX, rd(5).m("x"), r(5).sw("x"), c(95).sw("y"));
    a.op3(op::MAD, rd(4).m("xyz"), r(5).sw("x"), c(30).rel(AL), r(4));
    a.flow(op::ENDLOOP, 0, &[]);
    a.ins(
        op::SETP,
        cmp::GT,
        Some(d(reg::PRED, 0).m("x")),
        &[r(4).sw("x"), c(95).sw("z")],
    );
    a.pred(op::MOV, p0().sw("x"), rd(4).m("xyz"), &[c(95).sw("z")]);
    a.mov(od(3), r(4));
    a.mov(od(1).m("xy"), v(2));
    a.op2(op::DP4, rd(6).m("x"), r(1), c(8));
    a.op2(op::MUL, od(4).m("x"), r(6).sw("x"), c(9).sw("x"));
    a.mov(od(2), r(3));
    a.end()
}

/// `ps_3_0`:
/// ```text
/// dcl_texcoord0 v0.xy; dcl_texcoord1 v1.xyz; dcl_color0 v2; dcl_fog v3.x; dcl vFace
/// dcl_2d s0; dcl_cube s1
/// def c0, 0.5, 1.0, 0.0, 0.0
/// texld r0, v0, s0
/// add r1, r0.w, -c0.x
/// texkill r1                   ; alpha test
/// nrm r2.xyz, v1
/// texld r3, r2, s1             ; environment cube
/// dp3_sat r4.x, r2, c5
/// mad r5, r0, v2, r3
/// mul r5.xyz, r5, r4.x
/// lrp r6.xyz, v3.x, r5, c6     ; fog
/// if_lt vFace, c0.z            ; back faces darker
///   mul r6.xyz, r6, c0.x
/// endif
/// cmp r6.w, r0.w, c0.y, c0.z
/// mov oC0, r6
/// ```
pub fn textured_pixel_shader() -> Vec<u32> {
    let mut a = Asm::ps();
    a.dcl(usage::TEXCOORD, 0, vd(0).m("xy"))
        .dcl(usage::TEXCOORD, 1, vd(1).m("xyz"))
        .dcl(usage::COLOR, 0, vd(2))
        .dcl(usage::FOG, 0, vd(3).m("x"))
        .dcl_misc(1)
        .dcl_sampler(2, 0)
        .dcl_sampler(3, 1);
    a.def(0, [0.5, 1.0, 0.0, 0.0]);
    a.op2(op::TEX, rd(0), v(0), sampler(0));
    a.op2(op::ADD, rd(1), r(0).sw("w"), c(0).sw("x").neg());
    a.ins(op::TEXKILL, 0, Some(rd(1)), &[]);
    a.op1(op::NRM, rd(2).m("xyz"), v(1));
    a.op2(op::TEX, rd(3), r(2), sampler(1));
    a.op2(op::DP3, rd(4).m("x").sat(), r(2), c(5));
    a.op3(op::MAD, rd(5), r(0), v(2), r(3));
    a.op2(op::MUL, rd(5).m("xyz"), r(5), r(4).sw("x"));
    a.op3(op::LRP, rd(6).m("xyz"), v(3).sw("x"), r(5), c(6));
    a.flow(op::IFC, cmp::LT, &[s(reg::MISC, 1), c(0).sw("z")]);
    a.op2(op::MUL, rd(6).m("xyz"), r(6), c(0).sw("x"));
    a.flow(op::ENDIF, 0, &[]);
    a.op3(
        op::CMP,
        rd(6).m("w"),
        r(0).sw("w"),
        c(0).sw("y"),
        c(0).sw("z"),
    );
    a.mov(oc(0), r(6));
    a.end()
}
