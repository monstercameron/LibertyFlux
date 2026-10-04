// original: 0x00905c60 Checkpoint_ScaleIntColors
// original: 0x00905c60 Checkpoint_ScaleIntColors
/// Scale integer checkpoint colours and forward them (`Checkpoint_ScaleIntColors`).
///
/// Converts the last four unsigned arguments to float through a double
/// helper table (adding 2^32 for values with the top bit set), scales them,
/// reads three selector floats through the second argument, and passes the
/// ten words (selectors, raw third and first arguments, scaled floats, raw
/// eighth argument) to the draw helper (cdecl/10, stubbed). Returns the
/// helper's answer.
///
/// a-F05 fix: the r-s66 rewrite converted the wrong arguments (first, second,
/// third, seventh) and dereferenced the eighth as the float source; the
/// original converts the fourth through seventh and dereferences the second.
export!(cdecl, rw_00905c60(a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32 {
    unsafe {
        let scale = *global::<f32>(0xfe86e8);
        let t0 = *global::<f64>(0xfe8f50);
        let t1 = *global::<f64>(0xfe8f58);
        let cvt = |x: u32| -> f32 {
            let adj = if x >> 31 == 0 { t0 } else { t1 };
            (((x as i32) as f64) + adj) as f32 * scale
        };
        let f4 = cvt(a4);
        let f5 = cvt(a5);
        let f6 = cvt(a6);
        let f7 = cvt(a7);
        let m0 = (a2 as *const f32).read();
        let m4 = (a2 as *const f32).add(1).read();
        let m8 = (a2 as *const f32).add(2).read();
        callee_cdecl!(1, u32, m0.to_bits(), m4.to_bits(), m8.to_bits(), a3, a1,
                      f4.to_bits(), f5.to_bits(), f6.to_bits(), f7.to_bits(), a8)
    }
});
