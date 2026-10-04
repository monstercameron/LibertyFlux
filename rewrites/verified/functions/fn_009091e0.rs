// original: 0x009091E0 ui_grid_visit (proposed)

/// Visit every cell of an N x N grid, calling one of two callees per cell.
///
/// Arguments (cdecl, two stack words): `a0` and `a1` are integer coordinates.
/// `N` comes from a global integer; a global flag byte selects the wide (3.0)
/// or narrow (1.0) match radius for the row coordinate, while the column
/// radius is always 1.0. For each cell (`edi`, `esi`) the function calls the
/// inside callee when both coordinates are within their radius of (`a0`,
/// `a1`) and the outside callee otherwise, passing (`edi`, `esi`) to either.
/// The comparisons are ordered single-precision float compares of the integer
/// coordinates converted to float; the radius arithmetic runs in the
/// original's order. The return value is `N`.
///
/// Original: 0x009091E0 (cdecl, two stack words; returns N in eax).
lf_checker_rt::export!(cdecl, rw_009091E0(a0: u32, a1: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x0116_09F6;
        const NN: u32 = 0x0103_44E4;
        const INSIDE: u32 = 1;
        const OUTSIDE: u32 = 2;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let flag = (lf_checker_rt::relocated(FLAG) as *const u8).read();
        let row_r = if flag != 0 { 3.0f32 } else { 1.0f32 };
        let col_r = 1.0f32;
        let n = (lf_checker_rt::relocated(NN) as *const i32).read();
        if n <= 0 {
            return n as u32;
        }
        let a0f = (a0 as i32) as f32;
        let a1f = (a1 as i32) as f32;
        let mut edi = 0i32;
        while edi < n {
            let edif = edi as f32;
            let mut esi = 0i32;
            while esi < n {
                let esif = esi as f32;
                let mut inside = !(sub(a0f, row_r) > edif);
                if inside {
                    inside = !(edif > add(a0f, row_r));
                }
                if inside {
                    inside = !(sub(a1f, col_r) > esif);
                }
                if inside {
                    inside = !(esif > add(a1f, col_r));
                }
                if inside {
                    lf_checker_rt::callee_cdecl!(INSIDE, u32, edi as u32, esi as u32);
                } else {
                    lf_checker_rt::callee_cdecl!(OUTSIDE, u32, edi as u32, esi as u32);
                }
                esi += 1;
            }
            edi += 1;
        }
        n as u32
    }
});
