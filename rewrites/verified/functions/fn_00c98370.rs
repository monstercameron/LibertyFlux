// original: 0x00c98370 ped_task_pose_update (proposed)
//! Rewrite of the ped task posing update (original 0x00C98370) for lane r-b325.
#![allow(unsafe_code)]

const TABLE_BASE: u32 = 0x0129_5CD8;
const VT_TASK_QUERY: u32 = 0x38;
const VT_PED_QUERY: u32 = 0xA0;
const VT_ANIM_QUERY: u32 = 0xE0;
const PED_STATE_OFF: u32 = 0x40;
const PED_TYPE_OFF: u32 = 0x2E;
const PED_EXTRA_OFF: u32 = 0x100;
const MAT_ROW3: u32 = 0x30;
const ROW_STRIDE: u32 = 0xE0;

const HALF: f32 = f32::from_bits(0x3F00_0000);
const ONE: f32 = f32::from_bits(0x3F80_0000);
const SIGN_MASK: f32 = f32::from_bits(0x8000_0000);
const HALF_BITS: u32 = 0x3F00_0000;
const ONE_BITS: u32 = 0x3F80_0000;
const SIGN_MASK_BITS: u32 = 0x8000_0000;

const ID_MAT: u32 = 2;
const ID_PAIR: u32 = 3;
const ID_SINK_A: u32 = 4;
const ID_SINK_B: u32 = 5;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn rd16u(a: u32) -> u16 {
    unsafe { (a as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wrf(a: u32, v: f32) {
    unsafe { wr32(a, v.to_bits()) }
}
#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}
#[inline(always)]
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}
#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}
#[inline(always)]
fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}
#[inline(always)]
fn sq(a: f32) -> f32 {
    core::hint::black_box(a).sqrt()
}
#[inline(always)]
fn xorb(a: f32, b: f32) -> f32 {
    f32::from_bits(a.to_bits() ^ b.to_bits())
}
#[inline(always)]
unsafe fn tbl(idx: u32) -> u32 {
    unsafe { lf_checker_rt::global::<u32>(TABLE_BASE).add(idx as usize).read() }
}
/// Task-query virtual call: slot VT_TASK_QUERY of the per-type object, one
/// integer argument, callee cleans the stack.
#[inline(always)]
unsafe fn vt38(obj: u32, arg: u32) -> u32 {
    unsafe {
        let vt = rd32(obj);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_TASK_QUERY)) as usize);
        f(obj, arg)
    }
}
/// Ped-query virtual call, no stack arguments.
#[inline(always)]
unsafe fn vt_a0(obj: u32) -> u32 {
    unsafe {
        let vt = rd32(obj);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_PED_QUERY)) as usize);
        f(obj)
    }
}
/// Anim-query virtual call, no stack arguments.
#[inline(always)]
unsafe fn vt_e0(obj: u32) -> u32 {
    unsafe {
        let vt = rd32(obj);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_ANIM_QUERY)) as usize);
        f(obj)
    }
}

