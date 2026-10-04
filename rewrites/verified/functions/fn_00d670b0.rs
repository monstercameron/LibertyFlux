// original: 0x00d670b0 emit_blend_blocks
//
// Combines the two input triples into difference and sum triples, then
// emits six nine-word apply blocks per record: the record's triple
// minus the differences, plus the differences, minus the sums and plus
// the sums (two blocks repeat), each with its own constant tail. All
// floating-point operations keep the original's operand order so NaN
// payloads and signed zeros match bit for bit.

use lf_checker_rt::{callee_cdecl, export};

// Callee ids (see contract).
const T1_ALLOC: u32 = 21; // 0x446280 cdecl/2: sized setup (3, count*6)
const T2_APPLY: u32 = 22; // 0x4462E0 cdecl/9: apply block (6 sites per record)
const T3_DONE: u32 = 23; // 0x437950 cdecl/0: teardown, answers the return

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rf32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

/// Rewrite of the original function at 0x00D670B0
/// (cdecl/5 -> u32: walks `count` records, returns the teardown answer,
/// or the entry count when it is not positive).
///
/// Combines the two input triples into difference and sum triples, then
/// emits six nine-word apply blocks per record: the record's triple
/// minus the differences, plus the differences, minus the sums and plus
/// the sums (two blocks repeat), each with its own constant tail. All
/// floating-point operations keep the original's operand order so NaN
/// payloads and signed zeros match bit for bit.
export!(cdecl, rw_d670b0(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> u32 {
    unsafe {
        let v0 = rf32(p3);
        let v1 = rf32(p3 + 4);
        let v2 = rf32(p3 + 8);
        let w0 = rf32(p4);
        let w1 = rf32(p4 + 4);
        let w2 = rf32(p4 + 8);
        let d0 = v0 - w0;
        let d1 = v1 - w1;
        let d2 = v2 - w2;
        let s0 = w0 + v0;
        let s1 = w1 + v1;
        let s2 = w2 + v2;
        let n0 = (p2 as *const i32).read_unaligned();
        if n0 <= 0 {
            wr32(p2, 0);
            return n0 as u32;
        }
        callee_cdecl!(T1_ALLOC, u32, 3, (n0 as u32).wrapping_mul(6));
        // Re-read like the original; under scripted callees this still holds
        // the entry count, so the early-out below never fires on either side.
        let n1 = (p2 as *const i32).read_unaligned();
        if n1 <= 0 {
            let ans = callee_cdecl!(T3_DONE, u32,);
            wr32(p2, 0);
            return ans;
        }
        let n = n1 as u32;
        let mut k = 0u32;
        while k < n {
            let u = rf32(p0.wrapping_add(k.wrapping_mul(16)));
            let v = rf32(p0.wrapping_add(k.wrapping_mul(16)).wrapping_add(4));
            let w = rf32(p0.wrapping_add(k.wrapping_mul(16)).wrapping_add(8));
            let esi = rd32(p1.wrapping_add(k.wrapping_mul(4)));
            let c1 = (u - d0, v - d1, w - d2);
            let c2 = (d0 + u, d1 + v, d2 + w);
            let c3 = (u - s0, v - s1, w - s2);
            let c5 = (s0 + u, s1 + v, s2 + w);
            callee_cdecl!(T2_APPLY, u32, c1.0.to_bits(), c1.1.to_bits(), c1.2.to_bits(),
                0, 0, 0xBF80_0000, esi, 0, 0x3F80_0000);
            callee_cdecl!(T2_APPLY, u32, c2.0.to_bits(), c2.1.to_bits(), c2.2.to_bits(),
                0, 0, 0xBF80_0000, esi, 0x3F80_0000, 0);
            callee_cdecl!(T2_APPLY, u32, c3.0.to_bits(), c3.1.to_bits(), c3.2.to_bits(),
                0, 0, 0xBF80_0000, esi, 0, 0);
            callee_cdecl!(T2_APPLY, u32, c2.0.to_bits(), c2.1.to_bits(), c2.2.to_bits(),
                0, 0, 0xBF80_0000, esi, 0x3F80_0000, 0);
            callee_cdecl!(T2_APPLY, u32, c5.0.to_bits(), c5.1.to_bits(), c5.2.to_bits(),
                0, 0, 0xBF80_0000, esi, 0x3F80_0000, 0x3F80_0000);
            callee_cdecl!(T2_APPLY, u32, c1.0.to_bits(), c1.1.to_bits(), c1.2.to_bits(),
                0, 0, 0xBF80_0000, esi, 0, 0x3F80_0000);
            k += 1;
        }
        let ans = callee_cdecl!(T3_DONE, u32,);
        wr32(p2, 0);
        ans
    }
});
