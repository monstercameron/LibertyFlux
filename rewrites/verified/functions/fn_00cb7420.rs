// original: 0x00cb7420 CTaskComplexMoveWander::vf7
/// Fills `out` with the wander target point; returns `out`.
export!(thiscall, rw_cb7420(this_ptr: u32, out: u32) -> u32 {
    unsafe {
        let holder = ((this_ptr.wrapping_sub(12)) as *const u32).read();
        if holder != 0 {
            let v1 = (holder as *const u32).read();
            let s1 = ((v1.wrapping_add(0x30)) as *const u32).read();
            let target_of: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(s1 as usize);
            let mid = target_of(holder);
            let v2 = (mid as *const u32).read();
            let s2 = ((v2.wrapping_add(0x18)) as *const u32).read();
            let valid: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(s2 as usize);
            if valid(mid) as u8 != 0 {
                let holder2 = ((this_ptr.wrapping_sub(12)) as *const u32).read();
                let v1b = (holder2 as *const u32).read();
                let s1b = ((v1b.wrapping_add(0x30)) as *const u32).read();
                let target_of2: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(s1b as usize);
                let mid2 = target_of2(holder2);
                let v3 = (mid2 as *const u32).read();
                let s3 = ((v3.wrapping_add(0x1C)) as *const u32).read();
                let write_pt: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(s3 as usize);
                let _ = write_pt(mid2, out);
                return out;
            }
        }
        ((out) as *mut u32)
            .write(((this_ptr.wrapping_add(0x7C)) as *const u32).read());
        ((out.wrapping_add(4)) as *mut u32)
            .write(((this_ptr.wrapping_add(0x80)) as *const u32).read());
        ((out.wrapping_add(8)) as *mut u32)
            .write(((this_ptr.wrapping_add(0x84)) as *const u32).read());
        out
    }
});
