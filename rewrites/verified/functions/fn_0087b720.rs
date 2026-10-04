// original: 0x0087b720 rage::crmtNodeFrame::vf1
/// Tear down a frame node: release its child, clear the link, run the
/// teardown helper and zero the state words.
///
/// If the "has child" flag byte is set and a child link is present, the
/// child is released through its own release entry first. Returns the
/// helper's answer.
export!(thiscall, rw_0087b720(this: u32) -> u32 {
    unsafe {
        if ((this + 0x20) as *const u8).read() != 0 {
            let child = ((this + 0x1c) as *const u32).read();
            if child != 0 {
                let table = (child as *const u32).read();
                let target = (table as *const u32).read();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _: u32 = release(child, 1);
            }
        }
        ((this + 0x1c) as *mut u32).write(0);
        let answer: u32 = callee_thiscall!(2, u32, this);
        ((this + 0x0c) as *mut u32).write(0);
        ((this + 0x10) as *mut u32).write(0);
        if ((this + 0x08) as *const u32).read() != 0 {
            ((this + 0x08) as *mut u32).write(0);
        }
        answer
    }
});
