// original: 0x008d7fa0 init_grid_and_query
/// Initialise a 64-record grid and ask the batch helper about it.
///
/// Fills 64 records of 24 words from three constant floats (each record
/// takes the triple three times plus zero words and a 0xFFFF mark), then
/// calls the batch helper with the incoming words, the grid base twice
/// and an out-word starting at 0x40. Returns 1 when the helper leaves a
/// positive word behind, else 0.
export!(cdecl, rw_008d7fa0(arg0: u32, level: f32, arg2: u32) -> u32 {
    unsafe {
        let helper_answer = callee_cdecl!(1, u32,);
        let c0 = *(global::<u32>(0x01b4b328) as *const u32);
        let c1 = *(global::<u32>(0x01b4b324) as *const u32);
        let c2 = *(global::<u32>(0x01b4b320) as *const u32);
        let mut area = [0u32; 1546];
        let mut i = 0usize;
        while i < 64 {
            let r = 6 + i * 24;
            area[r] = 0;
            area[r + 4] = c2;
            area[r + 5] = c1;
            area[r + 6] = c0;
            area[r + 8] = c2;
            area[r + 9] = c1;
            area[r + 10] = c0;
            area[r + 12] = c2;
            area[r + 13] = c1;
            area[r + 14] = c0;
            area[r + 16] = 0;
            area[r + 17] = 0;
            area[r + 18] = 0;
            area[r + 19] = 0xffff;
            i += 1;
        }
        let mut out: u32 = 0x40;
        callee_cdecl!(
            2,
            u32,
            arg0,
            level.to_bits(),
            area.as_ptr().add(6) as u32,
            arg2,
            helper_answer,
            &mut out as *mut u32 as u32
        );
        if (out as i32) > 0 {
            1
        } else {
            0
        }
    }
});
