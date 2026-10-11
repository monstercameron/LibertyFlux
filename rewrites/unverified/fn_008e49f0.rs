// original: 0x008E49F0 frontend_audio_notification

/// Build the temporary frontend notification record and submit it for the
/// current entity.
///
/// The one stack argument is forwarded unchanged as the notification value.
/// A zeroed 72-byte local record is passed to an initializer, then byte
/// `0x46` is ORed with `0x08`. The entity helper receives the value, a pointer
/// to that record, `u32::MAX`, and two zero words. If the scripted status
/// helper returns a nonzero low byte, a final registration call receives the
/// manager and callback values read from their global slots. This is a
/// 32-bit thiscall method; its return register is not a semantic result.
lf_checker_rt::export!(thiscall, rw_008e49f0(this: u32, value: u32) -> u32 {
    unsafe {
        const LOCAL_RECORD_SIZE: usize = 72;
        const LOCAL_FLAG_BYTE: usize = 0x46;
        const LOCAL_FLAG: u8 = 0x08;
        const MANAGER_GLOBAL: u32 = 0x115f848;
        const CALLBACK_GLOBAL: u32 = 0x11618fc;

        let mut local_record = [0u8; LOCAL_RECORD_SIZE];
        let local_record_ptr = local_record.as_mut_ptr() as usize as u32;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, local_record_ptr);
        local_record[LOCAL_FLAG_BYTE] |= LOCAL_FLAG;
        let _ = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            this,
            value,
            local_record_ptr,
            u32::MAX,
            0,
            0
        );
        if lf_checker_rt::callee_cdecl!(3, u8) != 0 {
            let manager = (lf_checker_rt::global::<u32>(MANAGER_GLOBAL)).read_unaligned();
            let callback = (lf_checker_rt::global::<u32>(CALLBACK_GLOBAL)).read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(4, u32, manager, callback);
        }
        0
    }
});
