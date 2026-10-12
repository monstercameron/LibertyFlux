// original: 0x00b15360 load_packed_audio_record

/// Read a four-byte header and an eleven-word packed record from the stream.
/// When the byte flag at 0x0116d27d is zero, pass the packed record and stream
/// to the record unpacker. The stream reads and unpacker are scripted external
/// effects. Before returning, force AL to one and perform the stack-cookie
/// check. The routine uses thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00B15360(stream: u32) -> u32 {
    unsafe {
        const COOKIE_VA: u32 = 0x0105_7FB4;
        const UNPACK_FLAG_VA: u32 = 0x0116_D27D;
        const PACKED_WORDS: usize = 11;
        const READ_RECORD_BYTES: u32 = 0x2c;

        let mut local_words = core::mem::MaybeUninit::<[u32; PACKED_WORDS + 1]>::uninit();
        let words = local_words.as_mut_ptr().cast::<u32>();
        let _header_status = lf_checker_rt::callee_cdecl!(
            1, u32, words as u32, 4u32
        );
        let read_status = lf_checker_rt::callee_cdecl!(
            2, u32, words.add(1) as u32, READ_RECORD_BYTES
        );

        let prior_eax = if core::ptr::read_volatile(lf_checker_rt::global::<u8>(UNPACK_FLAG_VA)) == 0 {
            lf_checker_rt::callee_thiscall!(
                3, u32, words.add(1) as u32, stream
            )
        } else {
            read_status
        };
        let result = (prior_eax & !0xff) | 1;
        let cookie = core::ptr::read_volatile(lf_checker_rt::global::<u32>(COOKIE_VA));
        let _ = lf_checker_rt::callee_thiscall!(4, u32, cookie);
        result
    }
});
