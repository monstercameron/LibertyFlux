// original: 0x00b572c0 store_level_in_indexed_target
// rs05f11/rs05f12: resolve slot `index` of the table at `this+0x30C`.
#[inline(always)]
unsafe fn indexed_slot(this: *const u8, index: u32) -> *const u8 {
    let table = this.add(0x30C) as *const u32;
    *table.add(index as usize) as *const u8
}

// rs05f11: look up an indexed slot and store a level into its target.
//
// Resolves slot `index` at `this+index*4+0x30C`, takes the handle at `+0x8`
// (exits when null), refreshes it (thiscall/0), and when the returned
// target's word at `+0x18` is set, refreshes again and stores `level` at
// target `+0x10`.
export!(thiscall, rw_b572c0(this: *const u8, index: u32, level: f32) -> () {
    unsafe {
        let slot = indexed_slot(this, index);
        let handle = *(slot.add(8) as *const u32);
        if handle == 0 {
            return;
        }
        let refresh: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let target = refresh(handle) as *mut u8;
        if *(target.add(0x18) as *const u32) == 0 {
            return;
        }
        let target = refresh(handle) as *mut u8;
        *(target.add(0x10) as *mut f32) = level;
    }
});
