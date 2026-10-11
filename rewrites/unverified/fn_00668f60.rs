// original: 0x00668F60 rage::ptxEventEmitter::vf8

/// The event branch requires a non-null inner object at offset +0x48, two nonzero stack words, and a nonzero word from the selected data-table entry. The table index is the byte at offset +2 of the relocated manager global. The function then calls inner-object vtable slot +0x44 with the two stack words followed by this object's +0x4c and +0x0c words. The callee is thiscall with four stack arguments. The no-call path leaves EAX unspecified, so the proof compares memory and outgoing calls but disables the return-channel check.
lf_checker_rt::export!(thiscall, rw_00668F60(this: u32, event_key: u32, data_table: u32) -> u32 {
    const INNER_OBJECT: u32 = 0x48;
    const EVENT_POINTER: u32 = 0x0c;
    const FORWARDED_MEMBER: u32 = 0x4c;
    const TABLE_INDEX: u32 = 0x14;
    const VTABLE_SLOT: u32 = 0x44;
    const MANAGER_GLOBAL_VA: u32 = 0x01bb6678;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    let inner_object = unsafe { read_u32(this.wrapping_add(INNER_OBJECT)) };
    if inner_object == 0 || event_key == 0 || data_table == 0 {
        return 0;
    }
    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned() };
    let selected_index = unsafe { (manager.wrapping_add(2) as *const u8).read() as u32 };
    let selected_word = unsafe {
        read_u32(data_table.wrapping_add(TABLE_INDEX).wrapping_add(selected_index.wrapping_mul(4)))
    };
    if selected_word == 0 {
        return 0;
    }
    let forwarded_member = unsafe { read_u32(this.wrapping_add(FORWARDED_MEMBER)) };
    let event_pointer = unsafe { read_u32(this.wrapping_add(EVENT_POINTER)) };
    let vtable = unsafe { read_u32(inner_object) };
    let target = unsafe { read_u32(vtable.wrapping_add(VTABLE_SLOT)) };
    let method: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    let _ignored_result = method(inner_object, event_key, data_table, forwarded_member, event_pointer);
    0
});
