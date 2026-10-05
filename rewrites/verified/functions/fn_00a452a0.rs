// original: 0x00a452a0 NativeImpl_SET_CAR_CAN_BE_DAMAGED
/// Refresh the damage state: sample, apply, clamp two floats, visit slots.
///
/// Unless gated out (bits 0x7C00 of +0x28 equal 0xC00, or a linked object
/// with a set byte at +0xE), samples the float hook (vtable +0xFC,
/// thiscall/0, ST0 float), re-samples when above 300.0, and applies the
/// value (vtable +0xF4, thiscall/2 on `(this, bits, 0)`). Then notifies the
/// registry (thiscall/1 on the shared object), resets +0x10AC/+0x10D8 to
/// (-4000.0, 1.0) when out of range, and visits `count` stride-0x170 slots
/// from +0xF80 (thiscall/0 each; thiscall, no stack arguments; no
/// meaningful return value).
export!(thiscall, rw_00a452a0(this: u32) -> u32 {
    unsafe {
        const GATE_MASK: u32 = 0x7c00;
        const GATE_SKIP: u32 = 0xc00;
        const SAMPLE_SLOT: u32 = 0xfc;
        const APPLY_SLOT: u32 = 0xf4;
        let mut skip = false;
        if ((this.wrapping_add(0x28) as *const u32).read_unaligned() & GATE_MASK) == GATE_SKIP {
            skip = true;
        } else {
            let inner = (this.wrapping_add(0x6c) as *const u32).read_unaligned();
            if inner != 0 && ((inner.wrapping_add(0xe)) as *const u8).read() != 0 {
                skip = true;
            }
        }
        if !skip {
            let vt = (this as *const u32).read_unaligned();
            let sample: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(
                ((vt.wrapping_add(SAMPLE_SLOT)) as *const u32).read_unaligned() as usize,
            );
            let v0 = sample(this);
            let hi = f32::from_bits((relocated(0x00fe8c10) as *const u32).read_unaligned());
            let mut feed = hi;
            if core::hint::black_box(v0) > core::hint::black_box(hi) {
                feed = sample(this);
            }
            let vt2 = (this as *const u32).read_unaligned();
            let apply: extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
                ((vt2.wrapping_add(APPLY_SLOT)) as *const u32).read_unaligned() as usize,
            );
            let _: u32 = apply(this, feed.to_bits(), 0);
        }
        let _: u32 = callee_thiscall!(1, u32, relocated(0x012e2420), this);
        let lo = f32::from_bits((this.wrapping_add(0x10ac) as *const u32).read_unaligned());
        let hi2 = f32::from_bits((this.wrapping_add(0x10d8) as *const u32).read_unaligned());
        let neg4000 = f32::from_bits((relocated(0x00e9cae8) as *const u32).read_unaligned());
        let neg1000 = f32::from_bits((relocated(0x00fe8e04) as *const u32).read_unaligned());
        // Reset when the low word is strictly between -4000 and 0, or the
        // high word is strictly between -1000 and 0; the second check also
        // runs when the low word is not below zero at all (fall-through).
        let mut reset = false;
        if core::hint::black_box(0.0f32) > core::hint::black_box(lo)
            && core::hint::black_box(lo) > core::hint::black_box(neg4000)
        {
            reset = true;
        } else if core::hint::black_box(hi2) > core::hint::black_box(neg1000)
            && core::hint::black_box(0.0f32) > core::hint::black_box(hi2)
        {
            reset = true;
        }
        if reset {
            (this.wrapping_add(0x10d8) as *mut u32).write_unaligned(0x3f800000);
            (this.wrapping_add(0x10ac) as *mut u32).write_unaligned(0xc57a0000);
        }
        let count = (this.wrapping_add(0xf84) as *const i32).read_unaligned();
        if count > 0 {
            let base = (this.wrapping_add(0xf80) as *const u32).read_unaligned();
            let mut i = 0i32;
            let mut off = 0u32;
            loop {
                let cnt = (this.wrapping_add(0xf84) as *const i32).read_unaligned();
                let target = if i >= cnt { 0 } else { base.wrapping_add(off) };
                let _: u32 = callee_thiscall!(2, u32, target);
                i += 1;
                off = off.wrapping_add(0x170);
                let cnt2 = (this.wrapping_add(0xf84) as *const i32).read_unaligned();
                if !(i < cnt2) {
                    break;
                }
            }
        }
        0
    }
});
