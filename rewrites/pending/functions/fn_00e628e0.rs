// original: 0x00e628e0 audio_poll6_stride18
/// Poll six audio slots in order, returning the last poll answer.
///
/// Walks six 0x18-byte records starting at the slot base, invoking the poll
/// routine (thiscall/0) on each record in turn.
export!(cdecl, rw_00e628e0() -> u32 {
    unsafe {
        const BASE: u32 = 0x0116_1850;
        const COUNT: u32 = 6;
        const STRIDE: u32 = 0x18;
        let poll: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut slot = relocated(BASE);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            last = poll(slot);
            slot = slot.wrapping_add(STRIDE);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
