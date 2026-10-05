// original: 0x00a13a50 model_slot_lookup (proposed)
/// Look up the model slot for the cached target, or null when unusable.
///
/// Follows the object at `this + 0x170` to its identity (the signed word at
/// `+0x2e`), indexes the global table with it, and reads the slot number
/// (the signed word at `+0x3c4` of the entry). Returns null when there is no
/// object or the slot is -1, otherwise the address `TABLE + (slot * 5) * 128`.
/// Thiscall, no stack arguments.
export!(thiscall, rw_00a13a50(this: u32) -> u32 {
    unsafe {
        const OBJ_OFF: u32 = 0x170;
        const ID_OFF: u32 = 0x2e;
        const SLOT_OFF: u32 = 0x3c4;
        const TABLE: u32 = 0x01295cd8;
        const SLOTS: u32 = 0x012bd1e0;
        let obj = ((this + OBJ_OFF) as *const u32).read_unaligned();
        if obj == 0 {
            return 0;
        }
        let id = ((obj + ID_OFF) as *const u16).read_unaligned() as i16 as i32;
        let tab = relocated(TABLE) as *const u32;
        let ent = tab.wrapping_add(id as usize).read_unaligned();
        let slot = ((ent + SLOT_OFF) as *const u16).read_unaligned() as i16 as i32;
        if slot == -1 {
            return 0;
        }
        relocated(SLOTS).wrapping_add(((slot * 5) << 7) as u32)
    }
});
