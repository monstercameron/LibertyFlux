// original: 0x00d6f030 replay_bar_time_factor
/// Combine the bar clock reading with the selected time bases.
///
/// Returns `(17 - ticks + base1 * w) / base2` in single precision, where
/// `ticks` comes from the clock object's virtual slot +0x24 (callee 2),
/// `w` is the float at +0x0C, and each base is picked by one sample of
/// callee 1 between the globals at 0x105C880/0x105C87C. Returns +0.0 when
/// the clock link at +0x94 is null.
lf_checker_rt::export!(thiscall, rw_00d6f030(this_ptr: u32) -> f64 {
    unsafe {
        use core::arch::x86::{
            _mm_add_ss, _mm_cvtss_f32, _mm_div_ss, _mm_mul_ss, _mm_set_ss, _mm_sub_ss,
        };
        let b = this_ptr as *const u8;
        let link = *((b.add(0x94)) as *const u32);
        if link == 0 {
            return 0.0;
        }
        let obj = *(link as *const u32);
        let a1: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let mut base1 = *lf_checker_rt::global::<u32>(0x105c880);
        if (a1 & 0xFF) != 0 {
            base1 = *lf_checker_rt::global::<u32>(0x105c87c);
        }
        let vtab = *(obj as *const u32);
        let slot = *(((vtab as *const u8).add(0x24)) as *const u32);
        let tick: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let ticks = tick(obj);
        let w = f32::from_bits(*((b.add(0x0c)) as *const u32));
        let mut x = _mm_cvtss_f32(_mm_sub_ss(
            _mm_set_ss(17.0),
            _mm_set_ss((ticks as i32) as f32),
        ));
        let adj = _mm_cvtss_f32(_mm_mul_ss(
            _mm_set_ss((base1 as i32) as f32),
            _mm_set_ss(w),
        ));
        x = _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(x), _mm_set_ss(adj)));
        let a2: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let mut base2 = *lf_checker_rt::global::<u32>(0x105c880);
        if (a2 & 0xFF) != 0 {
            base2 = *lf_checker_rt::global::<u32>(0x105c87c);
        }
        x = _mm_cvtss_f32(_mm_div_ss(
            _mm_set_ss(x),
            _mm_set_ss((base2 as i32) as f32),
        ));
        x as f64
    }
});
