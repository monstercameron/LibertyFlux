// original: 0x008B8C60 fe_scaled_action_a

/// Resolve a scaled frontend action and store its 8-byte descriptor at `out`.
///
/// Multiplies two magnitude factors (either an unsigned pair read from the
/// pad-state table, or a signed pair picked from globals by two device
/// queries), thresholds the product to remember whether it is large, then
/// maps an axis value supplied by the axis query through five thresholds to
/// one of six action ids. The selected id and the axis value go to the
/// action resolver; the 8 bytes it returns are copied to `out`, which is
/// also the return value. NaN axis values fall through to the default id,
/// exactly like values below the lowest threshold.
export!(cdecl, rw_008B8C60(out: *mut u8) -> u32 {
    unsafe {
        const PROD_LIMIT: f32 = f32::from_bits(0x4961_0000); // 921600.0
        const T_HI: f32 = f32::from_bits(0x3FDC_71A2);
        const T_2: f32 = f32::from_bits(0x3FD1_1229);
        const T_3: f32 = f32::from_bits(0x3FBB_BBBF);
        const T_4: f32 = f32::from_bits(0x3FA5_5ACB);
        const T_LO: f32 = f32::from_bits(0x3F80_0000); // 1.0
        const ID_DEFAULT: u32 = 0x34;
        const ID_2: u32 = 0xF3;
        const ID_3: u32 = 0xF4;
        const ID_BIG: u32 = 0xF1;
        const ID_4: u32 = 0xF2;
        let alt_source = *global::<u8>(0x017A_CCF8) != 0;
        let (lo, hi) = if !alt_source {
            let n = callee_cdecl!(1, u32,);
            let base = *global::<u32>(0x0116_8BB0) as *const u8;
            let row = base.add(n.wrapping_sub(1).wrapping_mul(16) as usize) as *const u32;
            ((*row) as f32, (*row.add(1)) as f32)
        } else {
            let a = callee_cdecl!(2, u32,);
            let s = if a & 0xFF != 0 {
                *global::<u32>(0x0105_C888)
            } else {
                *global::<u32>(0x0105_C884)
            };
            let b = callee_cdecl!(2, u32,);
            let c = if b & 0xFF != 0 {
                *global::<u32>(0x0105_C87C)
            } else {
                *global::<u32>(0x0105_C880)
            };
            ((s as i32) as f32, (c as i32) as f32)
        };
        let large = hi * lo > PROD_LIMIT;
        let v: f32 = callee_thiscall!(3, f32, 0x0118_D7F0, 1);
        let mut slot = v;
        // Ordered `>` comparisons reproduce the comiss ja/jbe chain,
        // including NaN falling through every band to the default id.
        let id = if v > T_HI {
            ID_DEFAULT
        } else if v > T_2 {
            ID_2
        } else if v > T_3 {
            ID_3
        } else if v > T_4 {
            if large { ID_BIG } else { ID_DEFAULT }
        } else if v > T_LO {
            ID_4
        } else {
            ID_DEFAULT
        };
        let descriptor = callee_cdecl!(4, u32, &mut slot as *mut f32 as u32, id);
        core::ptr::copy_nonoverlapping(descriptor as *const u8, out, 8);
        out as u32
    }
});
