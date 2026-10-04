// original: 0x008724A0 rage::crmtManager::vf2
//! Detach the manager's child if one is attached: call slot 8 of the
//! child's function table, then clear `this+4`. No return value is produced
//! (EAX keeps its entry value when there is nothing to detach, so the
//! contract compares no return channel).
export!(thiscall, rw_008724A0(this: *mut u8) -> u32 {
    unsafe {
        let child = *(this.add(4) as *const u32);
        if child != 0 {
            let vt = *(child as *const u32);
            let detach: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(8)) as *const u32) as usize);
            detach(child);
            *(this.add(4) as *mut u32) = 0;
        }
        0
    }
});
