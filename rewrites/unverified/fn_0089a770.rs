// original: 0x0089A770 rage::audEnvelopeSound::audEnvelopeSound_2
/// Initialize an envelope sound node and run the base constructor.
///
/// The thiscall object is a 0xfc-byte sound node. The method installs its
/// relocated class table, optionally starts the inherited voice when the
/// channel's voice address is nonzero, and releases a prior envelope-manager
/// slot when the stored slot is not `0xff`. It marks that slot unused after
/// releasing it, then forwards the object to the base constructor and returns
/// that constructor's result. The two direct callees and the final base
/// constructor are intercepted by this function's contract.
///
/// Channel records 0 through 2 and both zero and nonzero runtime strides
/// are exercised.
lf_checker_rt::export!(thiscall, rw_0089a770(
    this: u32,
) -> u32 {
    unsafe {
        const CHANNEL: u32 = 0x40;
        const VOICE_SLOT: u32 = 0x48;
        const ENVELOPE_SLOT: u32 = 0xf7;
        const VOICE_RECORD_STRIDE: u32 = 0x6f40;
        const VOICE_ARRAY_OFFSET: u32 = 0x6f10;
        const VOICE_TABLE_GLOBAL: u32 = 0x0115_d988;
        const VOICE_STRIDE_GLOBAL: u32 = 0x0115_d964;
        const CLASS_TABLE: u32 = 0x00e7_9b84;
        const ENVELOPE_MANAGER: u32 = 0x0115_d8a0;
        const NO_SLOT: u8 = 0xff;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        write_u32(this, lf_checker_rt::relocated(CLASS_TABLE));

        let voice_slot = read_u8(this.wrapping_add(VOICE_SLOT));
        if voice_slot != NO_SLOT {
            let channel = u32::from(read_u8(this.wrapping_add(CHANNEL)));
            let table = read_u32(lf_checker_rt::relocated(VOICE_TABLE_GLOBAL));
            let voice_record = table
                .wrapping_add(channel.wrapping_mul(VOICE_RECORD_STRIDE))
                .wrapping_add(VOICE_ARRAY_OFFSET);
            let voice_base = read_u32(voice_record);
            let stride = read_u32(lf_checker_rt::relocated(VOICE_STRIDE_GLOBAL));
            let voice_address = voice_base
                .wrapping_add(u32::from(voice_slot).wrapping_mul(stride));
            if voice_address != 0 {
                let _ = lf_checker_rt::callee_thiscall!(1, u32, this, 0);
            }
        }

        let envelope_slot = read_u8(this.wrapping_add(ENVELOPE_SLOT));
        if envelope_slot != NO_SLOT {
            let channel = u32::from(read_u8(this.wrapping_add(CHANNEL)));
            let manager = lf_checker_rt::relocated(ENVELOPE_MANAGER);
            let _ = lf_checker_rt::callee_thiscall!(2, u32, manager, channel, u32::from(envelope_slot));
            write_u8(this.wrapping_add(0xf7), NO_SLOT);
        }

        lf_checker_rt::callee_thiscall!(3, u32, this)
    }
});
