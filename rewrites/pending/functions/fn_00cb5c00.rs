// original: 0x00cb5c00 record_kind_check
/// True when the object's record kind is one of the five known ids.
export!(cdecl, rw_cb5c00(arg0: u32) -> u8 {
    unsafe {
        let vptr = (arg0 as *const u32).read();
        let slot = ((vptr.wrapping_add(0xD0)) as *const u32).read();
        let resolve: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let id_obj = resolve(arg0, arg0);
        let rec: u32 = lf_checker_rt::callee_thiscall!(2, u32, id_obj, arg0);
        let code = if rec == 0 {
            -1i32
        } else {
            ((rec.wrapping_add(0x2E)) as *const i16).read() as i32
        };
        let g1 = (lf_checker_rt::global::<u32>(0x012FA278) as *const u32).read();
        let g2 = (lf_checker_rt::global::<u32>(0x012F9D68) as *const u32).read();
        let g3 = (lf_checker_rt::global::<u32>(0x012FA4A0) as *const u32).read();
        let g4 = (lf_checker_rt::global::<u32>(0x012F9EDC) as *const u32).read();
        let g5 = (lf_checker_rt::global::<u32>(0x012FA0BC) as *const u32).read();
        if code as u32 == g1
            || code as u32 == g2
            || code as u32 == g3
            || code as u32 == g4
            || code as u32 == g5
        {
            1
        } else {
            0
        }
    }
});
