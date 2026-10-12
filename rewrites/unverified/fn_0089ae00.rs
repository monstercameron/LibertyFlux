// original: 0x0089AE00 rage::audEnvelopeSound::blend_child
/// Resolve the voice slot and blend its timing into an envelope child.
///
/// The thiscall takes the sound node and an optional byte output pointer. It
/// returns zero when the voice slot is absent or resolves to a null voice.
/// Otherwise it obtains the voice query result, asks the parameter resolver
/// for five scalar terms, and combines those terms using signed duration and
/// adjustment checks. A negative duration or a negative final sum falls back
/// to the voice query result. When a result is accepted, the optional output
/// byte is cleared; otherwise it receives the query helper's byte. Runtime
/// voice tables and both callees are supplied by the contract.
lf_checker_rt::export!(thiscall, rw_0089ae00(
    this: u32,
    output_flag: u32,
) -> u32 {
    unsafe {
        const VOICE_SLOT: u32 = 0x48;
        const CHANNEL: u32 = 0x40;
        const VOICE_STRIDE_GLOBAL: u32 = 0x0115_d964;
        const VOICE_TABLE_GLOBAL: u32 = 0x0115_d988;
        const VOICE_RECORD_STRIDE: u32 = 0x6f40;
        const VOICE_ARRAY_OFFSET: u32 = 0x6f10;
        const NO_VOICE: u8 = 0xff;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let slot = read_u8(this.wrapping_add(VOICE_SLOT));
        if slot == NO_VOICE {
            if output_flag != 0 { write_u8(output_flag, 0); }
            return 0;
        }

        let channel = u32::from(read_u8(this.wrapping_add(CHANNEL)));
        let table = read_u32(lf_checker_rt::relocated(VOICE_TABLE_GLOBAL));
        let stride = read_u32(lf_checker_rt::relocated(VOICE_STRIDE_GLOBAL));
        let voice_record = table
            .wrapping_add(channel.wrapping_mul(VOICE_RECORD_STRIDE))
            .wrapping_add(VOICE_ARRAY_OFFSET);
        let voice_base = read_u32(voice_record);
        let voice_address = voice_base.wrapping_add(u32::from(slot).wrapping_mul(stride));
        if voice_address == 0 {
            if output_flag != 0 { write_u8(output_flag, 0); }
            return 0;
        }

        let mut query_flag = 0u8;
        let mut query_result = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            voice_address,
            core::ptr::addr_of_mut!(query_flag) as u32,
        );

        let mut output_words = core::mem::MaybeUninit::<[u32; 5]>::uninit();
        let output_ptr = output_words.as_mut_ptr().cast::<u32>();
        let _ = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            this,
            output_ptr.add(0) as u32,
            output_ptr.add(1) as u32,
            output_ptr.add(2) as u32,
            output_ptr.add(3) as u32,
            output_ptr.add(4) as u32,
        );
        let words = output_words.assume_init();
        let term_b = words[0];
        let term_a = words[1];
        let signed_duration = words[3] as i32;
        let signed_adjustment = words[4] as i32;

        let mut result;
        if signed_duration < 0 {
            result = query_result;
        } else {
            result = term_b.wrapping_add(term_a).wrapping_add(words[3]);
            if signed_adjustment >= 0 {
                result = result.wrapping_add(words[4]);
            }
            if query_flag == 0 && (query_result as i32) >= 0 && (query_result as i32) < (result as i32) {
                result = query_result;
            }
            if (result as i32) < 0 {
                result = query_result;
            } else {
                query_flag = 0;
            }
        }

        if output_flag != 0 { write_u8(output_flag, query_flag); }
        result
    }
});
