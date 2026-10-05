// original: 0x00b1b630 rebuild_palette_gated (proposed)

/// Rebuilds the palette entry when the gate flag is set.
///
/// Thiscall with no stack arguments. With the gate byte clear it returns
/// at once (returning the caller's EAX, so that path is excluded from
/// the proof). Otherwise it runs the key probe (thiscall, no stack
/// arguments), reads the variant bit (cdecl of 0) to pick tag 0x41 or
/// 0x3B for the swatch fetch (cdecl of a scratch buffer and the tag;
/// word 0 of the returned block), truncates the fetched level (cdecl of
/// a scratch buffer and 0x37; word 0 of the returned block) to an
/// integer, optionally pokes the live flag (thiscall on the flag word,
/// no stack arguments), clamps the level byte against a second fetch
/// (nonnegative levels cap at the fetch, negative levels and NaN yield
/// 0 and the fetch respectively), and submits the colour with the level
/// in the top byte (cdecl of one word) before running two local passes
/// (thiscall, no stack arguments) and tail-calling the notifier.
/// Returns the notifier's result.
lf_checker_rt::export!(thiscall, rw_00b1b630(this: u32) -> u32 {
    unsafe {
        const GATE: u32 = 0x011609f6;
        const LIVE_FLAG: u32 = 0x01161548;
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            // Exact cvttss2si: out of range and NaN yield 0x80000000,
            // where Rust would saturate.
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        if (lf_checker_rt::global::<u8>(GATE) as *const u8).read() == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(1, u32, this);
        let variant: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0);
        let tag = if (variant as u8) != 0 { 0x41u32 } else { 0x3bu32 };
        let mut scratch = [0u32; 4];
        let swatch: u32 = lf_checker_rt::callee_cdecl!(3, u32, scratch.as_mut_ptr() as u32, tag);
        let rgb = (swatch as *const u32).read_unaligned();
        let level_block: u32 =
            lf_checker_rt::callee_cdecl!(4, u32, scratch.as_mut_ptr() as u32, 0x37);
        let level = (level_block as *const f32).read_unaligned();
        if (lf_checker_rt::global::<u8>(LIVE_FLAG) as *const u8).read() != 0 {
            lf_checker_rt::callee_thiscall!(5, u32, lf_checker_rt::relocated(LIVE_FLAG));
        }
        let level_f = cvtt(level) as u8 as f32;
        let level2_block: u32 =
            lf_checker_rt::callee_cdecl!(4, u32, scratch.as_mut_ptr() as u32, 0x37);
        let cap = (level2_block as *const f32).read_unaligned();
        let clamped = if 0.0f32 > level_f {
            0.0f32
        } else if !(level_f > cap) {
            level_f
        } else {
            cap
        };
        let byte = cvtt(clamped) as u8 as u32;
        let arg = (rgb & 0xffffff) | (byte << 24);
        lf_checker_rt::callee_cdecl!(6, u32, arg);
        lf_checker_rt::callee_thiscall!(7, u32, this);
        lf_checker_rt::callee_thiscall!(8, u32, this);
        lf_checker_rt::callee_cdecl!(9, u32,)
    }
});
