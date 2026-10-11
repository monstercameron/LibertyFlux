// original: 0x005E1310 proposed_ped_task_dispatch_list_cleanup
/// Clear a five-entry collection after dispatching each present item through
/// its virtual handler and three helper calls. The collection object stores
/// the entry array pointer at byte offset `0x1c`; each array entry is a 32-bit
/// object pointer. A present item supplies its vtable at offset zero and the
/// handler at vtable offset `0xd0`. The handler receives the item as `this`
/// and a zero word. Its result becomes `this` for the first direct helper; the
/// next helper receives the item and the bit pattern for `1.0f`; the final
/// helper receives the item and the address of its array slot. Each processed
/// slot is cleared. Null slots are skipped. Calls use the original order and
/// thiscall cleanup so the checker can intercept them independently.
lf_checker_rt::export!(thiscall, rw_005e1310(this: u32) -> u32 {
    const ITEMS_OFFSET: u32 = 0x1c;
    const ITEM_VTABLE_OFFSET: u32 = 0;
    const HANDLER_SLOT_OFFSET: u32 = 0xd0;
    const ITEM_COUNT: u32 = 5;
    const UNIT_WEIGHT_BITS: u32 = 0x3f80_0000;
    const HANDLER_ID: u32 = 1;
    const AFTER_HANDLER_ID: u32 = 2;
    const APPLY_WEIGHT_ID: u32 = 3;
    const RELEASE_SLOT_ID: u32 = 4;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let items = read_u32(this.wrapping_add(ITEMS_OFFSET));
        for index in 0..ITEM_COUNT {
            let slot = items.wrapping_add(index.wrapping_mul(4));
            let item = read_u32(slot);
            if item == 0 {
                continue;
            }

            let vtable = read_u32(item.wrapping_add(ITEM_VTABLE_OFFSET));
            let handler = read_u32(vtable.wrapping_add(HANDLER_SLOT_OFFSET));
            let call_handler: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(handler as usize);
            let handler_result = call_handler(item, 0);
            let helper_receiver =
                lf_checker_rt::callee_thiscall!(AFTER_HANDLER_ID, u32, handler_result);
            let _ = helper_receiver;
            let _ = lf_checker_rt::callee_thiscall!(
                APPLY_WEIGHT_ID,
                u32,
                item,
                UNIT_WEIGHT_BITS
            );
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_SLOT_ID, u32, item, slot);
            write_u32(slot, 0);
        }
    }
    0
});
