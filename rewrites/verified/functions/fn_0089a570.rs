// original: 0x0089A570 audio_stop_child_or_null
/// Stop the voice associated with a looping sound node, if the node has one.
///
/// The cdecl argument is the node pointer. A `0xff` slot is the no-voice
/// sentinel and returns the node pointer unchanged. Otherwise the channel's
/// voice-table entry and runtime slot stride form the voice pointer. A null
/// result returns zero; a non-null result is forwarded to the base stop
/// routine as its this pointer, and that routine's return value is returned.
///
/// The table has three covered channel records. The stride must be nonzero
/// because the corresponding operation in this method faults on zero. The
/// base stop routine is intercepted at both outgoing tail-call sites.
lf_checker_rt::export!(cdecl, rw_0089a570(
    this: u32,
) -> u32 {
    unsafe {
        const CHANNEL: u32 = 0x40;
        const VOICE_SLOT: u32 = 0x48;
        const VOICE_RECORD_STRIDE: u32 = 0x6f40;
        const VOICE_ARRAY_OFFSET: u32 = 0x6f10;
        const VOICE_TABLE_GLOBAL: u32 = 0x0115_d988;
        const VOICE_STRIDE_GLOBAL: u32 = 0x0115_d964;
        const NO_SLOT: u8 = 0xff;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let slot = read_u8(this.wrapping_add(VOICE_SLOT));
        if slot == NO_SLOT {
            return this;
        }

        let channel = u32::from(read_u8(this.wrapping_add(CHANNEL)));
        let table = read_u32(lf_checker_rt::relocated(VOICE_TABLE_GLOBAL));
        let slot_record = table
            .wrapping_add(channel.wrapping_mul(VOICE_RECORD_STRIDE))
            .wrapping_add(VOICE_ARRAY_OFFSET);
        let slot_base = read_u32(slot_record);
        let slot_stride = read_u32(lf_checker_rt::relocated(VOICE_STRIDE_GLOBAL));
        let voice = slot_base.wrapping_add(u32::from(slot).wrapping_mul(slot_stride));
        if voice == 0 {
            return 0;
        }

        lf_checker_rt::callee_thiscall!(1, u32, voice)
    }
});
