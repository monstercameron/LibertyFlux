// original: 0x00DE6A50 scrolling_menu_adjust_selection
/// Compare the child interface's unsigned count with the supplied index. It
/// always calls virtual slot +0x218 once; when count and index differ, it
/// requests the count's item through slot +0x1E0 and calls that returned
/// item's slots +0x1B0 and +0x18 with argument zero. It then requests the
/// supplied index, calls the item's slots +0x1AC and +0x18 with argument one,
/// and returns the last result. Thiscall uses one stack argument.
lf_checker_rt::export!(thiscall, rw_de6a50(this: u32, item_index: u32) -> u32 {
    const CHILD: u32 = 0x1E0;
    const VTABLE: u32 = 0;
    const COUNT_SLOT: u32 = 0x21C;
    const ADJUST_SLOT: u32 = 0x218;
    const ITEM_SLOT: u32 = 0x1E0;
    const FIRST_ITEM_SLOT: u32 = 0x1B0;
    const ITEM_ACTION_SLOT: u32 = 0x18;
    const NEXT_ITEM_SLOT: u32 = 0x1AC;

    unsafe {
        unsafe fn virtual0(object: u32, slot: u32) -> u32 {
            let table = (object as *const u32).read_unaligned();
            let address = ((table + slot) as *const u32).read_unaligned();
            let method: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(address as usize);
            method(object)
        }
        unsafe fn virtual1(object: u32, slot: u32, value: u32) -> u32 {
            let table = (object as *const u32).read_unaligned();
            let address = ((table + slot) as *const u32).read_unaligned();
            let method: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(address as usize);
            method(object, value)
        }

        let child = ((this + CHILD) as *const u32).read_unaligned();
        let item_count = virtual0(child, COUNT_SLOT);
        let _ = virtual0(child, ADJUST_SLOT);

        if item_count != item_index {
            let item = virtual1(child, ITEM_SLOT, item_count);
            let _ = virtual0(item, FIRST_ITEM_SLOT);
            let _ = virtual1(item, ITEM_ACTION_SLOT, 0);
        }

        let selected_item = virtual1(child, ITEM_SLOT, item_index);
        let _ = virtual0(selected_item, NEXT_ITEM_SLOT);
        virtual1(selected_item, ITEM_ACTION_SLOT, 1)
    }
});

