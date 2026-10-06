// original: 0x00963000 tick_delta_scale_store_34_30_38
/// Scale a tick delta by 0.001 and store it as float (offsets 0x34/0x30/0x38).
///
/// Takes one `tick` word. Fetches the stats object three times through the
/// query helper (thiscall/1, ECX and the stack word both from globals),
/// stores `tick` at `+0x34`, loads the reference at `+0x30`, and stores
/// `(delta as u32 as f32) * 0.001` at `+0x38`, where delta is tick minus reference;
/// when `tick` is `-1` (equality with `0xFFFFFFFF`) it stores and returns the object early. Returns the last object pointer.
/// The float conversion is exact: the original widens the signed delta to
/// double and adds 2^32 when negative, which equals the wrapping (unsigned)
/// delta exactly; one rounding to float follows, then one multiply. The
/// rewrite converts the wrapping delta directly, with the multiply's operand
/// order pinned. The original reuses its incoming argument slot as the float
/// temporary, so the stack comparison is off; the value is observed in the
/// `+0x38` store instead.
lf_checker_rt::export!(cdecl, rw_00963000(tick: u32) -> u32 {
    unsafe {
        const QUERY_ECX: u32 = 0x11f6954;
        const QUERY_ARG: u32 = 0x11f6f34;
        const STORE: u32 = 0x34;
        const LOAD: u32 = 0x30;
        const FLOAT_OUT: u32 = 0x38;
        const SCALE: f32 = f32::from_bits(0x3a83126f); // 0.001
        const TWO32: f64 = 4294967296.0;
        let ecx = (lf_checker_rt::global::<u32>(QUERY_ECX) as *const u32).read_unaligned();
        let arg = (lf_checker_rt::global::<u32>(QUERY_ARG) as *const u32).read_unaligned();
        let p1: u32 = lf_checker_rt::callee_thiscall!(1, u32, ecx, arg);
        (p1.wrapping_add(STORE) as *mut u32).write_unaligned(tick);
        let ecx = (lf_checker_rt::global::<u32>(QUERY_ECX) as *const u32).read_unaligned();
        let arg = (lf_checker_rt::global::<u32>(QUERY_ARG) as *const u32).read_unaligned();
        let p2: u32 = lf_checker_rt::callee_thiscall!(1, u32, ecx, arg);
        if tick == 0xffff_ffff {
            return p2;
        }
        let m = (p2.wrapping_add(LOAD) as *const u32).read_unaligned();
        let d = tick.wrapping_sub(m);
        // Mirror the original: signed widen to double, add 2^32 if negative
        // (exact: the sum fits in 53 bits), round once to float, scale.
        let g = (d as i32) as f64 + if (d as i32) < 0 { TWO32 } else { 0.0 };
        let scaled = core::hint::black_box(g as f32) * core::hint::black_box(SCALE);
        let ecx = (lf_checker_rt::global::<u32>(QUERY_ECX) as *const u32).read_unaligned();
        let arg = (lf_checker_rt::global::<u32>(QUERY_ARG) as *const u32).read_unaligned();
        let p3: u32 = lf_checker_rt::callee_thiscall!(1, u32, ecx, arg);
        (p3.wrapping_add(FLOAT_OUT) as *mut u32).write_unaligned(scaled.to_bits());
        p3
    }
});
