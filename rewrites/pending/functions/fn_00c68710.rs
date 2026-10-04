// original: 0x00c68710 resolve_current_value
// Resolve the current value through a probe chain: the mode flag selects a
// word global; otherwise two probe/level gates (levels 3 and 5) and two
// token checks select among three dword globals, defaulting to the last.
export!(cdecl, rw_00c68710() -> u32 {
    unsafe {
        let flag: u32 = callee_cdecl!(1, u32,);
        if flag & 0xff != 0 {
            return *(global::<u16>(0x169e0e4)) as u32;
        }
        let a: u32 = callee_cdecl!(2, u32,);
        if a == 0 {
            return *global::<u32>(0x12fa3f8);
        }
        let b: u32 = callee_cdecl!(3, u32,);
        let r1: u32 = callee_thiscall!(4, u32, b);
        if (r1 as i32) < 3 {
            return *global::<u32>(0x12fa3f8);
        }
        let c: u32 = callee_cdecl!(5, u32,);
        let r2: u32 = callee_thiscall!(6, u32, c);
        if (r2 as i32) >= 5 {
            return *global::<u32>(0x12fa3f8);
        }
        let token = *global::<u32>(0x12b4138);
        let g068 = *global::<u32>(0x12fa068);
        if callee_cdecl!(7, u32, g068, token) & 0xff != 0 {
            return g068;
        }
        let g044 = *global::<u32>(0x12fa044);
        if callee_cdecl!(8, u32, g044, token) & 0xff != 0 {
            return g044;
        }
        *global::<u32>(0x12fa3f8)
    }
});
