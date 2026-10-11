// original: 0x00C116B0 streaming_record_update_flag_mask

/// Set or clear selected bits in the stream record at +0x399. The original
/// branches on the low byte of `set`: zero ORs the mask into the byte; nonzero
/// clears those bits.
lf_checker_rt::export!(thiscall, rw_00c116b0(this: u32, set: u32, mask: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x399;
        let flags = (this + FLAGS) as *mut u8;
        if (set as u8) == 0 {
            let old = flags.read();
            flags.write(old | mask as u8);
        } else {
            let old = flags.read();
            flags.write(old & !(mask as u8));
        }
    }
    0
});
