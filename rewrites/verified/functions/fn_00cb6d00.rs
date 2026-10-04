// original: 0x00cb6d00 CTaskComplexWanderStandard::vf6
/// Wander urgency (-1.0 when blocked or stalled, else 15.0).
export!(thiscall, rw_cb6d00(this_ptr: u32, arg0: u32) -> f32 {
    unsafe {
        let sub = ((this_ptr.wrapping_add(8)) as *const u32).read();
        if sub != 0 {
            let v1 = (sub as *const u32).read();
            let s1 = ((v1.wrapping_add(0xC)) as *const u32).read();
            let k1: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(s1 as usize);
            if k1(sub) == 0x11D {
                let mid: u32 = lf_checker_rt::callee_thiscall!(2, u32, sub, arg0);
                if mid != 0 {
                    let v3 = (mid as *const u32).read();
                    let s3 = ((v3.wrapping_add(0xC)) as *const u32).read();
                    let k3: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(s3 as usize);
                    if k3(mid) == 0x3B8
                        && ((mid.wrapping_add(0x74)) as *const u8).read() & 1 != 0
                    {
                        return -1.0;
                    }
                }
                let subb = ((this_ptr.wrapping_add(8)) as *const u32).read();
                let dep = ((subb.wrapping_add(0x14)) as *const u32).read();
                if dep != 0 {
                    let v4 = (dep as *const u32).read();
                    let s4 = ((v4.wrapping_add(0xC)) as *const u32).read();
                    let k4: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(s4 as usize);
                    if k4(dep) == 0x3AE {
                        return -1.0;
                    }
                }
            }
        }
        15.0
    }
});
