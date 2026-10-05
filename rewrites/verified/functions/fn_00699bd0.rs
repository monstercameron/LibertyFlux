// original: 0x00699BD0 rage::crAnimChannelQuantizeFloat::serialize

/// Serializes a quantized-float channel through a versioned serializer.
///
/// `this` is the channel object (bit-set struct at `+8`, two parameter
/// words at `+0x14`/`+0x18`); the stack argument is the serializer (flag
/// byte at `+0`, version word at `+2`, stream object at `+4`). The two
/// serializer primitives (callees 1-6, thiscall: stream, buffer, byte
/// size) move the parameter words, then a version-4-or-later serializer
/// (unsigned compare) is handed to the modern routine (callee 7, thiscall:
/// bit-set, serializer) and the function returns. Older versions take the
/// legacy path: the item count word and each item word pass through the
/// function's own dead argument slot and a scratch slot (those pointer
/// arguments are skipped and their contents snapshotted), the bit-set
/// builder (callee 8, thiscall: bit-set, width, count) installs the bit
/// buffer, and each item is masked to the planted width and inserted at
/// `width * index` bits (single-word or straddling two words, unsigned
/// bound compare). The width is planted beyond the real constant 16
/// (7/20/31) on some trials to exercise the two-word insert, which is dead
/// with the real width; this is listed as narrowed. Finally the first
/// parameter word is scaled by a global constant (buffer first). The loop
/// entry uses an unsigned `jae` against zero and the exit a signed `jl`;
/// both agree on the small contract counts. No return value.
///
/// Original: 0x00699BD0 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00699BD0(this: u32, ser: u32) -> u32 {
    unsafe {
        const SER_OBJ: u32 = 4;
        const SER_VER: u32 = 2;
        const BITS_OFF: u32 = 8;
        const P0_OFF: u32 = 0x14;
        const P1_OFF: u32 = 0x18;
        const SCALE_VA: u32 = 0xFE867C;
        const OP12_A: u32 = 1;
        const OP12_B: u32 = 2;
        const CNT_A: u32 = 3;
        const CNT_B: u32 = 4;
        const ITM_A: u32 = 5;
        const ITM_B: u32 = 6;
        const V4: u32 = 7;
        const INIT: u32 = 8;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let flag = unsafe { (ser as *const u8).read() } & 1;
        let sobj = rd32(ser + SER_OBJ);
        if flag != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OP12_A, u32, sobj, this + P0_OFF, 4);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(OP12_B, u32, sobj, this + P0_OFF, 4);
        }
        if flag != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OP12_A, u32, sobj, this + P1_OFF, 4);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(OP12_B, u32, sobj, this + P1_OFF, 4);
        }
        let ver = rd16(ser + SER_VER);
        if ver >= 4 {
            let _ = lf_checker_rt::callee_thiscall!(V4, u32, this + BITS_OFF, ser);
            return 0;
        }
        // The original reuses its incoming serializer slot as the count word
        // and one fixed scratch slot for every item word; mirror both so the
        // call-time snapshots match.
        let mut count_slot: u32 = ser;
        if flag != 0 {
            let _ = lf_checker_rt::callee_thiscall!(CNT_A, u32, sobj,
                core::ptr::addr_of_mut!(count_slot) as u32, 2);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(CNT_B, u32, sobj,
                core::ptr::addr_of_mut!(count_slot) as u32, 2);
        }
        let count = count_slot & 0xFFFF;
        let _ = lf_checker_rt::callee_thiscall!(INIT, u32, this + BITS_OFF, 0x10, count);
        let mut i: u32 = 0;
        let mut item_slot: u32 = 0;
        while (i as i32) < (count as i32) {
            if flag != 0 {
                let _ = lf_checker_rt::callee_thiscall!(ITM_A, u32, sobj,
                    core::ptr::addr_of_mut!(item_slot) as u32, 2);
            } else {
                let _ = lf_checker_rt::callee_thiscall!(ITM_B, u32, sobj,
                    core::ptr::addr_of_mut!(item_slot) as u32, 2);
            }
            let bw = rd32(this + BITS_OFF + 4);
            let bitpos = bw.wrapping_mul(i);
            let cl = 32u32.wrapping_sub(bw) as u8;
            let mask = 0xFFFFFFFFu32.wrapping_shr((cl & 31) as u32);
            let v = (item_slot & 0xFFFF) & mask;
            let widx = bitpos >> 5;
            let bit = bitpos & 31;
            let base = rd32(this + BITS_OFF);
            if bit <= 32u32.wrapping_sub(bw) {
                let m = mask.wrapping_shl(bit);
                let vv = v.wrapping_shl(bit);
                let old = rd32(base.wrapping_add(widx.wrapping_mul(4)));
                wr32(base.wrapping_add(widx.wrapping_mul(4)), (old & !m) | vv);
            } else {
                let m1 = 0xFFFFFFFFu32.wrapping_shl(bit);
                let vv = v.wrapping_shl(bit);
                let old1 = rd32(base.wrapping_add(widx.wrapping_mul(4)));
                wr32(base.wrapping_add(widx.wrapping_mul(4)), (old1 & !m1) | vv);
                let cl2 = 32u32.wrapping_sub(bit);
                let v2 = v.wrapping_shr(cl2 & 31);
                let m2 = 0xFFFFFFFFu32.wrapping_shr(cl2 & 31);
                let old2 = rd32(base.wrapping_add(widx.wrapping_mul(4)).wrapping_add(4));
                wr32(base.wrapping_add(widx.wrapping_mul(4)).wrapping_add(4), (old2 & !m2) | v2);
            }
            i = i.wrapping_add(1);
        }
        let b = f32::from_bits(rd32(this + P0_OFF));
        let k = f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_VA)));
        wr32(this + P0_OFF, fmul(b, k).to_bits());
        0
    }
});
