// original: 0x00862370 physics_copy_surface_state

/// Copies the source surface and motion records into the object's output
/// slots, preserving every copied dword's bit pattern. It clears flag bit 2
/// at `+0x27c`, writes the fixed unit and negative scale values, and returns
/// the source word from `+0x1a8`.
///
/// The caller supplies only the object pointer in ECX. No helper is called.
lf_checker_rt::export!(thiscall, rw_00862370(this: u32) -> u32 {
    unsafe {
        const SOURCE_SURFACE: u32 = 0x0100;
        const SOURCE_MOTION: u32 = 0x0120;
        const SOURCE_STATE: u32 = 0x01a8;
        const DEST_SURFACE: u32 = 0x01f0;
        const DEST_MOTION: u32 = 0x0200;
        const DEST_STATE: u32 = 0x0214;
        const DEST_SCALE: u32 = 0x0218;
        const DEST_UNIT: u32 = 0x021c;
        const FLAGS: u32 = 0x027c;
        const CLEAR_FLAG_BIT_2: u8 = 0xfb;
        const NEGATIVE_SCALE_BITS: u32 = 0xbe4c_cccd;
        const UNIT_BITS: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn read_word(base: u32, offset: u32) -> u32 {
            unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(base: u32, offset: u32, value: u32) {
            unsafe { ((base.wrapping_add(offset)) as *mut u32).write_unaligned(value) }
        }

        let surface_0 = read_word(this, SOURCE_SURFACE);
        let surface_1 = read_word(this, SOURCE_SURFACE + 4);
        let surface_2 = read_word(this, SOURCE_SURFACE + 8);
        let surface_3 = read_word(this, SOURCE_SURFACE + 12);
        let motion_0 = read_word(this, SOURCE_MOTION);
        let motion_1 = read_word(this, SOURCE_MOTION + 4);
        let motion_2 = read_word(this, SOURCE_MOTION + 8);
        let motion_3 = read_word(this, SOURCE_MOTION + 12);
        let source_state = read_word(this, SOURCE_STATE);

        write_word(this, DEST_MOTION, motion_0);
        write_word(this, DEST_MOTION + 4, motion_1);
        write_word(this, DEST_MOTION + 8, motion_2);
        write_word(this, DEST_MOTION + 12, motion_3);
        write_word(this, DEST_UNIT, UNIT_BITS);
        write_word(this, DEST_SURFACE, surface_0);
        write_word(this, DEST_SURFACE + 4, surface_1);
        write_word(this, DEST_SURFACE + 8, surface_2);
        write_word(this, DEST_SURFACE + 12, surface_3);
        write_word(this, DEST_STATE, source_state);
        *((this.wrapping_add(FLAGS)) as *mut u8) =
            *((this.wrapping_add(FLAGS)) as *const u8) & CLEAR_FLAG_BIT_2;
        write_word(this, DEST_SCALE, NEGATIVE_SCALE_BITS);

        source_state
    }
});
