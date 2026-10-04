// original: 0x009e5a70 ped_stamped_blend (proposed)

/// Blend factor from the age of a stamped value, or 1.0 when unset/stale.
///
/// Returns 1.0 (`fld1`, via ST0) when the valid flag at `this + 0xa6c` is
/// clear. Otherwise reads the global tick and the stamp at `this + 0xa68`:
/// when the tick is not exactly `stamp + 0x1f4` the flag is cleared and
/// 1.0 is returned. On an exact match it returns
/// `(float(tick) - float(stamp)) * RATE` in single precision, with both
/// words converted as *unsigned* 32-bit values (the original converts via
/// double with a 2^32 add-back for the sign bit) and the subtraction and
/// multiplication in the original's order. `thiscall`, no stack words.
lf_checker_rt::export!(thiscall, rw_009e5a70(this: u32) -> f32 {
    unsafe {
        const STAMP: u32 = 0xa68;
        const FLAG: u32 = 0xa6c;
        const WINDOW: u32 = 0x1f4;
        const COUNTER: u32 = 0x011735b4;
        const RATE: u32 = 0x00fe86d0;
        if ((this + FLAG) as *const u8).read() == 0 {
            return 1.0;
        }
        let now = lf_checker_rt::global::<u32>(COUNTER).read();
        let then = ((this + STAMP) as *const u32).read_unaligned();
        if now != then.wrapping_add(WINDOW) {
            ((this + FLAG) as *mut u8).write(0);
            return 1.0;
        }
        let k = f32::from_bits(lf_checker_rt::global::<u32>(RATE).read());
        let a = now as f32;
        let b = then as f32;
        let d = core::hint::black_box(a) - core::hint::black_box(b);
        core::hint::black_box(d) * core::hint::black_box(k)
    }
});
