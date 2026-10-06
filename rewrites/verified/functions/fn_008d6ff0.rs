// original: 0x008d6ff0 init_record_array_and_call
/// Initialise a 64-record array and run the batch helper over it.
///
/// Normalises a negative limit word through a small helper, then fills
/// 64 records of 24 words from three constant floats (each record takes
/// the triple three times plus zero words and a 0xFFFF mark), and calls
/// the batch helper with the four incoming words, the array base and the
/// constants 0x40 and 4. Returns 1 when the helper answers zero.
export!(cdecl, rw_008d6ff0(arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        let limit = if (arg3 as i32) < 0 {
            callee_cdecl!(1, u32, arg3)
        } else {
            arg3
        };
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
        let answer = callee_cdecl!(
            2,
            u32,
            arg0,
            arg1,
            arg2,
            area.as_ptr().add(6) as u32,
            limit,
            0x40,
            4
        );
        if answer == 0 {
            1
        } else {
            0
        }
    }
});
