// original: 0x00bf5600 build_emitter_tinted
/// Build the emitter, run the shared setup helpers, then convert the indexed
/// table colour to three scaled floats. Null object: channel none.
export!(thiscall, rw_bf5600(this: *const u8, obj: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        let h: u32 = callee_cdecl!(1, u32, relocated(0xEBBD40), 0, 0, 0);
        let esi: u32 = callee_thiscall!(2, u32, relocated(0x1394D60), h, 0, 0);
        if esi == 0 {
            return 0;
        }
        let mut b1 = [0u32; 8];
        let mut b2 = [0u32; 8];
        let _: u32 = callee_thiscall!(3, u32, this as u32, b1.as_mut_ptr() as u32);
        let _: u32 = callee_thiscall!(4, u32, this as u32, b2.as_mut_ptr() as u32);
        let mut c1 = [0u32; 4];
        let mut c2 = [0u32; 4];
        let mut c3 = [0u32; 4];
        let _: u32 = callee_cdecl!(
            5, u32, c3.as_mut_ptr() as u32, c2.as_mut_ptr() as u32,
            c1.as_mut_ptr() as u32, 1
        );
        let mut d = [0u32; 8];
        let _: u32 = callee_thiscall!(6, u32, esi, d.as_mut_ptr() as u32);
        let idx = *((obj as *const u8).add(0xF94));
        let col = *global::<u32>(0x12FAE88).add(idx as usize);
        let scale = f32::from_bits(*global::<u32>(0xFE86E8));
        let r = ((col >> 16) & 0xFF) as f32 * scale;
        *((esi as *mut u8).add(0x180) as *mut u32) = r.to_bits();
        let g = ((col >> 8) & 0xFF) as f32 * scale;
        *((esi as *mut u8).add(0x184) as *mut u32) = g.to_bits();
        let b = (col & 0xFF) as f32 * scale;
        *((esi as *mut u8).add(0x188) as *mut u32) = b.to_bits();
        callee_thiscall!(7, u32, esi)
    }
});
