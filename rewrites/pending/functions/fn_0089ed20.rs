// original: 0x0089ED20 rage::audStreamingSound::~audStreamingSound__deleting
// ---------------------------------------------------------------------------
// 0x0089ED20 rage::audStreamingSound deleting destructor: same shape as the
// sequential one above, running the streaming-sound teardown first.
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089ED20(this_ptr: u32, flags: u32) -> u32 {
    callee_thiscall!(1, u32, this_ptr);
    if flags & 1 != 0 && this_ptr != 0 {
        let bank = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
        callee_thiscall!(2, u32, relocated(0x115D8A0), this_ptr, bank);
    }
    this_ptr
});
