// original: 0x00d72d70 replay_overlay_draw_gauge (proposed)

/// Draw one replay-overlay gauge: two derived level floats through the draw
/// pair, then the gauge frame.
///
/// `this` is the overlay object; `mode` selects the gauge (0 or 1; any other
/// value draws nothing and returns `mode` unchanged). Words at `+0x08` and
/// `+0x10` are the base level floats; a small positive bias constant read
/// from a global is added to the base for the first derived level, and the
/// sum of both bases minus the bias forms the second. The fill callee stages
/// two scratch words that are forwarded to the first draw call; the second
/// draw call takes the two derived levels. A two-byte global switch picks
/// argument 7 (only when the first byte is anything but `0x6a` and the
/// second is zero) or 2 for the style call, then come the mode call, the
/// width call (constant 0x3f8ccccd), the frame query (constant 0x3e for mode
/// 0, 0x3b for mode 1) and the frame commit of the queried word.
/// Returns the frame commit's answer on modes 0/1.
///
/// Original: 0x00d72d70 (thiscall, one stack word). Float order is the
/// original's: base+bias, then (top+base)-bias. The fill callee's two words
/// arrive in the original's frame, not the caller's: they are read back from
/// the rewrite's own cell after the stub writes them.
lf_checker_rt::export!(thiscall, rw_00d72d70(this: u32, mode: u32) -> u32 {
    unsafe {
        const BIAS_G: u32 = 0x00fe86ec;
        const STYLE_SW_G: u32 = 0x0116c250;
        const STYLE_ALT: u8 = 0x6a;
        const STYLE_DEFAULT: u32 = 2;
        const STYLE_ALT_ARG: u32 = 7;
        const BASE_OFF: u32 = 0x08;
        const TOP_OFF: u32 = 0x10;
        const WIDTH_BITS: u32 = 0x3f8ccccd;
        const FILL: u32 = 1;
        const DRAW_A: u32 = 2;
        const DRAW_B: u32 = 3;
        const STYLE: u32 = 4;
        const MODE_SET: u32 = 5;
        const WIDTH_SET: u32 = 6;
        const FRAME_QUERY: u32 = 7;
        const FRAME_COMMIT: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let base = f32::from_bits(rd32(this.wrapping_add(BASE_OFF)));
        let top = f32::from_bits(rd32(this.wrapping_add(TOP_OFF)));
        let bias = f32::from_bits(rd32(lf_checker_rt::global::<u32>(BIAS_G) as u32));
        let first = add(base, bias);
        let second = sub(add(top, base), bias);
        let (fill_arg, frame_const) = if mode == 0 {
            (8u32, 0x3eu32)
        } else if mode == 1 {
            (0x2cu32, 0x3bu32)
        } else {
            return mode;
        };

        let mut cell = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            FILL, u32,
            cell.as_mut_ptr() as u32, fill_arg
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(DRAW_A, u32, cell[0], cell[1]);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            DRAW_B, u32,
            first.to_bits(), second.to_bits()
        );
        let sw = lf_checker_rt::global::<u8>(STYLE_SW_G) as u32;
        let style = if rd8(sw) != STYLE_ALT && rd8(sw.wrapping_add(3)) == 0 {
            STYLE_ALT_ARG
        } else {
            STYLE_DEFAULT
        };
        let _: u32 = lf_checker_rt::callee_cdecl!(STYLE, u32, style);
        let _: u32 = lf_checker_rt::callee_cdecl!(MODE_SET, u32, 1);
        let _: u32 = lf_checker_rt::callee_cdecl!(WIDTH_SET, u32, WIDTH_BITS);
        let q: u32 = lf_checker_rt::callee_cdecl!(
            FRAME_QUERY, u32,
            &mode as *const u32 as u32, frame_const
        );
        lf_checker_rt::callee_cdecl!(FRAME_COMMIT, u32, rd32(q))
    }
});
