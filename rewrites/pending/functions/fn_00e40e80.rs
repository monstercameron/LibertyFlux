// original: 0x00e40e80 table_slot_store_notify
// table slot store with two notifications.
// If the table pointer at this+0x50 is null, does nothing. Otherwise stores
// value at table+index*0x7c and notifies twice with (slot, aux). The table
// pointer is re-read before each call. Returns nothing meaningful.
export!(thiscall, rw_00e40e80(this_obj: u32, value: u32, index: u32, aux0: u32, aux1: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x50;
        const STRIDE: u32 = 0x7c;
        let table = *((this_obj.wrapping_add(TABLE_OFF)) as *const u32);
        if table == 0 {
            return 0;
        }
        let slot = table.wrapping_add(index.wrapping_mul(STRIDE));
        *(slot as *mut u32) = value;
        let table = *((this_obj.wrapping_add(TABLE_OFF)) as *const u32);
        callee_thiscall!(1, u32, table.wrapping_add(index.wrapping_mul(STRIDE)), aux0);
        let table = *((this_obj.wrapping_add(TABLE_OFF)) as *const u32);
        callee_thiscall!(2, u32, table.wrapping_add(index.wrapping_mul(STRIDE)), aux1);
        0
    }
});
