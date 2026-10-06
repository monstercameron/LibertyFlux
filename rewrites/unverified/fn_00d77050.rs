// original: 0x00D77050 CRenderPhaseRainUpdate::vf7

/// Build two worker objects and fold their slot answers into masks.
///
/// Allocates and constructs two workers (a null allocation faults on the
/// dereference, identically on both sides), calls virtual slot 8 of each
/// twice, and combines each pair with the signed `% 16` idiom (`and`
/// 0x8000000f with fixup: C-style signed remainder, negative answers
/// stay negative) and a truncating `/ 16` shifted left 14, xoring the
/// masked mix into the worker's word at `+4`. Emits a marker between the
/// two halves. Returns the second mix. Thiscall: object in `ecx`
/// (unused), no stack words.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

const ALLOC: u32 = 1;
const CTOR_A: u32 = 2;
const SLOT_A: u32 = 3;
const EMIT: u32 = 4;
const CTOR_B: u32 = 5;
const SLOT_B: u32 = 6;

export!(thiscall, rw_00d77050(this: u32) -> u32 {
    unsafe {
        const MARKER: u32 = 0x00c26c00;
        const MASK: u32 = 0x01ff_c000;
        #[inline(always)]
        fn smod16(v: u32) -> i32 {
            (v as i32) % 16
        }
        #[inline(always)]
        fn mix_pair(r1: u32, r2: u32) -> u32 {
            let t = (16 - smod16(r1)) % 16;
            ((r2.wrapping_add(t as u32) as i32) / 16).wrapping_shl(14) as u32
        }
        let _ = this;
        let p1 = callee_cdecl!(ALLOC, u32, 0x10, 1);
        let edi = if p1 == 0 {
            0
        } else {
            callee_thiscall!(CTOR_A, u32, p1, 0x23)
        };
        let v1 = (edi as *const u32).read_unaligned();
        let s1 = ((v1 + 8) as *const u32).read_unaligned();
        let slot_a: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(s1 as usize);
        let r1 = slot_a(edi);
        let r2 = slot_a(edi);
        let f1 = ((edi + 4) as *const u32).read_unaligned();
        let m1 = mix_pair(r1, r2) ^ f1;
        ((edi + 4) as *mut u32).write_unaligned(f1 ^ (m1 & MASK));
        callee_cdecl!(EMIT, u32, relocated(MARKER));
        let p2 = callee_cdecl!(ALLOC, u32, 8, 0);
        let esi = if p2 == 0 {
            0
        } else {
            callee_thiscall!(CTOR_B, u32, p2)
        };
        let v2 = (esi as *const u32).read_unaligned();
        let s2 = ((v2 + 8) as *const u32).read_unaligned();
        let slot_b: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(s2 as usize);
        let r3 = slot_b(esi);
        let r4 = slot_b(esi);
        let f2 = ((esi + 4) as *const u32).read_unaligned();
        let m2 = mix_pair(r3, r4) ^ f2;
        ((esi + 4) as *mut u32).write_unaligned(f2 ^ (m2 & MASK));
        m2 & MASK
    }
});
