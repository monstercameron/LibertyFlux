// original: 0x008d77d0 integrate_scaled_rates
// Zeroes three control words, then scales three rate readings by a constant
// and stores each product against its matching gain word. The last slot
// copies an uninitialized frame word, which is 0 under the checker's
// defined zero fill. Returns the last rate reading.
export!(thiscall, rw_008d77d0(obj: *mut u8) -> u32 {
    unsafe {
        *(obj.wrapping_add(0x48) as *mut u32) = 0;
        *(obj.wrapping_add(0x44) as *mut u32) = 0;
        *(obj.wrapping_add(0x40) as *mut u32) = 0;
        let k = *global::<f32>(0x00FE_8684);
        let t68 = *(obj.wrapping_add(0x68) as *const f32);
        let a1: u32 = callee_cdecl!(1, u32,);
        let f1 = (a1 as i32) as f32 * k;
        let t64 = *(obj.wrapping_add(0x64) as *const f32);
        let a2: u32 = callee_cdecl!(1, u32,);
        let f2 = (a2 as i32) as f32 * k;
        let t60 = *(obj.wrapping_add(0x60) as *const f32);
        let a3: u32 = callee_cdecl!(1, u32,);
        let f3 = (a3 as i32) as f32 * k;
        *(obj.wrapping_add(0x74) as *mut f32) = f2 * t64;
        *(obj.wrapping_add(0x78) as *mut f32) = f1 * t68;
        *(obj.wrapping_add(0x70) as *mut f32) = f3 * t60;
        *(obj.wrapping_add(0x7C) as *mut u32) = 0;
        a3
    }
});
