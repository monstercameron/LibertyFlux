// original: 0x009e7000 ped_scaled_index (proposed)

/// Cached index, or a freshly computed scaled product, truncated.
///
/// Returns the cached word at `this + 0xe44` unless it is -1. Otherwise
/// probes (`thiscall` on this, no stack words): a nonzero low byte runs
/// the cdecl runner `(this, 0)` and returns its answer. A zero byte maps
/// (`cdecl` `(this, 0, a1)`, low byte kept), scales (`cdecl`, no args,
/// low 16 bits kept) and returns `trunc((scale & 0xffff) * K * map)`
/// with the multiplications in the original's order, reproducing
/// `cvttss2si` exactly (0x80000000 for NaN or out-of-range). `thiscall`,
/// one stack word.
lf_checker_rt::export!(thiscall, rw_009e7000(this: u32, a1: u32) -> u32 {
    unsafe {
        const CACHED: u32 = 0xe44;
        const K_MUL: u32 = 0x00fe8680;
        const PROBE: u32 = 1;
        const RUN: u32 = 2;
        const MAP: u32 = 3;
        const SCALE_SRC: u32 = 4;
        let c = ((this + CACHED) as *const u32).read_unaligned();
        if c != 0xffffffff {
            return c;
        }
        let probe = lf_checker_rt::callee_thiscall!(PROBE, u32, this);
        if (probe & 0xff) != 0 {
            return lf_checker_rt::callee_cdecl!(RUN, u32, this, 0u32);
        }
        let m = lf_checker_rt::callee_cdecl!(MAP, u32, this, 0u32, a1);
        // Zero-argument cdecl call (the callee macro needs at least one
        // argument, so this goes through the stub table directly).
        let scale_fn: extern "cdecl" fn() -> u32 =
            unsafe { core::mem::transmute(lf_checker_rt::callee_addr(SCALE_SRC) as usize) };
        let s = scale_fn();
        let lo = (m & 0xff) as f32;
        let hi = (s & 0xffff) as f32;
        let k = f32::from_bits(lf_checker_rt::global::<u32>(K_MUL).read());
        let p1 = core::hint::black_box(hi) * core::hint::black_box(k);
        let p2 = core::hint::black_box(p1) * core::hint::black_box(lo);
        // cvttss2si: 0x80000000 when NaN, >= 2^31, or < -2^31.
        if p2.is_nan() || p2 >= 2147483648.0f32 || p2 < -2147483648.0f32 {
            0x80000000u32
        } else {
            p2 as i32 as u32
        }
    }
});