/// Ped task posing update: refresh the task's cached aim/direction data from
/// the ped's current node matrices.
///
/// `this` points to the task object, whose state pointer lives at `+0x40`.
/// `a0`/`a1` are the two float parameters the caller passes (a rate and a
/// distance limit). The ped type word at `state+0x2e` selects a per-type
/// object from a global table; eighteen task-query virtual calls (slot
/// `+0x38`, one small integer id each) fetch node slots, and eight matrix
/// calls resolve them to 4x3 row-major matrices (four rows of three floats,
/// each row padded to 16 bytes, 64 bytes total).
///
/// The body is three phases of single-precision SSE arithmetic in the
/// original's operand order: a direction is formed from two rows, normalized
/// through an inverse square root (a zero length keeps a zero factor instead
/// of dividing), and added back into / subtracted from the third rows of the
/// fetched matrices; a long straight-line section then combines rows of two
/// matrices into two cached blocks; the tail resolves two sink descriptors
/// (through two ped-query calls each, whose zero/non-zero answer selects
/// between a stored extra pointer and a further query chain) and passes each
/// cached block with a table word to a sink call. The two stack arguments are
/// the only inputs besides the heap objects; the integer return value is the
/// last sink call's answer, which the caller ignores.
///
/// Two details are part of the contract, not the algorithm: one stack slot
/// is read without ever being written (the checker defines it as zero
/// through `stack_fill`), and the sink calls take pointers into the stack
/// frame (compared by an 8-word snapshot of the pointed-to data). The
/// original aligns the stack to 16 bytes on entry and restores it before
/// returning; that is frame bookkeeping with no observable effect.
///
/// Original: 0x00C98370 (thiscall, two float stack words, callee pops 8).
lf_checker_rt::export!(thiscall, rw_00c98370(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let arg0_bits = a0;
        let arg1_bits = a1;
        let arg0 = f32::from_bits(a0);
        let arg1 = f32::from_bits(a1);
        let mut r_ecx: u32 = this;
        // GENERATED by gen.py -- do not hand-edit; see report for method
        let mut r_edi: u32 = r_ecx;
        let mut s_n324: u32 = 0xf as u32;
        let mut r_eax: u32 = rd32(r_edi.wrapping_add(0x40));
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xf as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = r_eax;
        r_eax = lf_checker_rt::callee_thiscall!(ID_MAT, u32, r_ecx, r_eax);
        let mut x_xmm0 = rdf(r_eax.wrapping_add(0x0));
        let mut s_n144: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x4));
        let mut s_n140: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x8));
        let mut s_n136: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x10));
        let mut s_n128: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x14));
        let mut s_n124: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x18));
        let mut s_n120: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x20));
        let mut s_n112: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x24));
        let mut s_n108: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x28));
        let mut s_n104: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x30));
        let mut s_n96: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x34));
        let mut s_n92: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x38));
        r_eax = rd32(r_edi.wrapping_add(0x40));
        let mut s_n88: u32 = (x_xmm0).to_bits();
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        s_n324 = 0xb as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xb as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = r_eax;
        r_eax = lf_checker_rt::callee_thiscall!(ID_MAT, u32, r_ecx, r_eax);
        x_xmm0 = rdf(r_eax.wrapping_add(0x0));
        let mut s_n80: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x4));
        let mut s_n76: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x8));
        let mut x_xmm3 = f32::from_bits(s_n96);
        let mut x_xmm4 = f32::from_bits(s_n92);
        let mut s_n72: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x10));
        let mut s_n64: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x14));
        let mut s_n60: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x18));
        let mut s_n56: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x20));
        let mut x_xmm5 = f32::from_bits(s_n88);
        let mut s_n48: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x24));
        let mut s_n44: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x28));
        let mut s_n40: u32 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_eax.wrapping_add(0x30));
        let mut s_n32: u32 = (x_xmm0).to_bits();
        let mut x_xmm1 = rdf(r_eax.wrapping_add(0x34));
        x_xmm3 = sub(x_xmm3, x_xmm0);
        x_xmm4 = sub(x_xmm4, x_xmm1);
        let mut s_n28: u32 = (x_xmm1).to_bits();
        let mut x_xmm2 = rdf(r_eax.wrapping_add(0x38));
        let mut s_n24: u32 = (x_xmm2).to_bits();
        x_xmm5 = sub(x_xmm5, x_xmm2);
        x_xmm0 = x_xmm3;
        x_xmm1 = x_xmm4;
        x_xmm1 = mul(x_xmm1, x_xmm4);
        x_xmm0 = mul(x_xmm0, x_xmm3);
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = x_xmm5;
        x_xmm0 = mul(x_xmm0, x_xmm5);
        let mut x_xmm7 = HALF;
        x_xmm2 = xorb(x_xmm2, x_xmm2);
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = arg0;
        x_xmm0 = mul(x_xmm0, x_xmm7);
        let mut x_xmm6 = sq(x_xmm1);
        x_xmm6 = sub(x_xmm6, arg1);
        x_xmm6 = mul(x_xmm6, x_xmm0);
        //   (control flow assembled by hand)
        if x_xmm1 != 0.0 {
            x_xmm0 = sq(x_xmm1);
            x_xmm2 = ONE;
            x_xmm2 = div(x_xmm2, x_xmm0);
        }
        x_xmm0 = SIGN_MASK;
        r_eax = rd32(r_edi.wrapping_add(0x40));
        x_xmm3 = mul(x_xmm3, x_xmm2);
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        x_xmm3 = xorb(x_xmm3, x_xmm0);
        r_ecx = tbl(r_eax);
        x_xmm4 = mul(x_xmm4, x_xmm2);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        x_xmm5 = mul(x_xmm5, x_xmm2);
        r_eax = rd32(r_eax.wrapping_add(0x38));
        x_xmm4 = xorb(x_xmm4, x_xmm0);
        x_xmm5 = xorb(x_xmm5, x_xmm0);
        x_xmm3 = mul(x_xmm3, x_xmm6);
        x_xmm4 = mul(x_xmm4, x_xmm6);
        x_xmm0 = x_xmm3;
        x_xmm0 = mul(x_xmm0, x_xmm7);
        x_xmm5 = mul(x_xmm5, x_xmm6);
        let mut s_n224: u32 = (x_xmm0).to_bits();
        x_xmm0 = x_xmm4;
        x_xmm0 = mul(x_xmm0, x_xmm7);
        s_n324 = 0xe as u32;
        let mut s_n272: u32 = (x_xmm3).to_bits();
        let mut s_n200: u32 = (x_xmm0).to_bits();
        x_xmm0 = x_xmm5;
        x_xmm0 = mul(x_xmm0, x_xmm7);
        let mut s_n276: u32 = (x_xmm4).to_bits();
        let mut s_n304: u32 = (x_xmm5).to_bits();
        let mut s_n240: u32 = (x_xmm0).to_bits();
        r_eax = vt38(r_ecx, 0xe as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = r_eax;
        r_eax = lf_checker_rt::callee_thiscall!(ID_MAT, u32, r_ecx, r_eax);
        x_xmm0 = f32::from_bits(s_n224);
        x_xmm0 = add(x_xmm0, rdf(r_eax.wrapping_add(0x30)));
        s_n324 = 0xa as u32;
        wrf(r_eax.wrapping_add(0x30), x_xmm0);
        x_xmm0 = f32::from_bits(s_n200);
        x_xmm0 = add(x_xmm0, rdf(r_eax.wrapping_add(0x34)));
        wrf(r_eax.wrapping_add(0x34), x_xmm0);
        x_xmm0 = f32::from_bits(s_n240);
        x_xmm0 = add(x_xmm0, rdf(r_eax.wrapping_add(0x38)));
        wrf(r_eax.wrapping_add(0x38), x_xmm0);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xa as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = r_eax;
        r_eax = lf_checker_rt::callee_thiscall!(ID_MAT, u32, r_ecx, r_eax);
        x_xmm0 = rdf(r_eax.wrapping_add(0x30));
        x_xmm0 = sub(x_xmm0, f32::from_bits(s_n224));
        s_n324 = 0xf as u32;
        wrf(r_eax.wrapping_add(0x30), x_xmm0);
        x_xmm0 = rdf(r_eax.wrapping_add(0x34));
        x_xmm0 = sub(x_xmm0, f32::from_bits(s_n200));
        wrf(r_eax.wrapping_add(0x34), x_xmm0);
        x_xmm0 = rdf(r_eax.wrapping_add(0x38));
        x_xmm0 = sub(x_xmm0, f32::from_bits(s_n240));
        wrf(r_eax.wrapping_add(0x38), x_xmm0);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xf as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = r_eax;
        r_eax = lf_checker_rt::callee_thiscall!(ID_MAT, u32, r_ecx, r_eax);
        x_xmm0 = f32::from_bits(s_n272);
        x_xmm0 = add(x_xmm0, rdf(r_eax.wrapping_add(0x30)));
        wrf(r_eax.wrapping_add(0x30), x_xmm0);
        x_xmm0 = f32::from_bits(s_n276);
        x_xmm0 = add(x_xmm0, rdf(r_eax.wrapping_add(0x34)));
        wrf(r_eax.wrapping_add(0x34), x_xmm0);
        x_xmm0 = f32::from_bits(s_n304);
        x_xmm0 = add(x_xmm0, rdf(r_eax.wrapping_add(0x38)));
        wrf(r_eax.wrapping_add(0x38), x_xmm0);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        s_n324 = 0xb as u32;
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xb as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = r_eax;
        r_eax = lf_checker_rt::callee_thiscall!(ID_MAT, u32, r_ecx, r_eax);
        x_xmm0 = rdf(r_eax.wrapping_add(0x30));
        x_xmm0 = sub(x_xmm0, f32::from_bits(s_n272));
        s_n324 = 0xe as u32;
        wrf(r_eax.wrapping_add(0x30), x_xmm0);
        x_xmm0 = rdf(r_eax.wrapping_add(0x34));
        x_xmm0 = sub(x_xmm0, f32::from_bits(s_n276));
        wrf(r_eax.wrapping_add(0x34), x_xmm0);
        x_xmm0 = rdf(r_eax.wrapping_add(0x38));
        x_xmm0 = sub(x_xmm0, f32::from_bits(s_n304));
        wrf(r_eax.wrapping_add(0x38), x_xmm0);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xe as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 0xd as u32;
        r_ecx = ((rd16u(r_ecx.wrapping_add(0x2e))) as u16 as i16) as u32;
        let mut r_esi: u32 = r_eax;
        r_ecx = tbl(r_ecx);
        let mut r_edx: u32 = rd32(r_ecx.wrapping_add(0x0));
        r_edx = rd32(r_edx.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xd as u32);
        s_n324 = r_esi;
        let mut s_n328: u32 = r_eax;
        r_ecx = r_edi;
        r_eax = lf_checker_rt::callee_thiscall!(ID_PAIR, u32, r_ecx, r_eax, r_esi);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 0xf as u32;
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xf as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 0xe as u32;
        r_ecx = ((rd16u(r_ecx.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_esi = r_eax;
        r_ecx = tbl(r_ecx);
        r_edx = rd32(r_ecx.wrapping_add(0x0));
        r_edx = rd32(r_edx.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xe as u32);
        s_n324 = r_esi;
        s_n328 = r_eax;
        r_ecx = r_edi;
        r_eax = lf_checker_rt::callee_thiscall!(ID_PAIR, u32, r_ecx, r_eax, r_esi);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 0xa as u32;
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xa as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 9 as u32;
        r_ecx = ((rd16u(r_ecx.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_esi = r_eax;
        r_ecx = tbl(r_ecx);
        r_edx = rd32(r_ecx.wrapping_add(0x0));
        r_edx = rd32(r_edx.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 9 as u32);
        s_n324 = r_esi;
        s_n328 = r_eax;
        r_ecx = r_edi;
        r_eax = lf_checker_rt::callee_thiscall!(ID_PAIR, u32, r_ecx, r_eax, r_esi);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 0xb as u32;
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xb as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 0xa as u32;
        r_ecx = ((rd16u(r_ecx.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_esi = r_eax;
        r_ecx = tbl(r_ecx);
        r_edx = rd32(r_ecx.wrapping_add(0x0));
        r_edx = rd32(r_edx.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xa as u32);
        s_n324 = r_esi;
        s_n328 = r_eax;
        r_ecx = r_edi;
        r_eax = lf_checker_rt::callee_thiscall!(ID_PAIR, u32, r_ecx, r_eax, r_esi);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 0xf as u32;
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xf as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = r_eax;
        r_eax = lf_checker_rt::callee_thiscall!(ID_MAT, u32, r_ecx, r_eax);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 0xb as u32;
        r_ecx = ((rd16u(r_ecx.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_esi = r_eax;
        r_ecx = tbl(r_ecx);
        r_edx = rd32(r_ecx.wrapping_add(0x0));
        r_edx = rd32(r_edx.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xb as u32);
        r_ecx = rd32(r_edi.wrapping_add(0x40));
        s_n324 = r_eax;
        r_eax = lf_checker_rt::callee_thiscall!(ID_MAT, u32, r_ecx, r_eax);
        x_xmm0 = f32::from_bits(s_n128);
        x_xmm4 = f32::from_bits(s_n140);
        x_xmm1 = f32::from_bits(s_n136);
        x_xmm2 = f32::from_bits(s_n96);
        x_xmm3 = f32::from_bits(s_n92);
        s_n276 = (x_xmm0).to_bits();
        s_n140 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n112);
        s_n240 = (x_xmm0).to_bits();
        s_n136 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n144);
        x_xmm6 = x_xmm0;
        let mut s_n176: u32 = (x_xmm0).to_bits();
        x_xmm5 = f32::from_bits(s_n120);
        x_xmm6 = mul(x_xmm6, x_xmm2);
        x_xmm0 = x_xmm4;
        x_xmm0 = mul(x_xmm0, x_xmm3);
        x_xmm7 = f32::from_bits(s_n108);
        s_n272 = (x_xmm1).to_bits();
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = x_xmm1;
        s_n112 = (x_xmm1).to_bits();
        x_xmm1 = f32::from_bits(s_n88);
        x_xmm0 = mul(x_xmm0, x_xmm1);
        let mut s_n256: u32 = (x_xmm3).to_bits();
        x_xmm3 = f32::from_bits(s_n124);
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = f32::from_bits(s_n276);
        x_xmm0 = mul(x_xmm0, x_xmm2);
        let mut s_n152: u32 = (x_xmm5).to_bits();
        s_n304 = (x_xmm6).to_bits();
        x_xmm6 = x_xmm3;
        x_xmm6 = mul(x_xmm6, f32::from_bits(s_n256));
        s_n108 = (x_xmm5).to_bits();
        s_n224 = (x_xmm4).to_bits();
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = x_xmm5;
        x_xmm5 = f32::from_bits(s_n256);
        x_xmm0 = mul(x_xmm0, x_xmm1);
        x_xmm5 = mul(x_xmm5, x_xmm7);
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = f32::from_bits(s_n240);
        x_xmm0 = mul(x_xmm0, x_xmm2);
        s_n128 = (x_xmm4).to_bits();
        let mut s_n160: u32 = (x_xmm7).to_bits();
        x_xmm5 = add(x_xmm5, x_xmm0);
        x_xmm0 = f32::from_bits(s_n104);
        s_n120 = (x_xmm7).to_bits();
        x_xmm0 = mul(x_xmm0, x_xmm1);
        x_xmm1 = SIGN_MASK;
        x_xmm6 = xorb(x_xmm6, x_xmm1);
        x_xmm5 = add(x_xmm5, x_xmm0);
        x_xmm0 = f32::from_bits(0u32);
        let mut s_n84: u32 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n304);
        x_xmm0 = xorb(x_xmm0, x_xmm1);
        s_n96 = (x_xmm0).to_bits();
        s_n304 = (x_xmm0).to_bits();
        x_xmm5 = xorb(x_xmm5, x_xmm1);
        s_n88 = (x_xmm5).to_bits();
        s_n92 = (x_xmm6).to_bits();
        x_xmm0 = rdf(r_esi.wrapping_add(0x14));
        x_xmm1 = rdf(r_esi.wrapping_add(0x4));
        x_xmm1 = mul(x_xmm1, f32::from_bits(s_n176));
        let mut s_n312: u32 = (x_xmm0).to_bits();
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n276));
        s_n256 = (x_xmm5).to_bits();
        x_xmm5 = rdf(r_esi.wrapping_add(0x24));
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = x_xmm5;
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n240));
        x_xmm2 = rdf(r_esi.wrapping_add(0x10));
        s_n200 = (x_xmm6).to_bits();
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x18));
        let mut s_n308: u32 = (x_xmm0).to_bits();
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n276));
        x_xmm6 = rdf(r_esi.wrapping_add(0x0));
        let mut s_n156: u32 = (x_xmm1).to_bits();
        x_xmm1 = rdf(r_esi.wrapping_add(0x8));
        x_xmm1 = mul(x_xmm1, f32::from_bits(s_n176));
        x_xmm4 = mul(x_xmm4, x_xmm6);
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x28));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n240));
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = x_xmm2;
        x_xmm0 = mul(x_xmm0, x_xmm3);
        let mut s_n148: u32 = (x_xmm1).to_bits();
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = x_xmm7;
        x_xmm0 = mul(x_xmm0, rdf(r_esi.wrapping_add(0x20)));
        x_xmm7 = f32::from_bits(s_n312);
        x_xmm7 = mul(x_xmm7, x_xmm3);
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x4));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n224));
        let mut s_n192: u32 = (x_xmm4).to_bits();
        x_xmm4 = f32::from_bits(s_n160);
        x_xmm7 = add(x_xmm7, x_xmm0);
        x_xmm0 = x_xmm5;
        x_xmm5 = f32::from_bits(s_n308);
        x_xmm0 = mul(x_xmm0, x_xmm4);
        x_xmm5 = mul(x_xmm5, x_xmm3);
        x_xmm7 = add(x_xmm7, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x8));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n224));
        x_xmm3 = f32::from_bits(s_n312);
        s_n160 = (x_xmm7).to_bits();
        x_xmm7 = f32::from_bits(s_n152);
        x_xmm5 = add(x_xmm5, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x28));
        x_xmm0 = mul(x_xmm0, x_xmm4);
        x_xmm4 = f32::from_bits(s_n272);
        x_xmm4 = mul(x_xmm4, x_xmm6);
        x_xmm5 = add(x_xmm5, x_xmm0);
        x_xmm0 = x_xmm2;
        x_xmm0 = mul(x_xmm0, x_xmm7);
        x_xmm3 = mul(x_xmm3, x_xmm7);
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = f32::from_bits(s_n104);
        x_xmm0 = mul(x_xmm0, rdf(r_esi.wrapping_add(0x20)));
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x4));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n272));
        x_xmm3 = add(x_xmm3, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x24));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n104));
        x_xmm3 = add(x_xmm3, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x8));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n272));
        x_xmm2 = f32::from_bits(s_n308);
        x_xmm1 = f32::from_bits(s_n304);
        x_xmm2 = mul(x_xmm2, x_xmm7);
        x_xmm1 = mul(x_xmm1, x_xmm6);
        x_xmm2 = add(x_xmm2, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x28));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n104));
        x_xmm2 = add(x_xmm2, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x10));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n200));
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = f32::from_bits(s_n256);
        x_xmm0 = mul(x_xmm0, rdf(r_esi.wrapping_add(0x20)));
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = f32::from_bits(s_n312);
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n200));
        x_xmm1 = add(x_xmm1, rdf(r_esi.wrapping_add(0x30)));
        s_n312 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_esi.wrapping_add(0x4));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n304));
        x_xmm7 = f32::from_bits(s_n312);
        x_xmm7 = add(x_xmm7, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x24));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n256));
        x_xmm7 = add(x_xmm7, x_xmm0);
        x_xmm0 = x_xmm7;
        x_xmm0 = add(x_xmm0, rdf(r_esi.wrapping_add(0x34)));
        s_n312 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n308);
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n200));
        s_n308 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_esi.wrapping_add(0x8));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n304));
        x_xmm7 = f32::from_bits(s_n308);
        x_xmm7 = add(x_xmm7, x_xmm0);
        x_xmm0 = rdf(r_esi.wrapping_add(0x28));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n256));
        x_xmm7 = add(x_xmm7, x_xmm0);
        x_xmm0 = x_xmm7;
        x_xmm0 = add(x_xmm0, rdf(r_esi.wrapping_add(0x38)));
        x_xmm7 = f32::from_bits(s_n176);
        x_xmm7 = mul(x_xmm7, x_xmm6);
        s_n308 = (x_xmm0).to_bits();
        x_xmm0 = rdf(r_esi.wrapping_add(0x10));
        x_xmm0 = mul(x_xmm0, f32::from_bits(s_n276));
        x_xmm7 = add(x_xmm7, x_xmm0);
        x_xmm0 = f32::from_bits(s_n240);
        x_xmm0 = mul(x_xmm0, rdf(r_esi.wrapping_add(0x20)));
        s_n96 = (x_xmm1).to_bits();
        x_xmm1 = f32::from_bits(s_n64);
        x_xmm7 = add(x_xmm7, x_xmm0);
        x_xmm0 = f32::from_bits(s_n156);
        s_n140 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n148);
        s_n136 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n192);
        s_n128 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n312);
        s_n92 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n308);
        s_n88 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n76);
        s_n144 = (x_xmm7).to_bits();
        x_xmm7 = f32::from_bits(s_n160);
        s_n104 = (x_xmm2).to_bits();
        x_xmm2 = f32::from_bits(s_n48);
        s_n224 = (x_xmm1).to_bits();
        s_n76 = (x_xmm1).to_bits();
        x_xmm1 = f32::from_bits(s_n72);
        s_n124 = (x_xmm7).to_bits();
        s_n120 = (x_xmm5).to_bits();
        s_n112 = (x_xmm4).to_bits();
        s_n108 = (x_xmm3).to_bits();
        s_n240 = (x_xmm0).to_bits();
        s_n64 = (x_xmm0).to_bits();
        s_n176 = (x_xmm2).to_bits();
        s_n304 = (x_xmm1).to_bits();
        s_n48 = (x_xmm1).to_bits();
        s_n72 = (x_xmm2).to_bits();
        x_xmm2 = f32::from_bits(s_n56);
        x_xmm7 = f32::from_bits(s_n80);
        x_xmm3 = f32::from_bits(s_n44);
        x_xmm4 = f32::from_bits(s_n28);
        x_xmm0 = mul(x_xmm0, x_xmm4);
        x_xmm6 = f32::from_bits(s_n60);
        s_n44 = (x_xmm2).to_bits();
        s_n276 = (x_xmm2).to_bits();
        x_xmm2 = f32::from_bits(s_n32);
        x_xmm5 = x_xmm7;
        x_xmm5 = mul(x_xmm5, x_xmm2);
        s_n308 = (x_xmm6).to_bits();
        s_n56 = (x_xmm3).to_bits();
        x_xmm5 = add(x_xmm5, x_xmm0);
        x_xmm0 = x_xmm1;
        x_xmm1 = f32::from_bits(s_n24);
        x_xmm0 = mul(x_xmm0, x_xmm1);
        s_n192 = (x_xmm7).to_bits();
        x_xmm5 = add(x_xmm5, x_xmm0);
        x_xmm0 = x_xmm6;
        x_xmm0 = mul(x_xmm0, x_xmm4);
        x_xmm4 = mul(x_xmm4, x_xmm3);
        s_n312 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n224);
        x_xmm6 = f32::from_bits(s_n312);
        x_xmm0 = mul(x_xmm0, x_xmm2);
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = f32::from_bits(s_n276);
        x_xmm0 = mul(x_xmm0, x_xmm1);
        x_xmm6 = add(x_xmm6, x_xmm0);
        s_n312 = (x_xmm6).to_bits();
        x_xmm6 = f32::from_bits(s_n176);
        x_xmm0 = x_xmm6;
        x_xmm0 = mul(x_xmm0, x_xmm2);
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = f32::from_bits(s_n40);
        x_xmm0 = mul(x_xmm0, x_xmm1);
        x_xmm1 = SIGN_MASK;
        x_xmm5 = xorb(x_xmm5, x_xmm1);
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = f32::from_bits(0u32);
        let mut s_n20: u32 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n312);
        x_xmm0 = xorb(x_xmm0, x_xmm1);
        s_n28 = (x_xmm0).to_bits();
        s_n32 = (x_xmm5).to_bits();
        s_n312 = (x_xmm0).to_bits();
        x_xmm4 = xorb(x_xmm4, x_xmm1);
        s_n24 = (x_xmm4).to_bits();
        x_xmm2 = rdf(r_eax.wrapping_add(0x18));
        s_n256 = (x_xmm5).to_bits();
        x_xmm5 = f32::from_bits(s_n224);
        x_xmm1 = x_xmm5;
        x_xmm1 = mul(x_xmm1, rdf(r_eax.wrapping_add(0x14)));
        x_xmm0 = x_xmm7;
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x4)));
        s_n200 = (x_xmm4).to_bits();
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = x_xmm6;
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x24)));
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = x_xmm7;
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x8)));
        x_xmm7 = rdf(r_eax.wrapping_add(0x20));
        s_n148 = (x_xmm1).to_bits();
        x_xmm1 = x_xmm5;
        x_xmm5 = rdf(r_eax.wrapping_add(0x0));
        x_xmm1 = mul(x_xmm1, x_xmm2);
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = x_xmm6;
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x28)));
        x_xmm6 = f32::from_bits(s_n308);
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = f32::from_bits(s_n240);
        x_xmm0 = mul(x_xmm0, x_xmm5);
        s_n156 = (x_xmm1).to_bits();
        x_xmm1 = rdf(r_eax.wrapping_add(0x10));
        x_xmm6 = mul(x_xmm6, x_xmm1);
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = x_xmm3;
        x_xmm0 = mul(x_xmm0, x_xmm7);
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = f32::from_bits(s_n308);
        s_n152 = (x_xmm6).to_bits();
        x_xmm6 = x_xmm0;
        x_xmm6 = mul(x_xmm6, rdf(r_eax.wrapping_add(0x14)));
        x_xmm0 = f32::from_bits(s_n240);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x4)));
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = x_xmm3;
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x24)));
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = f32::from_bits(s_n308);
        x_xmm0 = mul(x_xmm0, x_xmm2);
        s_n160 = (x_xmm6).to_bits();
        s_n308 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n240);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x8)));
        x_xmm4 = f32::from_bits(s_n308);
        x_xmm6 = f32::from_bits(s_n276);
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = x_xmm3;
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x28)));
        x_xmm3 = x_xmm6;
        x_xmm3 = mul(x_xmm3, rdf(r_eax.wrapping_add(0x14)));
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = x_xmm6;
        x_xmm0 = mul(x_xmm0, x_xmm1);
        x_xmm6 = mul(x_xmm6, x_xmm2);
        s_n308 = (x_xmm4).to_bits();
        x_xmm4 = f32::from_bits(s_n304);
        x_xmm4 = mul(x_xmm4, x_xmm5);
        x_xmm2 = f32::from_bits(s_n256);
        x_xmm2 = mul(x_xmm2, x_xmm5);
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = f32::from_bits(s_n40);
        x_xmm0 = mul(x_xmm0, x_xmm7);
        x_xmm4 = add(x_xmm4, x_xmm0);
        x_xmm0 = f32::from_bits(s_n304);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x4)));
        x_xmm3 = add(x_xmm3, x_xmm0);
        x_xmm0 = f32::from_bits(s_n40);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x24)));
        x_xmm3 = add(x_xmm3, x_xmm0);
        x_xmm0 = f32::from_bits(s_n304);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x8)));
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = f32::from_bits(s_n40);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x28)));
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = f32::from_bits(s_n312);
        x_xmm0 = mul(x_xmm0, x_xmm1);
        x_xmm1 = f32::from_bits(s_n312);
        x_xmm1 = mul(x_xmm1, rdf(r_eax.wrapping_add(0x14)));
        x_xmm2 = add(x_xmm2, x_xmm0);
        x_xmm0 = f32::from_bits(s_n200);
        x_xmm0 = mul(x_xmm0, x_xmm7);
        s_n276 = (x_xmm6).to_bits();
        x_xmm2 = add(x_xmm2, x_xmm0);
        x_xmm0 = f32::from_bits(s_n256);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x4)));
        x_xmm2 = add(x_xmm2, rdf(r_eax.wrapping_add(0x30)));
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = f32::from_bits(s_n200);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x24)));
        x_xmm1 = add(x_xmm1, x_xmm0);
        x_xmm0 = f32::from_bits(s_n312);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x18)));
        x_xmm1 = add(x_xmm1, rdf(r_eax.wrapping_add(0x34)));
        s_n312 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n256);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x8)));
        x_xmm6 = f32::from_bits(s_n312);
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = f32::from_bits(s_n200);
        x_xmm0 = mul(x_xmm0, rdf(r_eax.wrapping_add(0x28)));
        x_xmm6 = add(x_xmm6, x_xmm0);
        x_xmm0 = x_xmm6;
        x_xmm0 = add(x_xmm0, rdf(r_eax.wrapping_add(0x38)));
        s_n312 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n192);
        x_xmm0 = mul(x_xmm0, x_xmm5);
        x_xmm5 = f32::from_bits(s_n224);
        x_xmm5 = mul(x_xmm5, rdf(r_eax.wrapping_add(0x10)));
        x_xmm6 = f32::from_bits(s_n160);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        x_xmm0 = add(x_xmm0, x_xmm5);
        x_xmm5 = f32::from_bits(s_n176);
        x_xmm5 = mul(x_xmm5, x_xmm7);
        s_n60 = (x_xmm6).to_bits();
        s_n48 = (x_xmm4).to_bits();
        x_xmm0 = add(x_xmm0, x_xmm5);
        s_n44 = (x_xmm3).to_bits();
        s_n32 = (x_xmm2).to_bits();
        s_n28 = (x_xmm1).to_bits();
        s_n324 = 0xf as u32;
        s_n80 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n148);
        s_n76 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n156);
        s_n72 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n152);
        s_n64 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n308);
        s_n56 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n276);
        s_n40 = (x_xmm0).to_bits();
        x_xmm0 = f32::from_bits(s_n312);
        s_n24 = (x_xmm0).to_bits();
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xf as u32);
        r_esi = rd32(r_edi.wrapping_add(0x40));
        r_ecx = r_esi;
        r_edx = rd32(r_esi.wrapping_add(0x0));
        s_n192 = r_eax;
        r_edx = rd32(r_edx.wrapping_add(0xa0));
        r_eax = vt_a0(r_ecx);
        if r_eax == 0 {
            r_eax = rd32(r_esi.wrapping_add(0x100));
        } else {
            r_eax = rd32(r_esi.wrapping_add(0x0));
            r_ecx = r_esi;
            r_eax = rd32(r_eax.wrapping_add(0xa0));
            r_eax = vt_a0(r_ecx);
            r_edx = rd32(r_eax.wrapping_add(0x0));
            r_ecx = r_eax;
            r_eax = rd32(r_edx.wrapping_add(0xe0));
            r_eax = vt_e0(r_ecx);
        }
        r_eax = rd32(r_eax.wrapping_add(0x4));
        r_eax = rd32(r_eax.wrapping_add(0x0));
        s_n324 = 0u32;
        r_ecx = s_n192;
        r_ecx = r_ecx.wrapping_mul(0xe0);
        r_eax = rd16u(r_eax.wrapping_add(r_ecx).wrapping_add(0x14)) as u32;
        s_n328 = r_eax;
        r_ecx = r_edi;
        let snap_c991e6: [u32; 8] = [s_n144, s_n140, s_n136, 0u32 /*unwritten*/, s_n128, s_n124, s_n120, 0u32 /*unwritten*/];
        r_eax = lf_checker_rt::callee_thiscall!(ID_SINK_A, u32, r_ecx, r_eax, snap_c991e6.as_ptr() as u32);
        r_eax = rd32(r_edi.wrapping_add(0x40));
        s_n324 = 0xb as u32;
        r_eax = ((rd16u(r_eax.wrapping_add(0x2e))) as u16 as i16) as u32;
        r_ecx = tbl(r_eax);
        r_eax = rd32(r_ecx.wrapping_add(0x0));
        r_eax = rd32(r_eax.wrapping_add(0x38));
        r_eax = vt38(r_ecx, 0xb as u32);
        r_esi = rd32(r_edi.wrapping_add(0x40));
        r_ecx = r_esi;
        r_edx = rd32(r_esi.wrapping_add(0x0));
        s_n192 = r_eax;
        r_edx = rd32(r_edx.wrapping_add(0xa0));
        r_eax = vt_a0(r_ecx);
        if r_eax == 0 {
            r_eax = rd32(r_esi.wrapping_add(0x100));
        } else {
            r_eax = rd32(r_esi.wrapping_add(0x0));
            r_ecx = r_esi;
            r_eax = rd32(r_eax.wrapping_add(0xa0));
            r_eax = vt_a0(r_ecx);
            r_edx = rd32(r_eax.wrapping_add(0x0));
            r_ecx = r_eax;
            r_eax = rd32(r_edx.wrapping_add(0xe0));
            r_eax = vt_e0(r_ecx);
        }
        r_eax = rd32(r_eax.wrapping_add(0x4));
        r_eax = rd32(r_eax.wrapping_add(0x0));
        s_n324 = 0u32;
        r_ecx = s_n192;
        r_ecx = r_ecx.wrapping_mul(0xe0);
        r_eax = rd16u(r_eax.wrapping_add(r_ecx).wrapping_add(0x14)) as u32;
        s_n328 = r_eax;
        r_ecx = r_edi;
        let snap_c9925e: [u32; 8] = [s_n80, s_n76, s_n72, 0u32 /*unwritten*/, s_n64, s_n60, s_n56, 0u32 /*unwritten*/];
        r_eax = lf_checker_rt::callee_thiscall!(ID_SINK_B, u32, r_ecx, r_eax, snap_c9925e.as_ptr() as u32);
        r_eax
    }
});
