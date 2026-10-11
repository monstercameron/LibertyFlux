// original: 0x009BF0E0 CEventImminentDanger::vf4
/// Queries the event manager using its global pointer. A null query result
/// returns zero. Otherwise the result becomes ECX for a constructor call that
/// receives pointers to the event vectors, the scalar at byte offset 0x20 as
/// unchanged f32 bits, three adjacent dwords, and the low byte at offset 0x4c.
/// The constructor's EAX result is returned unchanged.
lf_checker_rt::export!(thiscall, rw_009bf0e0(this: u32) -> u32 {
    const MANAGER_GLOBAL_VA: u32 = 0x0166_69C8;
    const QUERY_VECTOR_OFFSET: u32 = 0x10;
    const SCALAR_BITS_OFFSET: u32 = 0x20;
    const CONSTRUCTOR_VECTOR_OFFSET: u32 = 0x30;
    const FIRST_WORD_OFFSET: u32 = 0x40;
    const SECOND_WORD_OFFSET: u32 = 0x44;
    const THIRD_WORD_OFFSET: u32 = 0x48;
    const FLAG_BYTE_OFFSET: u32 = 0x4C;

    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read() };
    let context = lf_checker_rt::callee_thiscall!(1, u32, manager);
    if context == 0 {
        return 0;
    }

    let query_vector = this.wrapping_add(QUERY_VECTOR_OFFSET);
    let scalar_bits = unsafe { ((this + SCALAR_BITS_OFFSET) as *const u32).read() };
    let constructor_vector = this.wrapping_add(CONSTRUCTOR_VECTOR_OFFSET);
    let first_word = unsafe { ((this + FIRST_WORD_OFFSET) as *const u32).read() };
    let second_word = unsafe { ((this + SECOND_WORD_OFFSET) as *const u32).read() };
    let third_word = unsafe { ((this + THIRD_WORD_OFFSET) as *const u32).read() };
    let flag = unsafe { ((this + FLAG_BYTE_OFFSET) as *const u8).read() } as u32;

    lf_checker_rt::callee_thiscall!(
        2,
        u32,
        context,
        query_vector,
        scalar_bits,
        constructor_vector,
        first_word,
        second_word,
        third_word,
        flag,
    )
});
