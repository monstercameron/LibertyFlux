// original: 0x005EBB30 CTaskComplexSearchForPedInCar::CTaskComplexSearchForPedInCar_2

/// Initialize a search-for-ped-in-car task from its base task and position.
///
/// ECX points to the destination task, the stack argument points to three
/// source words, and XMM2's low word supplies the float stored at offset
/// `0x30`. The base constructor is called with the destination as `this`.
/// Afterwards the constructor stores its relocated vtable pointer, copies the
/// three source words to offsets `0x20`, `0x24` and `0x28`, clears byte `0x34`
/// and word `0x38`, and returns the destination in EAX. The base constructor
/// is intercepted by the contract, so its internal initialization is outside
/// this function's proof.
lf_checker_rt::export!(thiscall, rw_005ebb30(this: *mut u8, source: *const u8) -> *mut u8 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const BASE_VTABLE_VA: u32 = 0x00fe1484;
    const SOURCE_WORD_0: usize = 0x00;
    const SOURCE_WORD_1: usize = 0x04;
    const SOURCE_WORD_2: usize = 0x08;
    const TASK_WORD_0: usize = 0x20;
    const TASK_WORD_1: usize = 0x24;
    const TASK_WORD_2: usize = 0x28;
    const TASK_RADIUS: usize = 0x30;
    const TASK_FLAG: usize = 0x34;
    const TASK_STATE: usize = 0x38;

    unsafe {
        let base_this = this as u32;
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, base_this);
        let relocated_vtable = lf_checker_rt::relocated(BASE_VTABLE_VA);
        this.cast::<u32>().write_unaligned(relocated_vtable);

        let source_value_0 = source.add(SOURCE_WORD_0).cast::<u32>().read_unaligned();
        let source_value_1 = source.add(SOURCE_WORD_1).cast::<u32>().read_unaligned();
        let source_value_2 = source.add(SOURCE_WORD_2).cast::<u32>().read_unaligned();
        this.add(TASK_WORD_0).cast::<u32>().write_unaligned(source_value_0);
        this.add(TASK_WORD_1).cast::<u32>().write_unaligned(source_value_1);
        this.add(TASK_WORD_2).cast::<u32>().write_unaligned(source_value_2);

        let radius_bits = lf_checker_rt::xmm_word(2, 0);
        this.add(TASK_RADIUS).cast::<u32>().write_unaligned(radius_bits);
        this.add(TASK_FLAG).write(0);
        this.add(TASK_STATE).cast::<u32>().write_unaligned(0);
        this
    }
});
