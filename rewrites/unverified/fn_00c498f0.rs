// original: 0x00C498F0 camera_spline_copy_orientation

/// Copy four orientation words from the active spline state into `destination`.
///
/// If `this + 0x20` is non-null, the source block is that pointer plus `0x40`;
/// otherwise the source block begins at `this`. The four words are copied in
/// order to `destination`, including the two single-precision fields as raw
/// bits. The fourth source word is read after the first three destination
/// writes, preserving the original behavior when those ranges overlap. EAX
/// returns that fourth word. This thiscall routine takes one stack pointer and
/// the callee removes it.
lf_checker_rt::export!(thiscall, rw_c498f0(this: u32, destination: u32) -> u32 {
    unsafe {
        const ACTIVE_STATE_FIELD: u32 = 0x20;
        const ACTIVE_STATE_COMPONENTS: [u32; 3] = [0x40, 0x44, 0x48];
        const FALLBACK_COMPONENTS: [u32; 3] = [0x00, 0x04, 0x08];
        const FINAL_ACTIVE_COMPONENT: u32 = 0x4c;
        const FINAL_FALLBACK_COMPONENT: u32 = 0x0c;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) };
        }

        let active_state = read_word(this + ACTIVE_STATE_FIELD);
        let (first_three, final_component) = if active_state != 0 {
            let active = active_state.wrapping_add(ACTIVE_STATE_COMPONENTS[0]);
            let words = [
                read_word(active),
                read_word(active + (ACTIVE_STATE_COMPONENTS[1] - ACTIVE_STATE_COMPONENTS[0])),
                read_word(active + (ACTIVE_STATE_COMPONENTS[2] - ACTIVE_STATE_COMPONENTS[0])),
            ];
            (words, FINAL_ACTIVE_COMPONENT)
        } else {
            let words = [
                read_word(this + FALLBACK_COMPONENTS[0]),
                read_word(this + FALLBACK_COMPONENTS[1]),
                read_word(this + FALLBACK_COMPONENTS[2]),
            ];
            (words, FINAL_FALLBACK_COMPONENT)
        };

        write_word(destination, first_three[0]);
        write_word(destination + 4, first_three[1]);
        write_word(destination + 8, first_three[2]);

        let final_source = if active_state != 0 {
            active_state.wrapping_add(final_component)
        } else {
            this.wrapping_add(final_component)
        };
        let final_word = read_word(final_source);
        write_word(destination + 0x0c, final_word);
        final_word
    }
});
