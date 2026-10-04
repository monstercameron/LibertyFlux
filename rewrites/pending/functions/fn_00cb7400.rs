// original: 0x00cb7400 CTaskComplexGoToPointAndStandStillTimed::vf7
/// Copies the 12-byte target point at +0xC to `dst`; returns `dst`.
#[allow(clippy::all)]
#[allow(non_snake_case)]
#[allow(unused_unsafe)]
export!(thiscall, rw_cb7400(this_ptr: u32, dst: u32) -> u32 {
    unsafe {
        let x = ((this_ptr.wrapping_add(0xC)) as *const u32).read();
        let y = ((this_ptr.wrapping_add(0x10)) as *const u32).read();
        let z = ((this_ptr.wrapping_add(0x14)) as *const u32).read();
        ((dst) as *mut u32).write(x);
        ((dst.wrapping_add(4)) as *mut u32).write(y);
        ((dst.wrapping_add(8)) as *mut u32).write(z);
        dst
    }
});
