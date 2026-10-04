// original: 0x00a87b20 CRenderPhase::vf10
/// Pick render mode 1 or 2 from a flag bit and a mode byte.
///
/// Reads bit 9 of the dword at `this + 0x8E8` and the byte at `this + 0x1D`.
/// When the mode byte is nonzero the result is the flag bit plus one,
/// otherwise it is two minus the flag bit, so the return is always 1 or 2.
export!(thiscall, rw_00a87b20(this_obj: u32) -> u32 {
    unsafe {
        let flags = *((this_obj.wrapping_add(0x8e8)) as *const u32);
        let bit = (flags >> 9) & 1;
        let mode = *((this_obj.wrapping_add(0x1d)) as *const u8);
        if mode != 0 { bit + 1 } else { 2 - bit }
    }
});
