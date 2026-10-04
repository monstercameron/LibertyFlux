// original: 0x00cc6000 queue_drain_and_free
/// Pop every node off the queue at `this` and release each through the
/// shared release helper. No meaningful return value.
export!(thiscall, rw_00cc6000(this: *mut u8) -> u32 {
    unsafe {
        while *(this as *const u32) != 0 {
            let node: u32 = callee_thiscall!(1, u32, this as u32);
            let _: u32 = callee_cdecl!(2, u32, node);
        }
        0
    }
});
