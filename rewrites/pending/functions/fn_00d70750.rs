// original: 0x00D70750 replay_progress_draw (proposed)
/// Draw the replay progress bar for this recorder object.
///
/// Asks the mode query twice; each answer picks which of two stored extents
/// feeds the bar ratio (first answer the numerator, second the denominator).
/// After publishing the scaled ratio it resolves two HUD colours and draws
/// three markers: the base marker from the object's own position, a second
/// marker offset by the object's span, and a third marker that additionally
/// folds in the measure the middle call returned. Returns the finish call's
/// answer.
export!(thiscall, rw_d70750(this: u32) -> u32 {
    const NUM_WIDE: u32 = 0x0105_C87C;
    const NUM_NARROW: u32 = 0x0105_C880;
    const DEN_WIDE: u32 = 0x0105_C888;
    const DEN_NARROW: u32 = 0x0105_C884;
    /// Bar-scale constant (also the literal passed to the publish call).
    const BAR_SCALE: f32 = f32::from_bits(0x3EE6_6666);
    const HUD_ANCHOR: u32 = 0x00EE_AEEC;
    const POS_X: usize = 0x18;
    const POS_Y: usize = 0x1C;
    const SPAN: usize = 0x24;
    const BASE_ARG: u32 = 0x2C;
    const THIRD_ARG: u32 = 0x4C;
    const NO_OVERRIDE: u32 = 0xFFFF_FFFF;
    unsafe {
        let wide1 = callee_cdecl!(1, u32,);
        let num = if wide1 & 0xFF != 0 {
            global::<u32>(NUM_WIDE).read()
        } else {
            global::<u32>(NUM_NARROW).read()
        };
        let wide2 = callee_cdecl!(2, u32,);
        let den = if wide2 & 0xFF != 0 {
            global::<u32>(DEN_WIDE).read()
        } else {
            global::<u32>(DEN_NARROW).read()
        };
        let ratio = (num as i32) as f32 / (den as i32) as f32;
        callee_cdecl!(3, u32, 1);
        let scaled = ratio * BAR_SCALE;
        callee_cdecl!(4, u32, scaled.to_bits(), BAR_SCALE.to_bits());
        callee_cdecl!(5, u32, 0);
        callee_cdecl!(6, u32, 0);
        let mut colour1 = ratio.to_bits();
        let got1 = callee_cdecl!(7, u32, &mut colour1 as *mut u32 as u32, 0x3E);
        callee_cdecl!(8, u32, (got1 as *const u32).read());
        let y0 = (this as *const f32).byte_add(POS_Y).read();
        let x0 = (this as *const f32).byte_add(POS_X).read();
        callee_cdecl!(
            9, u32, x0.to_bits(), y0.to_bits(), this + BASE_ARG, NO_OVERRIDE, NO_OVERRIDE
        );
        let mut anchor_tmp = 0u32;
        callee_cdecl!(
            10, u32, relocated(HUD_ANCHOR), &mut anchor_tmp as *mut u32 as u32
        );
        let mut measure_tmp = 0u32;
        let measure = callee_cdecl!(11, f32, &mut measure_tmp as *mut u32 as u32, 0);
        let mut colour2 = measure.to_bits();
        let got2 = callee_cdecl!(15, u32, &mut colour2 as *mut u32 as u32, 0x3C);
        callee_cdecl!(8, u32, (got2 as *const u32).read());
        let span = (this as *const f32).byte_add(SPAN).read();
        let x1 = span + x0;
        let mut scratch = 0u32;
        callee_cdecl!(
            12, u32, x1.to_bits(), y0.to_bits(),
            &mut scratch as *mut u32 as u32, NO_OVERRIDE, NO_OVERRIDE
        );
        // The measure store lands on the first colour's slot, so the third
        // marker folds the measure (not the colour) into its position.
        let x2 = x1 + measure;
        callee_cdecl!(
            13, u32, x2.to_bits(), y0.to_bits(), this + THIRD_ARG, NO_OVERRIDE, NO_OVERRIDE
        );
        callee_cdecl!(14, u32,)
    }
});
