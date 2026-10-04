// original: 0x00c5c590 task_timer_blend_value (proposed)

/// Blend a fresh sample into a task timer entry's smoothed value.
///
/// Looks the entry up in the global table by the index at `+4`, runs callee 1
/// on it, takes callee 2's float answer `r`, and stores
/// `cur * 0.95 + r * 0.05` back into the entry, with the multiplications in
/// the original's operand order. Both callees take the entry address plus 8
/// in ecx. Returns the entry address.
///
/// Original: 0x00c5c590 (thiscall: `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_00c5c590(this: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x12b60a0;
        const INDEX: u32 = 4;
        const SAMPLE_SLOT: u32 = 0xfe876c; // 0.05
        const KEEP_SLOT: u32 = 0xfe88c4; // 0.95
        const POLL: u32 = 1;
        const SAMPLE: u32 = 2;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let index = ((this + INDEX) as *const u32).read_unaligned();
        let base = lf_checker_rt::relocated(TABLE);
        let entry = ((base.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(POLL, u32, entry.wrapping_add(8));
        let r: f32 = lf_checker_rt::callee_thiscall!(SAMPLE, f32, entry.wrapping_add(8));
        let k0 = f32::from_bits(lf_checker_rt::global::<u32>(SAMPLE_SLOT).read());
        let k1 = f32::from_bits(lf_checker_rt::global::<u32>(KEEP_SLOT).read());
        let cur = f32::from_bits((entry as *const u32).read_unaligned());
        let fresh = mul(r, k0);
        let kept = mul(cur, k1);
        (entry as *mut u32).write_unaligned(add(kept, fresh).to_bits());
        entry
    }
});

