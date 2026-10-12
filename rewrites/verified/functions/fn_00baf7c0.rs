// original: 0x00BAF7C0 world_pool_fn_BAF7C0
/// Update state bits on an object, then conditionally toggle one mode bit.
///
/// The first cdecl argument is an object pointer and the second is an
/// unsigned word. A null object returns immediately. Otherwise the word at
/// offset 0x264 is updated so bits 1 and 2 match the corresponding bits of
/// twice the input, with wrapping arithmetic. The mode word at 0x1e2 is only
/// changed at bit 0x1000 when its low nibble is 8 through 10 and a linked
/// child at 0x1bc passes both of its flag and mode checks.
lf_checker_rt::export!(cdecl, rw_baf7c0(
    object: u32, value: u32
) -> u32 {
    const MODE_WORD_OFFSET: usize = 0x1E2;
    const LINKED_CHILD_OFFSET: usize = 0x1BC;
    const STATE_WORD_OFFSET: usize = 0x264;
    const CHILD_FLAGS_OFFSET: usize = 0x28;
    const CHILD_MODE_OFFSET: usize = 0x1304;
    const MODE_MASK: u16 = 0x000F;
    const ACTIVE_MODE_MIN: u16 = 8;
    const ACTIVE_MODE_MAX: u16 = 10;
    const STATE_MASK: u32 = 0x6;
    const CHILD_FLAGS_MASK: u32 = 0x03C0;
    const CHILD_FLAGS_VALUE: u32 = 0x0080;
    const CHILD_MODE_REQUIRED: u32 = 1;
    const MODE_UPDATE_BIT: u16 = 0x1000;

    if object == 0 { return 0; }
    let base = object as usize;
    let mode_ptr = (base + MODE_WORD_OFFSET) as *mut u16;
    let state_ptr = (base + STATE_WORD_OFFSET) as *mut u32;
    let mode_word = unsafe { mode_ptr.read_unaligned() };
    let old_state = unsafe { state_ptr.read_unaligned() };
    let doubled_value = value.wrapping_add(value);
    let next_state = old_state ^ ((doubled_value ^ old_state) & STATE_MASK);
    unsafe { state_ptr.write_unaligned(next_state) };

    let mode = mode_word & MODE_MASK;
    if !(ACTIVE_MODE_MIN..=ACTIVE_MODE_MAX).contains(&mode) { return 0; }
    let child = unsafe { ((base + LINKED_CHILD_OFFSET) as *const u32).read_unaligned() };
    if child == 0 { return 0; }
    let child_base = child as usize;
    let child_flags = unsafe { ((child_base + CHILD_FLAGS_OFFSET) as *const u32).read_unaligned() };
    if child_flags & CHILD_FLAGS_MASK != CHILD_FLAGS_VALUE { return 0; }
    let child_mode = unsafe { ((child_base + CHILD_MODE_OFFSET) as *const u32).read_unaligned() };
    if child_mode != CHILD_MODE_REQUIRED { return 0; }

    let mode_bits = unsafe { mode_ptr.read_unaligned() };
    let next_mode = if next_state & 6 == 2 {
        mode_bits & !MODE_UPDATE_BIT
    } else {
        mode_bits | MODE_UPDATE_BIT
    };
    unsafe { mode_ptr.write_unaligned(next_mode) };
    0
});
