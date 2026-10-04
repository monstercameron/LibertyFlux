// original: 0x00979e30 audio_bank_store_entry
/// Store one entry into a fixed audio bank.
///
/// Appends arg0, the 16 bytes at arg1 and the arg2 sample to slot
/// `count` of the bank at +0xfa50 and bumps the count at +0xfe50.
/// A full bank (32 entries) is left unchanged.
export!(thiscall, rw_00979e30(this: *mut u8, arg0: u32, arg1: *const u8, arg2: u32, _pad: u32) -> u32 {
    unsafe {
        let countp = this.add(0xfe50) as *mut u32;
        let count = *countp;
        // Signed comparison: negative counts take the store path with a
        // wrapped slot address, exactly like the original's `jge`.
        if (count as i32) < 0x20 {
            let slot_off = 0xfa50u32.wrapping_add(count.wrapping_mul(32));
            let slot = (this as u32).wrapping_add(slot_off) as *mut u8;
            *countp = count.wrapping_add(1);
            *((slot.add(0x10)) as *mut u32) = arg0;
            *((slot.add(0)) as *mut u32) = *((arg1.add(0)) as *const u32);
            *((slot.add(4)) as *mut u32) = *((arg1.add(4)) as *const u32);
            *((slot.add(8)) as *mut u32) = *((arg1.add(8)) as *const u32);
            *((slot.add(0xc)) as *mut u32) = *((arg1.add(0xc)) as *const u32);
            *((slot.add(0x14)) as *mut u32) = arg2;
        }
        0
    }
});
