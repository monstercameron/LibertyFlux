// original: 0x00891f40 aud_dispatch_stop_v1
/// Runs the stop-handler selected by the byte at 0x3b, then records state 3.
///
/// When the state word at 6 already reads 3 the function does nothing. When
/// the disable bit (bit 1 of the byte at 0x39) is set it only records state 3.
/// Otherwise it calls the handler from the dispatch table with this object;
/// the handler's answer is discarded.
export!(thiscall, rw_00891f40(this: *mut u8) -> () {
    unsafe {
        if *(this.add(6) as *const u16) == 3 {
            return;
        }
        if *(this.add(0x39)) & 2 == 0 {
            let idx = *(this.add(0x3b)) as u32;
            let base = relocated(0x115d5f4);
            let target = *((base.wrapping_add(idx.wrapping_mul(4))) as *const u32);
            let handler: extern "cdecl" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _ = handler(this as u32);
        }
        *(this.add(6) as *mut u16) = 3;
    }
});
