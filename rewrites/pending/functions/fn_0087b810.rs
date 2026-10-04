// original: 0x0087b810 capture_node_teardown_tail (proposed)
/// Tear down a capture node, then tail-forward to the base teardown.
///
/// Re-attaches the capture table, releases the child if the flag byte and
/// link say one exists, runs the two teardown helpers, zeroes the state
/// words and forwards to the base teardown, whose answer is returned.
export!(thiscall, rw_0087b810(this: u32) -> u32 {
    /// Capture-node behaviour table (file VA).
    const CAPTURE_TABLE: u32 = 0x00FE8410;
    unsafe {
        (this as *mut u32).write(relocated(CAPTURE_TABLE));
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
        let _: u32 = callee_thiscall!(3, u32, this);
        ((this + 0x0c) as *mut u32).write(0);
        ((this + 0x10) as *mut u32).write(0);
        if ((this + 0x08) as *const u32).read() != 0 {
            ((this + 0x08) as *mut u32).write(0);
        }
        callee_thiscall!(4, u32, this)
    }
});
