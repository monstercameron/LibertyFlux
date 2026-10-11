// original: 0x00BC2CB0 task_table_method_forwarder (proposed)

/// Forward two caller arguments and the word at `this + 0x20` through the
/// second function-table slot stored at `this + 0x74`. The target receives
/// `this + 0x74` in ECX and three stack words; its return value is ignored.
/// The wrapper returns its first input word and removes both input words.
lf_checker_rt::export!(thiscall, rw_00BC2CB0(this: u32, first: u32, second: u32) -> u32 {
    const TABLE_FIELD: u32 = 0x74;
    const OBJECT_ARGUMENT: u32 = 0x20;
    const METHOD_SLOT: u32 = 4;

    unsafe {
        let method_table = ((this.wrapping_add(TABLE_FIELD)) as *const u32).read_unaligned();
        let method_address = ((method_table.wrapping_add(METHOD_SLOT)) as *const u32).read_unaligned();
        let object_argument = ((this.wrapping_add(OBJECT_ARGUMENT)) as *const u32).read_unaligned();
        let method: unsafe extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(method_address as usize);
        let _method_result = method(this.wrapping_add(TABLE_FIELD), second, object_argument, first);
    }
    first
});
