// original: 0x0099e3b0 voice_slot_id
/// Returns the voice id of an entity, acquiring it when the slot is empty.
///
/// When the slot at offset 0x9c is set, the cached word at 0xc8 is returned.
/// Otherwise the value is acquired through callees 1 and 2 and returned.
export!(thiscall, rw_0099e3b0(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x1288780;
        if *((this.wrapping_add(0x9c)) as *const u32) != 0 {
            return *((this.wrapping_add(0xc8)) as *const u16) as u32;
        }
        let r1 = callee_thiscall!(1, u32, this, 1);
        callee_thiscall!(2, u32, relocated(MANAGER), r1)
    }
});
