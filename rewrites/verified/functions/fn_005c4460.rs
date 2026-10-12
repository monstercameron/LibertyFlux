// original: 0x005C4460 ped_task_item_matches_virtual_identifier

/// Checks a signed 16-bit identifier stored in the optional object at offset
/// 0x2c4 against the value returned by this object's virtual slot at offset
/// 0x12c. The item identifier is sign-extended before an exact 32-bit equality
/// comparison. A null item returns false without making the virtual call.
lf_checker_rt::export!(thiscall, rw_005c4460(this: u32) -> u32 {
    unsafe {
        const VTABLE_POINTER: u32 = 0x00;
        const ITEM_POINTER: u32 = 0x2c4;
        const ITEM_IDENTIFIER: u32 = 0x2e;
        const VTABLE_SLOT: u32 = 0x12c;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_signed_halfword(address: u32) -> i32 {
            unsafe { (address as *const i16).read_unaligned() as i32 }
        }

        let item = read_word(this + ITEM_POINTER);
        if item == 0 {
            return 0;
        }
        let item_identifier = read_signed_halfword(item + ITEM_IDENTIFIER) as u32;
        let vtable = read_word(this + VTABLE_POINTER);
        let target = read_word(vtable + VTABLE_SLOT);
        let lookup: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        u32::from(item_identifier == lookup(this))
    }
});
