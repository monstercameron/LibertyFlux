// original: 0x00ad2690 audio_param_apply
/// Apply one audio parameter set selected by `sel`.
///
/// Sends the selector-dependent command pairs, then (unless a global
/// bypass byte is clear) a fixed bank of level commands; scales four
/// caller-supplied coefficients by two global level values chosen
/// through two probe calls, forwards the four rounded products, and
/// finishes with a sign-dependent command. Returns the session
/// handle answer from the closing session call.
export!(cdecl, rw_ad2690(sel: u32, coeff: *const f32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32,);
        let mgr = *global::<u32>(0x17f583c);
        callee_thiscall!(2, u32, mgr, relocated(0x1110090));
        match sel {
            0 => {
                callee_cdecl!(3, u32, 8, 0);
                callee_cdecl!(3, u32, 7, 0);
            }
            1 => {
                callee_cdecl!(3, u32, 2, 1);
                callee_cdecl!(3, u32, 8, 1);
                callee_cdecl!(3, u32, 7, 0);
            }
            2 => {
                callee_cdecl!(3, u32, 2, 0);
                callee_cdecl!(3, u32, 8, 1);
                callee_cdecl!(3, u32, 7, 0);
            }
            _ => {}
        }
        if *global::<u8>(0x154e161) == 0 {
            callee_cdecl!(3, u32, 0x13, 0);
        } else {
            callee_cdecl!(3, u32, 0x13, 1);
            callee_cdecl!(3, u32, 0x19, 0xff);
            callee_cdecl!(3, u32, 0x1a, 0xff);
            callee_cdecl!(3, u32, 0x17, 4);
            callee_cdecl!(3, u32, 0x14, 0);
            callee_cdecl!(3, u32, 0x15, 0);
            callee_cdecl!(3, u32, 0x16, 0);
        }
        let probe_a = callee_cdecl!(4, u32,);
        let mut level_a = *global::<i32>(0x105c884);
        if (probe_a & 0xff) != 0 {
            level_a = *global::<i32>(0x105c888);
        }
        let probe_b = callee_cdecl!(5, u32,);
        let mut level_b = *global::<i32>(0x105c880);
        if (probe_b & 0xff) != 0 {
            level_b = *global::<i32>(0x105c87c);
        }
        let fa = level_a as f32;
        let fb = level_b as f32;
        let c0 = *coeff.offset(0);
        let c1 = *coeff.offset(1);
        let c2 = *coeff.offset(2);
        let c3 = *coeff.offset(3);
        let out_a = cvtt_f32_to_i32(fa * c0);
        let t = cvtt_f32_to_i32(c3 * fb);
        let out_b = level_b.wrapping_sub(t);
        let t = cvtt_f32_to_i32(c1 * fb);
        let out_d = level_b.wrapping_sub(t).wrapping_sub(out_b);
        let t = cvtt_f32_to_i32(c2 * fa);
        let out_c = t.wrapping_sub(out_a);
        callee_cdecl!(6, u32, out_a as u32, out_b as u32, out_c as u32, out_d as u32);
        let lim = *global::<i32>(0x106b310);
        let neg = if lim < 0 { lim as u32 } else { 0 };
        callee_cdecl!(3, u32, 1, neg);
        let mgr = *global::<u32>(0x17f583c);
        callee_thiscall!(2, u32, mgr, relocated(0x1110090))
    }
});
