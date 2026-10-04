// original: 0x00c683a0 CopModel_GetCurrent
// Resolve the current pair of values into two out-pointers. When the mode
// flag is set the pair comes from two word globals; otherwise a probe chain
// (two probes, a level check, two token checks) must all pass for the pair
// to come from two dword globals, and any failure takes the fallback exit.
export!(thiscall, rw_00c683a0(obj: u32, out0: *mut u32, out1: *mut u32) -> u32 {
    unsafe {
        let flag: u32 = callee_cdecl!(1, u32,);
        if flag & 0xff != 0 {
            *out1 = *(global::<u16>(0x169e0e4)) as u32;
            *out0 = *(global::<u16>(0x169d1a8)) as u32;
            return out0 as u32;
        }
        let t: u32 = callee_cdecl!(2, u32,);
        if t == 0 {
            return copmodel_fail(obj, out0, out1);
        }
        let u: u32 = callee_cdecl!(3, u32,);
        let r: u32 = callee_thiscall!(4, u32, u);
        if (r as i32) < 4 {
            return copmodel_fail(obj, out0, out1);
        }
        let token = *global::<u32>(0x12b4138);
        let cur = *global::<u32>(0x12fa494);
        if callee_cdecl!(5, u32, cur, token) & 0xff == 0 {
            return copmodel_fail(obj, out0, out1);
        }
        let alt = *global::<u32>(0x12fa650);
        if callee_cdecl!(6, u32, alt, token) & 0xff == 0 {
            return copmodel_fail(obj, out0, out1);
        }
        *out0 = alt;
        *out1 = cur;
        cur
    }
});

// Shared failure exit of rw_00c683a0: the fallback value goes through the
// second out-pointer, a default global through the first.
unsafe fn copmodel_fail(obj: u32, out0: *mut u32, out1: *mut u32) -> u32 {
    unsafe {
        let v: u32 = callee_thiscall!(7, u32, obj);
        *out1 = v;
        let d = *global::<u32>(0x12f9d44);
        *out0 = d;
        d
    }
}
