// original: 0x0087b940 rage::crmtNodeCapture::vf1
/// Tear down a capture node: release its child, run both teardown helpers,
/// zero the state words. Returns the last helper's answer.
export!(thiscall, rw_0087b940(this: u32) -> u32 {
    unsafe {
        if ((this + 0x24) as *const u8).read() != 0 {
            let child = ((this + 0x20) as *const u32).read();
            if child != 0 {
                let table = (child as *const u32).read();
                let target = (table as *const u32).read();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _: u32 = release(child, 1);
            }
        }
        ((this + 0x20) as *mut u32).write(0);
        let _: u32 = callee_thiscall!(2, u32, this);
        let answer: u32 = callee_thiscall!(3, u32, this);
        ((this + 0x0c) as *mut u32).write(0);
        ((this + 0x10) as *mut u32).write(0);
        if ((this + 0x08) as *const u32).read() != 0 {
            ((this + 0x08) as *mut u32).write(0);
        }
        answer
    }
});
