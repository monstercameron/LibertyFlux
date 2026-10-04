// original: 0x00981940 audio_bind_voice_table
/// Original 0x00981940 (unnamed): bind and fill the voice table.
///
/// Normalises the source record, allocates (or reuses) the `count`-entry
/// pointer array, clamps the limit byte to 0x10, then resolves each key
/// through the global audio manager, storing live handles and flagging them;
/// a null answer shrinks the count and retries the slot. Returns the source
/// pointer, or the clamped limit when the count is zero.
export!(thiscall, rw_00981940(this_: u32, arg: u32) -> u32 {
    callee_thiscall!(1, u32, this_, arg);
    let count = unsafe { ((arg + 0x23) as *const u8).read() } as u32;
    unsafe {
        ((this_ + 0x30) as *mut u32).write(count);
        let tag = ((arg + 0x1e) as *const u32).read_unaligned();
        ((this_ + 0x34) as *mut u32).write(tag);
    }
    let init = unsafe { ((this_ + 0x46) as *const u16).read() };
    if init == 0 {
        unsafe { ((this_ + 0x46) as *mut u16).write(count as u16); }
        let arr = if count == 0 {
            0
        } else {
            callee_cdecl!(2, u32, count.wrapping_mul(4))
        };
        unsafe { ((this_ + 0x40) as *mut u32).write(arr); }
    }
    unsafe { ((this_ + 0x44) as *mut u16).write(count as u16); }
    let lim = unsafe { ((arg + 0x22) as *const u8).read() } as u32;
    let clamped = if lim > 0x10 { 0x10 } else { lim };
    unsafe { ((this_ + 0x38) as *mut u32).write(clamped); }
    if count == 0 {
        unsafe { (this_ as *mut u32).write(arg); }
        return clamped;
    }
    let mut n = count;
    let mut i = 0u32;
    let mut slot = arg.wrapping_add(0x24);
    while i < n {
        let key = unsafe { (slot as *const u32).read() };
        let arr = unsafe { ((this_ + 0x40) as *const u32).read() };
        let ent = callee_thiscall!(3, u32, relocated(0x0115D9A0), key);
        unsafe {
            (arr.wrapping_add(i.wrapping_mul(4)) as *mut u32).write(ent);
            let back = (((this_ + 0x40) as *const u32).read()
                .wrapping_add(i.wrapping_mul(4)) as *const u32)
                .read();
            if back == 0 {
                n -= 1;
                ((this_ + 0x30) as *mut u32).write(n);
            } else {
                ((back + 0x22) as *mut u32).write(0);
                i += 1;
                slot = slot.wrapping_add(4);
            }
        }
    }
    unsafe { (this_ as *mut u32).write(arg); }
    arg
});
