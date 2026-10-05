// original: 0x00AD4F20 audio_init_slot_pointers (proposed)

/// Initialise one audio slot's pointers and zero its words.
///
/// Derives three pointers from the registry base (cdecl/0) and the slot
/// index: `base + (slot + 0x3e3) * 4`, `base + (slot + 0x3fb) * 4` and
/// `base + slot * 4 + 0x104c`, publishes them to their globals and zeroes
/// the pointed-to words (re-reading each global first). Then zeroes the
/// slot's words at offsets 0xe6c, 0xecc and 0xf2c off the registry base.
/// Cdecl/1; returns the registry base (the last helper answer).
lf_checker_rt::export!(cdecl, rw_00ad4f20(slot: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 1;
        const P0_GLOBAL: u32 = 0x01550DE8;
        const P1_GLOBAL: u32 = 0x01550DEC;
        const P2_GLOBAL: u32 = 0x01550DF0;
        const P0_BIAS: u32 = 0x3E3;
        const P1_BIAS: u32 = 0x3FB;
        const P2_BIAS: u32 = 0x104C;
        const Z0_OFF: u32 = 0xE6C;
        const Z1_OFF: u32 = 0xECC;
        const Z2_OFF: u32 = 0xF2C;
        let base = lf_checker_rt::callee_cdecl!(REGISTRY, u32,);
        let p0 = base.wrapping_add(slot.wrapping_add(P0_BIAS).wrapping_mul(4));
        let p1 = base.wrapping_add(slot.wrapping_add(P1_BIAS).wrapping_mul(4));
        let p2 = base.wrapping_add(slot.wrapping_mul(4)).wrapping_add(P2_BIAS);
        lf_checker_rt::global::<u32>(P2_GLOBAL).write(p2);
        lf_checker_rt::global::<u32>(P1_GLOBAL).write(p1);
        lf_checker_rt::global::<u32>(P0_GLOBAL).write(p0);
        (lf_checker_rt::global::<u32>(P0_GLOBAL).read() as *mut u32).write(0);
        (lf_checker_rt::global::<u32>(P1_GLOBAL).read() as *mut u32).write(0);
        (lf_checker_rt::global::<u32>(P2_GLOBAL).read() as *mut u32).write(0);
        let b1 = lf_checker_rt::callee_cdecl!(REGISTRY, u32,);
        (b1.wrapping_add(slot.wrapping_mul(4)).wrapping_add(Z0_OFF) as *mut u32).write(0);
        let b2 = lf_checker_rt::callee_cdecl!(REGISTRY, u32,);
        (b2.wrapping_add(slot.wrapping_mul(4)).wrapping_add(Z1_OFF) as *mut u32).write(0);
        let b3 = lf_checker_rt::callee_cdecl!(REGISTRY, u32,);
        (b3.wrapping_add(slot.wrapping_mul(4)).wrapping_add(Z2_OFF) as *mut u32).write(0);
        b3
    }
});
