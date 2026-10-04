// original: 0x00891f00 aud_dispatch_stop_v2
/// Runs the stop-handler selected by the byte at 0x3b when the state word at
/// 6 reads 1 or 2 and the disable bit is clear, then records state 3 and
/// returns 3. The handler's answer is discarded.
export!(thiscall, rw_00891f00(this: *mut u8) -> u32 {
    unsafe {
        let state = *(this.add(6) as *const u16);
        if state == 1 || state == 2 {
            if *(this.add(0x39)) & 2 == 0 {
                let idx = *(this.add(0x3b)) as u32;
                let base = relocated(0x115d5f4);
                let target = *((base.wrapping_add(idx.wrapping_mul(4))) as *const u32);
                let handler: extern "cdecl" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _ = handler(this as u32);
            }
        }
        *(this.add(6) as *mut u16) = 3;
        3
    }
});
