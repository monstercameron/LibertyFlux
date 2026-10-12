// original: 0x00A86A10 destroy_unflagged_slot_objects
/// Walks slot indices from the final entry down to zero. Entries whose flag
/// byte has the selected high bit set are skipped. For each remaining
/// non-null object pointer, calls virtual slot zero with the destruction flag.
/// The container pointer is in ECX; its fields are array, flags, count, stride.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a86a10(slots: u32) -> u32 {
    const ARRAY_POINTER: usize = 0x00;
    const FLAGS_POINTER: usize = 0x04;
    const SLOT_COUNT: usize = 0x08;
    const SLOT_STRIDE: usize = 0x0C;
    const DESTROYED_BIT: u8 = 0x80;

    let array = unsafe { ((slots as *const u8).add(ARRAY_POINTER) as *const u32).read_unaligned() };
    let flags = unsafe { ((slots as *const u8).add(FLAGS_POINTER) as *const u32).read_unaligned() };
    let count = unsafe { ((slots as *const u8).add(SLOT_COUNT) as *const u32).read_unaligned() };
    let stride = unsafe { ((slots as *const u8).add(SLOT_STRIDE) as *const u32).read_unaligned() };

    let mut index = count;
    while index != 0 {
        index -= 1;
        let flag = unsafe { ((flags as *const u8).add(index as usize)).read() };
        if flag & DESTROYED_BIT == 0 {
            let object = unsafe { ((array as *const u8).add((stride.wrapping_mul(index)) as usize) as *const u32).read_unaligned() };
            if object != 0 {
                let table = unsafe { (object as *const u32).read_unaligned() };
                let method = unsafe { (table as *const u32).read_unaligned() };
                let destroy: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(method as usize) };
                destroy(object, 1);
            }
        }
    }
    0
}
