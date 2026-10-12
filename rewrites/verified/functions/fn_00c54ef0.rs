// original: 0x00C54EF0 initialize_sized_pool_record (proposed)
/// Initialize a pool record and set its capacity field to `0x30` only when
/// the stack argument equals five; every other value selects `0x10`. The base
/// pool initializer receives `this + 4` in ECX and the same argument on the
/// stack, through a checker stand-in. Sixteen consecutive dwords beginning at
/// `this + 0x18` are cleared, as are the status and final words. The relocated
/// type tag is stored at offset zero; thiscall removes the argument and
/// returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_c54ef0(this: u32, pool_kind: u32) -> u32 {
    unsafe {
        const TYPE_TAG_VA: u32 = 0x00ec_b014;
        const FIRST_CLEAR_WORD: u32 = 0x18;
        const CLEAR_WORDS: u32 = 16;
        let type_tag = lf_checker_rt::relocated(TYPE_TAG_VA);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(4), pool_kind);
        (this as *mut u32).write_unaligned(type_tag);
        (this.wrapping_add(0x10) as *mut u32).write_unaligned(0);
        let mut index = 0u32;
        while index < CLEAR_WORDS {
            let offset = FIRST_CLEAR_WORD + index * 4;
            (this.wrapping_add(offset) as *mut u32).write_unaligned(0);
            index += 1;
        }
        let capacity = if pool_kind == 5 { 0x30 } else { 0x10 };
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(capacity);
        (this.wrapping_add(0x58) as *mut u32).write_unaligned(0);
        this
    }
});
