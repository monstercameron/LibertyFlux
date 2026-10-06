// original: 0x00963250 tick_delta_scale_store_3c_40_44
/// Scale a tick delta by 0.001 and store it as float (offsets 0x3C/0x40/0x44).
///
/// Takes one `tick` word. Fetches the stats object three times through the
/// query helper (thiscall/1, ECX and the stack word both from globals),
/// stores `tick` at `+0x3C`, loads the reference at `+0x40`, and stores
/// `(delta as u32 as f32) * 0.001` at `+0x44`, where delta is reference minus tick;
/// when the loaded value is `-1` it stores and returns that value early. Returns the last object pointer.
/// The float conversion is exact: the original widens the signed delta to
/// double and adds 2^32 when negative, which equals the wrapping (unsigned)
/// delta exactly; one rounding to float follows, then one multiply. The
/// rewrite converts the wrapping delta directly, with the multiply's operand
/// order pinned. The original reuses its incoming argument slot as the float
/// temporary, so the stack comparison is off; the value is observed in the
/// `+0x44` store instead.
lf_checker_rt::export!(cdecl, rw_00963250(tick: u32) -> u32 {
    unsafe {
        const QUERY_ECX: u32 = 0x11f6954;
        const QUERY_ARG: u32 = 0x11f6f34;
        const STORE: u32 = 0x3c;
        const LOAD: u32 = 0x40;
        const FLOAT_OUT: u32 = 0x44;
        const SCALE: f32 = f32::from_bits(0x3a83126f); // 0.001
        const TWO32: f64 = 4294967296.0;
        let ecx = (lf_checker_rt::global::<u32>(QUERY_ECX) as *const u32).read_unaligned();
        let arg = (lf_checker_rt::global::<u32>(QUERY_ARG) as *const u32).read_unaligned();
        let p1: u32 = lf_checker_rt::callee_thiscall!(1, u32, ecx, arg);
        (p1.wrapping_add(STORE) as *mut u32).write_unaligned(tick);
        let ecx = (lf_checker_rt::global::<u32>(QUERY_ECX) as *const u32).read_unaligned();
        let arg = (lf_checker_rt::global::<u32>(QUERY_ARG) as *const u32).read_unaligned();
        let p2: u32 = lf_checker_rt::callee_thiscall!(1, u32, ecx, arg);
        let m = (p2.wrapping_add(LOAD) as *const u32).read_unaligned();
        if m == 0xffff_ffff {
            return m;
        }
        let d = m.wrapping_sub(tick);
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
