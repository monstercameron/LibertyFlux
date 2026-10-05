// original: 0x00BBB3A0 register_directional_slot_and_notify (proposed)

/// Fill one slot of the directional table and notify its owner.
///
/// Callee 1 resolves a slot index from -1.0; an index above 63 writes 0
/// through the `out` pointer and returns with nothing else done.
///
/// Otherwise the two angles `a14`/`a18` are scaled to radians in place
/// (so the stack check is off; the scaled values are compared as vector
/// call arguments). Each angle feeds the sine/cosine pair (callees 2 and 3,
/// float in and out of XMM0 with the rewrite passing its input on the
/// stack): the first answer is sign-flipped, the pair is normalised by the
/// reciprocal of its length (an exact zero length gives 0, anything else,
/// NaN included, takes `1/sqrt`), and the normalised triple (with a zero
/// third lane) is staged. All multiplies, the square root and the divide
/// run in the original's operand order with the order pinned.
///
/// Slot `idx` of the table at `DIR_TABLE` (stride `SLOT_STRIDE`) then gets
/// `a8`/`ac`/`b10`, a zero word where the original stores an unstaged frame
/// word (verified zero under a zero stack fill), the first triple twice,
/// the second triple once, a kind byte of 8, the `b20`/`b1c` bytes, padding
/// and a trailing -1.0. A handle (callee 4) is written through `out`, and
/// the table owner (global at `OWNER`) is notified (callee 6) with
/// `(handle, 8, token)` where the token comes from callee 5.
///
/// Original: 0x00BBB3A0 (cdecl, eight stack words, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00BBB3A0(a8: u32, ac: u32, b10: u32, a14: u32, a18: u32, b1c: u32, b20: u32, out: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x012BD0C4;
        const DIR_TABLE: u32 = 0x0167CEA0;
        const DEG_TO_RAD: u32 = 0x00FE8728;
        const NEG_ZERO: u32 = 0x00FE8FA0;
        const ONE_F: u32 = 0x00FE88E8;
        const MAX_SLOT: u32 = 0x3F;
        const SLOT_KIND: u8 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let idx: u32 = lf_checker_rt::callee_cdecl!(1, u32, 0xBF800000u32);
        if idx > MAX_SLOT {
            wr32(out, 0);
            return 0;
        }
        let deg = f32::from_bits(rd32(lf_checker_rt::relocated(DEG_TO_RAD)));
        let negz = rd32(lf_checker_rt::relocated(NEG_ZERO));
        let one = f32::from_bits(rd32(lf_checker_rt::relocated(ONE_F)));
        let arad1 = mul(f32::from_bits(a14), deg);
        let s1 = f32::from_bits(lf_checker_rt::callee_cdecl!(2, u32, arad1.to_bits()));
        let t1 = f32::from_bits(s1.to_bits() ^ negz);
        let t2 = f32::from_bits(lf_checker_rt::callee_cdecl!(3, u32, arad1.to_bits()));
        let n2 = add(mul(t2, t2), mul(t1, t1));
        let inv = if n2 == 0.0 { 0.0 } else { div(one, core::hint::black_box(n2).sqrt()) };
        let nx1 = mul(inv, t1);
        let ny1 = mul(inv, t2);
        let nz1 = mul(inv, 0.0);
        let arad2 = mul(f32::from_bits(a18), deg);
        let u1 = f32::from_bits(lf_checker_rt::callee_cdecl!(2, u32, arad2.to_bits()));
        let v1 = f32::from_bits(u1.to_bits() ^ negz);
        let v2 = f32::from_bits(lf_checker_rt::callee_cdecl!(3, u32, arad2.to_bits()));
        let m2 = add(mul(v2, v2), mul(v1, v1));
        let jnv = if m2 == 0.0 { 0.0 } else { div(one, core::hint::black_box(m2).sqrt()) };
        let nx2 = mul(jnv, v1);
        let ny2 = mul(jnv, v2);
        let nz2 = mul(jnv, 0.0);
        let slot = lf_checker_rt::relocated(DIR_TABLE)
            .wrapping_add(idx.wrapping_mul(5).wrapping_mul(16));
        wr32(slot, a8);
        wr32(slot.wrapping_add(4), ac);
        wr32(slot.wrapping_add(8), b10);
        wr32(slot.wrapping_add(0x0C), 0);
        wr32(slot.wrapping_add(0x10), nx1.to_bits());
        wr32(slot.wrapping_add(0x14), ny1.to_bits());
        wr32(slot.wrapping_add(0x18), nz1.to_bits());
        wr32(slot.wrapping_add(0x1C), nx1.to_bits());
        wr32(slot.wrapping_add(0x20), ny1.to_bits());
        wr32(slot.wrapping_add(0x24), nz1.to_bits());
        wr32(slot.wrapping_add(0x28), nx2.to_bits());
        wr32(slot.wrapping_add(0x2C), ny2.to_bits());
        wr32(slot.wrapping_add(0x30), nz2.to_bits());
        wr8(slot.wrapping_add(0x34), SLOT_KIND);
        wr8(slot.wrapping_add(0x35), b20 as u8);
        wr8(slot.wrapping_add(0x36), 0);
        wr8(slot.wrapping_add(0x38), b1c as u8);
        wr8(slot.wrapping_add(0x39), 0xFF);
        wr32(slot.wrapping_add(0x3C), 0);
        wr32(slot.wrapping_add(0x40), 0);
        wr32(slot.wrapping_add(0x44), 0);
        wr32(slot.wrapping_add(0x48), 0xBF800000);
        let handle: u32 = lf_checker_rt::callee_cdecl!(4, u32, idx, 6);
        wr32(out, handle);
        let owner = rd32(lf_checker_rt::relocated(OWNER));
        let token: u32 = lf_checker_rt::callee_cdecl!(5, u32, 0, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, owner, handle, 8, token);
        0
    }
});
