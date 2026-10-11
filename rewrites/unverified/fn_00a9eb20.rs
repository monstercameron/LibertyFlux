// original: 0x00A9EB20 loader_register_resource
/// Finds the first unused 0x30-byte resource slot in a 256-entry table. It
/// marks that slot occupied, records the resource identifier and four words
/// from the supplied payload, then stores the supplied weight in the slot's
/// weight field. The result is six times the slot index plus the fixed table
/// bias. If every slot is occupied, it returns the address immediately after
/// the table without modifying it.
lf_checker_rt::export!(thiscall, rw_00A9EB20(this: u32, resource_id: u32, payload: u32, weight_bits: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0x40c0;
        const SLOT_STRIDE: u32 = 0x30;
        const SLOT_COUNT: u32 = 0x100;
        const SLOT_WEIGHT_BIAS: u32 = 0x15a;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let table = this.wrapping_add(SLOT_BASE);
        let mut index = 0u32;
        while index < SLOT_COUNT
            && read_u8(table.wrapping_add(index.wrapping_mul(SLOT_STRIDE))) != 0
        {
            index += 1;
        }
        if index == SLOT_COUNT {
            return table.wrapping_add(SLOT_COUNT.wrapping_mul(SLOT_STRIDE));
        }

        let slot = table.wrapping_add(index.wrapping_mul(SLOT_STRIDE));
        write_u8(slot, 1);
        write_u32(slot.wrapping_add(0x04), resource_id);
        write_u32(slot.wrapping_add(0x10), read_u32(payload));
        write_u32(slot.wrapping_add(0x14), read_u32(payload.wrapping_add(0x04)));
        write_u32(slot.wrapping_add(0x18), read_u32(payload.wrapping_add(0x08)));
        write_u32(slot.wrapping_add(0x1c), read_u32(payload.wrapping_add(0x0c)));
        write_u32(slot.wrapping_add(0x20), weight_bits);

        index.wrapping_add(SLOT_WEIGHT_BIAS).wrapping_mul(6)
    }
});
