// original: 0x00660c10 rage::snEstablishSessionTask::vf2
/// Begin establishing a session: reset the attempt counter, mark started.
///
/// Writes 0 to the attempt word at `+0x90`, and, when the started flag at
/// `+0xc` is still 0, sets it to 1. Returns nothing meaningful.
export!(thiscall, rw_00660c10(this: u32) -> u32 {
    unsafe {
        ((this + 0x90) as *mut u32).write(0);
        if ((this + 0xc) as *const u32).read() == 0 {
            ((this + 0xc) as *mut u32).write(1);
        }
        0
    }
});

