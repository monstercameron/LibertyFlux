// original: 0x005EB200 CTaskComplexSearchForPedOnFoot::~CTaskComplexSearchForPedOnFoot__deleting

/// Destroy a search-for-ped-on-foot task and optionally release its pool slot.
///
/// The thiscall receiver is the task; the single stack word's low bit requests
/// deletion. The method installs the relocated task vtable, calls the base
/// task destructor, and returns the receiver. With deletion requested, it
/// computes the signed slot index from the pool base and stride, sets bit 7 in
/// that slot's byte, lowers the signed high-water mark when the index is
/// smaller, and decrements the free-slot count with 32-bit wrapping. The base
/// destructor is scripted by the contract; its internal effects are outside
/// this function's proof. The pool contract uses a positive stride and a
/// receiver at a valid pool slot.
lf_checker_rt::export!(thiscall, rw_005eb200(this: *mut u8, delete_flags: u32) -> *mut u8 {
    const TASK_POOL_CONTEXT_VA: u32 = 0x0167e2a0;
    const BASE_DESTRUCTOR: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00fe137c;
    const POOL_BASE_WORD: usize = 0;
    const FREE_BITS_WORD: usize = 1;
    const POOL_STRIDE_WORD: usize = 3;
    const HIGH_WATER_WORD: usize = 4;
    const FREE_COUNT_WORD: usize = 5;
    const FREE_BIT: u8 = 0x80;

    unsafe {
        let this_address = this as u32;
        this.cast::<u32>()
            .write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this_address);

        if delete_flags & 1 != 0 {
            let context_address =
                lf_checker_rt::global::<u32>(TASK_POOL_CONTEXT_VA).read_unaligned();
            let context = context_address as *mut u32;
            let pool_base = context.add(POOL_BASE_WORD).read_unaligned();
            let free_bits = context.add(FREE_BITS_WORD).read_unaligned() as *mut u8;
            let stride = context.add(POOL_STRIDE_WORD).read_unaligned() as i32;
            let slot_index = this_address.wrapping_sub(pool_base) as i32 / stride;

            let slot_byte = free_bits.wrapping_offset(slot_index as isize);
            slot_byte.write(slot_byte.read() | FREE_BIT);

            let high_water = context.add(HIGH_WATER_WORD);
            let current_high_water = high_water.read_unaligned() as i32;
            if slot_index < current_high_water {
                high_water.write_unaligned(slot_index as u32);
            }

            let free_count = context.add(FREE_COUNT_WORD);
            free_count.write_unaligned(free_count.read_unaligned().wrapping_sub(1));
        }

        this
    }
});
