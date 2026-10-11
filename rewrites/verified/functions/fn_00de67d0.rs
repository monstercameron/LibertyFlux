// original: 0x00DE67D0 UIGTAScrollingRow::vf86
/// Store the row's selection byte in the indexed record, sample four float
/// properties through thiscall virtual slots +0xC8, +0xD0, +0xB8, and +0xC0
/// when the state byte at +0x218 is nonzero, then set the record's ready
/// byte. The index is the global word multiplied by 68 bytes. The method
/// forwards this to its base routine after updating the record.
lf_checker_rt::export!(thiscall, rw_de67d0(this: u32) -> u32 {
    const SELECTION_INDEX: u32 = 0x017A65A8;
    const STATE: u32 = 0x218;
    const RECORD_FLAGS: u32 = 0x12C;
    const RECORD_VALUES: u32 = 0xF0;
    const RECORD_READY: u32 = 0x130;
    const RECORD_STRIDE: u32 = 68;
    const SAMPLE_SLOTS: [u32; 4] = [0xC8, 0xD0, 0xB8, 0xC0];

    unsafe {
        let index = lf_checker_rt::global::<u32>(SELECTION_INDEX).read_unaligned();
        let record_offset = index.wrapping_mul(RECORD_STRIDE);
        let state = ((this + STATE) as *const u8).read();
        ((this + RECORD_FLAGS + record_offset) as *mut u8).write(state);

        if state != 0 {
            let table = (this as *const u32).read_unaligned();
            for (sample, slot) in SAMPLE_SLOTS.iter().copied().enumerate() {
                let address = ((table + slot) as *const u32).read_unaligned();
                let method: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(address as usize);
                let value = method(this);
                ((this + RECORD_VALUES + record_offset + (sample as u32) * 4) as *mut u32)
                    .write_unaligned(value.to_bits());
            }
        }

        ((this + RECORD_READY + record_offset) as *mut u8).write(1);
    }

    lf_checker_rt::callee_thiscall!(5, u32, this)
});

