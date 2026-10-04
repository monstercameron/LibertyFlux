// original: 0x008B8DA0 fe_scaled_action_b

/// Same as `rw_008B8C60` with the second action-id set
/// (default 0x2E; bands 0xEF, 0xF0, 0xED-if-large, 0xEE).
export!(cdecl, rw_008B8DA0(out: *mut u8) -> u32 {
    unsafe {
        const PROD_LIMIT: f32 = f32::from_bits(0x4961_0000);
        const T_HI: f32 = f32::from_bits(0x3FDC_71A2);
        const T_2: f32 = f32::from_bits(0x3FD1_1229);
        const T_3: f32 = f32::from_bits(0x3FBB_BBBF);
        const T_4: f32 = f32::from_bits(0x3FA5_5ACB);
        const T_LO: f32 = f32::from_bits(0x3F80_0000);
        const ID_DEFAULT: u32 = 0x2E;
        const ID_2: u32 = 0xEF;
        const ID_3: u32 = 0xF0;
        const ID_BIG: u32 = 0xED;
        const ID_4: u32 = 0xEE;
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
