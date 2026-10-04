// original: 0x0099f700 live_handle_check
/// Check the audio object's live-handle flag against the current handle.
///
/// Returns 1 only when the object is non-null, its flag byte at +0x26C has
/// bit 2 set, its handle at +0xB30 is non-null, and the current-handle helper
/// (cdecl/1 over a null argument, stubbed by the checker) answers that same
/// handle; otherwise 0.
export!(stdcall, rw_0099f700(obj: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        if *((obj.wrapping_add(0x26C)) as *const u8) & 4 == 0 {
            return 0;
        }
        let h = *((obj.wrapping_add(0xB30)) as *const u32);
        if h == 0 {
            return 0;
        }
        if h == callee_cdecl!(1, u32, 0) {
            1
        } else {
            0
        }
    }
});
