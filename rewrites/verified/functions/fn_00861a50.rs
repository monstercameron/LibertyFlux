// original: 0x00861A50 physics_copy_position_state

/// Copies a header word and two position and velocity records from `this` into
/// output slots at `+0x1e0`, `+0x1c0` and `+0x1d0`. The copies preserve each
/// source dword bit for bit. It also clears flag bit 1 at `+0x27c`, writes the
/// fixed scale values, and returns the source word from `+0x1a4`.
///
/// The caller supplies only the object pointer in ECX. No helper is called.
lf_checker_rt::export!(thiscall, rw_00861a50(this: u32) -> u32 {
    unsafe {
        const SOURCE_POSITION: u32 = 0x00f0;
        const SOURCE_VELOCITY: u32 = 0x0110;
        const SOURCE_HEADER: u32 = 0x0130;
        const SOURCE_STATE: u32 = 0x01a4;
        const DEST_POSITION: u32 = 0x01c0;
        const DEST_VELOCITY: u32 = 0x01d0;
        const DEST_HEADER: u32 = 0x01e0;
        const DEST_STATE: u32 = 0x01e4;
        const DEST_SCALE: u32 = 0x01e8;
        const DEST_UNIT: u32 = 0x01ec;
        const FLAGS: u32 = 0x027c;
        const CLEAR_FLAG_BIT_1: u8 = 0xfd;
        const SCALE_BITS: u32 = 0x3e4c_cccd;
        const UNIT_BITS: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn read_word(base: u32, offset: u32) -> u32 {
            unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(base: u32, offset: u32, value: u32) {
            unsafe { ((base.wrapping_add(offset)) as *mut u32).write_unaligned(value) }
        }

        let position_0 = read_word(this, SOURCE_POSITION);
        let position_1 = read_word(this, SOURCE_POSITION + 4);
        let position_2 = read_word(this, SOURCE_POSITION + 8);
        let position_3 = read_word(this, SOURCE_POSITION + 12);
        let velocity_0 = read_word(this, SOURCE_VELOCITY);
        let velocity_1 = read_word(this, SOURCE_VELOCITY + 4);
        let velocity_2 = read_word(this, SOURCE_VELOCITY + 8);
        let velocity_3 = read_word(this, SOURCE_VELOCITY + 12);
        let source_header = read_word(this, SOURCE_HEADER);
        let source_state = read_word(this, SOURCE_STATE);

        write_word(this, DEST_HEADER, source_header);
        write_word(this, DEST_VELOCITY, velocity_0);
        write_word(this, DEST_VELOCITY + 4, velocity_1);
        write_word(this, DEST_VELOCITY + 8, velocity_2);
        write_word(this, DEST_VELOCITY + 12, velocity_3);
        write_word(this, DEST_UNIT, UNIT_BITS);
        write_word(this, DEST_POSITION, position_0);
        write_word(this, DEST_POSITION + 4, position_1);
        write_word(this, DEST_POSITION + 8, position_2);
        write_word(this, DEST_POSITION + 12, position_3);
        write_word(this, DEST_STATE, source_state);
        *((this.wrapping_add(FLAGS)) as *mut u8) =
            *((this.wrapping_add(FLAGS)) as *const u8) & CLEAR_FLAG_BIT_1;
        write_word(this, DEST_SCALE, SCALE_BITS);

        source_state
    }
});
