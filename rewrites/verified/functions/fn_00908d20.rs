// original: 0x00908d20 radar_config_store_rescale
/// Store a radar-config block and rescale one entry.
///
/// Copies two scalars and three 2-word vectors into scalar globals, clears a
/// done byte, and (unless the bypass byte is set) runs an engine combine
/// step, picks two dimension values by two engine answers, and nudges the
/// stored vector when 1.0 exceeds their float ratio. Returns the second
/// engine answer on the full path, else the third vector's second word.
export!(cdecl, rw_00908D20(a: u32, b: u32, ptr1: u32, ptr2: u32, ptr3: u32) -> u32 {
    unsafe {
        *global::<u32>(0x10344B0) = a;
        *global::<u32>(0x10344B4) = b;
        let p1 = ptr1 as *const u32;
        *global::<u32>(0x10344B8) = *p1;
        *global::<u32>(0x10344BC) = *p1.add(1);
        let p2 = ptr2 as *const u32;
        let p3 = ptr3 as *const u32;
        *global::<u32>(0x10344C0) = *p2;
        *global::<u32>(0x10344C4) = *p2.add(1);
        *global::<u32>(0x10344C8) = *p3;
        let p3b = *p3.add(1);
        *global::<u32>(0x10344CC) = p3b;
        *global::<u8>(0x10344D9) = 0;
        if *global::<u8>(0x11609F6) == 0 {
            callee_cdecl!(
                1,
                u32,
                2,
                lf_k2_rt::relocated(0x10344B8),
                lf_k2_rt::relocated(0x10344C0),
                0
            );
            let s0: u32 = callee_cdecl!(2, u32,);
            let esi = if s0 & 0xFF != 0 {
                *global::<u32>(0x105C888)
            } else {
                *global::<u32>(0x105C884)
            };
            let s1: u32 = callee_cdecl!(2, u32,);
            let ecx = if s1 & 0xFF != 0 {
                *global::<u32>(0x105C87C)
            } else {
                *global::<u32>(0x105C880)
            };
            let ratio = (esi as i32 as f32) / (ecx as i32 as f32);
            if 1.0f32 > ratio {
                let adj = f32::from_bits(*p2.add(1))
                    - f32::from_bits(*global::<u32>(0x10344C4))
                    + f32::from_bits(*global::<u32>(0x10344BC));
                *global::<f32>(0x10344BC) = adj;
            }
            return s1;
        }
        p3b
    }
});
