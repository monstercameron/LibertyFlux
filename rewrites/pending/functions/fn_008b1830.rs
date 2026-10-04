// original: 0x008b1830 rage::audBiquadFilterEffectPc::vf2
/// Biquad filter process: run the filter over `count` samples for every
/// enabled channel.
///
/// `this` is the effect object. Coefficient set `(slot + 2) % 3` is selected
/// from the triple at +0x0C and stored back at +0x04. When the bypass byte at
/// +0xB8 is set nothing else happens. Otherwise each channel `ch` below the
/// count at +0xC0 whose mask bit (shift taken from a table, tested against
/// the byte at +0xC2) is set has its sample block filtered in place: the
/// single-stage form when the byte at +0xC3 is clear, the cascaded two-stage
/// form otherwise, scaled by the gain at +0xB4. Channel state lives at
/// +0x48/+0x60/+0x78/+0x90, one word per active channel, and is flushed for
/// denormals on every active channel even when `count` is zero.
///
/// The sample loops route every add, multiply and subtract through the
/// NaN-guarded helpers below: when both operands are NaN the hardware keeps
/// the destination operand's payload, so the operand order is part of the
/// behaviour and the plain operators cannot be trusted with it (the compiler
/// commutes them, including inside SSE intrinsics).
/// Quiet a NaN the way hardware propagation does (set the quiet bit).
#[inline(always)]
fn quiet(v: f32) -> f32 {
    f32::from_bits(v.to_bits() | 0x0040_0000)
}

/// Scalar float ops with the original's exact NaN propagation: when both
/// operands are NaN the hardware keeps the destination operand's payload,
/// but the compiler treats add/mul as commutative and may emit either order
/// (this was observed with both plain operators and SSE intrinsics), so the
/// NaN cases are decided explicitly and only NaN-free inputs reach hardware.
#[inline(always)]
fn fmul(dest: f32, src: f32) -> f32 {
    if dest.is_nan() {
        quiet(dest)
    } else if src.is_nan() {
        quiet(src)
    } else {
        dest * src
    }
}

#[inline(always)]
fn fadd(dest: f32, src: f32) -> f32 {
    if dest.is_nan() {
        quiet(dest)
    } else if src.is_nan() {
        quiet(src)
    } else {
        dest + src
    }
}

#[inline(always)]
fn fsub(dest: f32, src: f32) -> f32 {
    if dest.is_nan() {
        quiet(dest)
    } else if src.is_nan() {
        quiet(src)
    } else {
        dest - src
    }
}

/// Flush a denormal (or zero) filter state word to +0.0.
///
/// The original tests the stored word's exponent bits and zeroes the whole
/// word when they are all clear, so -0.0 also becomes +0.0.
#[inline(always)]
fn flush_denormal(x: f32) -> f32 {
    if x.to_bits() & 0x7F80_0000 == 0 {
        0.0
    } else {
        x
    }
}

export!(thiscall, rw_008B1830(this: *mut u8, buf: *mut f32, count: u32) -> () {
    unsafe {
        let slot = (*(this.add(0x08) as *const u32)).wrapping_add(2) % 3;
        *(this.add(0x04) as *mut u32) = slot;
        if *this.add(0xB8) != 0 {
            return;
        }
        let k = slot as usize * 4;
        let b0 = *(this.add(0x0C + k) as *const f32);
        let b1 = *(this.add(0x18 + k) as *const f32);
        let b2 = *(this.add(0x24 + k) as *const f32);
        let a1 = *(this.add(0x30 + k) as *const f32);
        let a2 = *(this.add(0x3C + k) as *const f32);
        if *this.add(0xC0) == 0 {
            return;
        }
        let mut cursor = buf;
        let mut state = this.add(0x60);
        let mut ch: u32 = 0;
        loop {
            let shift = *global::<u32>(0xE7CC08 + ch * 4);
            let bit = 1u32.wrapping_shl(shift) as u8;
            if *this.add(0xC2) & bit != 0 {
                let two_stage = *this.add(0xC3) != 0;
                let mut s1 = *(state.sub(0x18) as *const f32);
                let mut s2 = *(state as *const f32);
                let mut s3 = *(state.add(0x18) as *const f32);
                let mut s4 = *(state.add(0x30) as *const f32);
                if count != 0 {
                    let mut p = cursor;
                    let mut left = count;
                    if two_stage {
                        loop {
                            let x = *p;
                            let gain = *(this.add(0xB4) as *const f32);
                            let t2 = fadd(fmul(x, b0), s1);
                            let w = fadd(fmul(t2, b0), s3);
                            s1 = fadd(fsub(fmul(x, b1), fmul(t2, a1)), s2);
                            s3 = fadd(fsub(fmul(t2, b1), fmul(w, a1)), s4);
                            s2 = fsub(fmul(x, b2), fmul(t2, a2));
                            s4 = fsub(fmul(t2, b2), fmul(w, a2));
                            *p = fmul(gain, w);
                            p = p.add(1);
                            left = left.wrapping_sub(1);
                            if left == 0 {
                                break;
                            }
                        }
                    } else {
                        loop {
                            let x = *p;
                            let gain = *(this.add(0xB4) as *const f32);
                            let t2 = fadd(fmul(x, b0), s1);
                            s1 = fadd(fsub(fmul(x, b1), fmul(t2, a1)), s2);
                            s2 = fsub(fmul(x, b2), fmul(t2, a2));
                            *p = fmul(gain, t2);
                            p = p.add(1);
                            left = left.wrapping_sub(1);
                            if left == 0 {
                                break;
                            }
                        }
                    }
                }
                *(state.sub(0x18) as *mut f32) = flush_denormal(s1);
                *(state as *mut f32) = flush_denormal(s2);
                *(state.add(0x18) as *mut f32) = flush_denormal(s3);
                *(state.add(0x30) as *mut f32) = flush_denormal(s4);
                state = state.add(4);
            }
            cursor = (cursor as u32).wrapping_add(count.wrapping_mul(4)) as *mut f32;
            ch = ch.wrapping_add(1);
            if ch >= *this.add(0xC0) as u32 {
                break;
            }
        }
    }
});
