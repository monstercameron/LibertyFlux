// original: 0x00a130d0 pending_fix_apply (proposed)
/// Apply the pending fix, resolving it to an output record.
///
/// When the byte at `this + 0x2f8` is clear, or the byte at `this + 0x141`
/// is set, clears `+0x141` and returns 0 (low byte). Otherwise adds `delta`
/// to the float at `this + 0x2f0`, publishes that sum at `this + 0x190` and
/// the float at `this + 0x2f4` at `this + 0x194`, resolves the pair through
/// the callee, copies the four returned words to `out`, clears `+0x2f8`
/// and returns 1. Thiscall, two stack arguments.
export!(thiscall, rw_00a130d0(this: u32, out: u32, delta: u32) -> u32 {
    unsafe {
        const RESOLVE: u32 = 1;
        const PENDING_OFF: u32 = 0x2f8;
        const HELD_OFF: u32 = 0x141;
        const ACCUM_OFF: u32 = 0x2f0;
        const SIDE_OFF: u32 = 0x2f4;
        const PUB_A_OFF: u32 = 0x190;
        const PUB_B_OFF: u32 = 0x194;
        if ((this + PENDING_OFF) as *const u8).read() == 0 {
            ((this + HELD_OFF) as *mut u8).write(0);
            return 0;
        }
        if ((this + HELD_OFF) as *const u8).read() != 0 {
            ((this + HELD_OFF) as *mut u8).write(0);
            return 0;
        }
        let base = f32::from_bits(((this + ACCUM_OFF) as *const u32).read_unaligned());
        let x1 = core::hint::black_box(base) + core::hint::black_box(f32::from_bits(delta));
        let x0 = f32::from_bits(((this + SIDE_OFF) as *const u32).read_unaligned());
        ((this + PUB_A_OFF) as *mut u32).write_unaligned(x1.to_bits());
        ((this + PUB_B_OFF) as *mut u32).write_unaligned(x0.to_bits());
        let mut scratch = [0u32; 4];
        let r = callee_cdecl!(
            RESOLVE,
            u32,
            &mut scratch as *mut u32 as u32,
            x1.to_bits(),
            x0.to_bits()
        );
        for k in 0..4u32 {
            let w = ((r + 4 * k) as *const u32).read_unaligned();
            ((out + 4 * k) as *mut u32).write_unaligned(w);
        }
        ((this + PENDING_OFF) as *mut u8).write(0);
        1
    }
});
