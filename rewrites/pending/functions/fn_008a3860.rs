// original: 0x008a3860 audSound_slot_notify
/// Notify the voice and the aggregate about this sound's slot.
///
/// Original 0x008A3860 (`thiscall(this, flag)`): resolves the bank slot
/// (0 when the selector is 0xFF), reports it plus the word at +0x54 to one
/// callee, reports it plus `flag` to a second callee, then runs a third
/// callee on `this` and returns its answer.
export!(thiscall, rw_008a3860(this: u32, flag: u32) -> u32 {    let id = unsafe { ((this + 0x48) as *const u8).read_unaligned() } as u32;
    let bank = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
    let slot = if id == 0xFF { 0 } else { ({
        let __stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
        let __base = unsafe { global::<u32>(0x115d988).read_unaligned() };
        let __entry = unsafe {
            (__base
                .wrapping_add((bank).wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32)
                .read_unaligned()
        };
        __entry.wrapping_add(__stride.wrapping_mul(id))
    }) };
    let w54 = unsafe { ((this + 0x54) as *const u32).read_unaligned() };
    callee_thiscall!(1, u32, slot, w54, 0);
    let id2 = unsafe { ((this + 0x48) as *const u8).read_unaligned() } as u32;
    let bank2 = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
    let slot2 = if id2 == 0xFF { 0 } else { ({
        let __stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
        let __base = unsafe { global::<u32>(0x115d988).read_unaligned() };
        let __entry = unsafe {
            (__base
                .wrapping_add((bank2).wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32)
                .read_unaligned()
        };
        __entry.wrapping_add(__stride.wrapping_mul(id2))
    }) };
    callee_thiscall!(2, u32, slot2, flag);
    callee_thiscall!(3, u32, this)
});
