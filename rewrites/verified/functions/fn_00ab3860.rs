// original: 0x00ab3860 stream_init_request (proposed)

/// Build one streaming request for source `a0` and stamp a random delay.
///
/// Registers the `(a0, a1, a2)` triple through the register callee on the
/// loader singleton, then fills the request at `this`: owner word from the
/// singleton, clear flags, the source's tag byte (which also gates the
/// mode byte: `a3`'s low byte when the tag is clear, else 0), a 36-byte
/// copy of the source body at `+0x50`, and a delay word at `+0x7c` drawn
/// from the global multiply-add generator (`seed = seed * 0x5CDCFAA7 +
/// carry`, 64-bit) as `(seed & 0x7FFFFF) * C0 * C1 + C2` in the original's
/// float order. No return value.
///
/// Callees: 1 = triple register (thiscall, three words).
///
/// Original: 0x00ab3860 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00ab3860(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const REGISTER: u32 = 1;
        const SINGLETON: u32 = 0x013B_ABA0;
        const SEED_LO: u32 = 0x0111_01A0;
        const SEED_HI: u32 = 0x0111_01A4;
        const MULT: u64 = 0x5CDC_FAA7;
        const FRAC_MASK: u32 = 0x7F_FFFF;
        const C0: f32 = f32::from_bits(0x3400_0000);
        const C1: f32 = f32::from_bits(0x3E4C_CCCC);
        const C2: f32 = f32::from_bits(0x3F4C_CCCD);
        const TAG_OFF: u32 = 0x46;
        const BODY_OFF: u32 = 0x50;
        const BODY_BYTES: u32 = 36;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let loader = lf_checker_rt::relocated(SINGLETON);
        lf_checker_rt::callee_thiscall!(REGISTER, u32, loader, a0, a1, a2);
        let owner = (loader as *const u32).read_unaligned();
        ((this + 8) as *mut u32).write_unaligned(owner);
        ((this + 0x80) as *mut u8).write(0);
        ((this + 0x4C) as *mut u32).write_unaligned(0);
        let tag = ((a0 + TAG_OFF) as *const u8).read();
        ((this + 0x50) as *mut u8).write(tag);
        let mode = if tag != 0 { 0 } else { a3 as u8 };
        ((this + 0x51) as *mut u8).write(mode);
        let mut o = 0u32;
        while o < BODY_BYTES {
            let step = if o + 8 <= BODY_BYTES { 8 } else { 4 };
            if step == 8 {
                let v = ((a0 + BODY_OFF + o) as *const u64).read_unaligned();
                ((this + 0x0C + o) as *mut u64).write_unaligned(v);
            } else {
                let v = ((a0 + BODY_OFF + o) as *const u32).read_unaligned();
                ((this + 0x0C + o) as *mut u32).write_unaligned(v);
            }
            o += step;
        }
        ((this + 0x48) as *mut u32).write_unaligned(0);
        let s0 = (lf_checker_rt::global::<u32>(SEED_LO) as *const u32).read_unaligned();
        let s1 = (lf_checker_rt::global::<u32>(SEED_HI) as *const u32).read_unaligned();
        let t = (s0 as u64).wrapping_mul(MULT).wrapping_add(s1 as u64);
        (lf_checker_rt::global::<u32>(SEED_LO) as *mut u32).write_unaligned(t as u32);
        (lf_checker_rt::global::<u32>(SEED_HI) as *mut u32)
            .write_unaligned((t >> 32) as u32);
        let base = ((t as u32) & FRAC_MASK) as f32;
        let delay = add(mul(mul(base, C0), C1), C2);
        ((this + 0x7C) as *mut u32).write_unaligned(delay.to_bits());
        0
    }
});
