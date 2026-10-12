// original: 0x005ED530 CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorStandard>::vf0

/// Destroy a move-seek task, release its optional calculator, and free its slot.
///
/// ECX points to the task and the stack word's low bit requests pool deletion.
/// The method saves the calculator pointer at offset `0x20`, installs the
/// relocated task and subobject vtables, conditionally calls the calculator
/// release helper with both the calculator and the address of its stored
/// pointer, installs the final base vtable, then calls the base destructor.
/// With deletion requested, it computes the signed pool index, marks the
/// matching free-bitmap byte, lowers the signed high-water mark when needed,
/// and decrements the free count with 32-bit wrapping. Both direct calls are
/// scripted by the contract, so their internal effects are outside this proof.
/// The pool contract uses positive stride 64 and a receiver at slot index 1.
lf_checker_rt::export!(thiscall, rw_005ed530(this: *mut u8, delete_flags: u32) -> *mut u8 {
    const TASK_POOL_CONTEXT_VA: u32 = 0x0167e2a0;
    const RELEASE_CALCULATOR: u32 = 1;
    const BASE_DESTRUCTOR: u32 = 2;
    const TASK_VTABLE_VA: u32 = 0x00eb76ac;
    const CALCULATOR_VTABLE_VA: u32 = 0x00eb7704;
    const FINAL_TASK_VTABLE_VA: u32 = 0x00eb7688;
    const POOL_BASE_WORD: usize = 0;
    const FREE_BITS_WORD: usize = 1;
    const POOL_STRIDE_WORD: usize = 3;
    const HIGH_WATER_WORD: usize = 4;
    const FREE_COUNT_WORD: usize = 5;
    const FREE_BIT: u8 = 0x80;

    unsafe {
        let calculator = this.add(0x20).cast::<*mut u8>().read_unaligned();
        let calculator_slot = this.add(0x20);

        this.cast::<u32>()
            .write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        this.add(0x14)
            .cast::<u32>()
            .write_unaligned(lf_checker_rt::relocated(CALCULATOR_VTABLE_VA));

        if !calculator.is_null() {
            let calculator_address = calculator as u32;
            let slot_address = calculator_slot as u32;
            let _ = lf_checker_rt::callee_thiscall!(
                RELEASE_CALCULATOR,
                u32,
                calculator_address,
                slot_address
            );
        }

        this.add(0x74)
            .cast::<u32>()
            .write_unaligned(lf_checker_rt::relocated(FINAL_TASK_VTABLE_VA));
        let this_address = this as u32;
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
