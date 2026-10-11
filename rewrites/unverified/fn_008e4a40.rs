// original: 0x008E4A40 STARTING_TUNE

/// Initialize and submit the starting-tune record for this audio entity.
///
/// The entity is supplied in ECX. A 16-bit state at byte offset `0x04` gates
/// setup when it equals `u16::MAX`; a nonzero dword at `0x50` also gates setup.
/// Otherwise the method initializes a 72-byte temporary record, submits it
/// with the entity, and, when the submission writes a nonzero handle at
/// `0x50`, performs follow-up registration calls. Callee-owned work is
/// forwarded through checker stand-ins. This is a 32-bit thiscall method
/// without stack arguments; no return register carries a semantic result.
lf_checker_rt::export!(thiscall, rw_008e4a40(this: u32) -> u32 {
    unsafe {
        const STATE_WORD: usize = 0x04;
        const HANDLE_DWORD: usize = 0x50;
        const RECORD_SIZE: usize = 72;
        const RECORD_VALUE: usize = 0x00;
        const STARTING_TUNE_BITS: u32 = 0x40c0_0000;
        const TUNE_RESOURCE: u32 = 0x00e8_2ee8;
        const FOLLOWUP_RESOURCE: u32 = 0x00e8_2ef8;
        const RESOURCE_SLOT_GLOBAL: u32 = 0x0103_0c0c;
        const MANAGER_GLOBAL: u32 = 0x0115_f848;
        const CALLBACK_GLOBAL: u32 = 0x0116_18fc;

        let state = (this as usize as *const u16).add(STATE_WORD / 2).read_unaligned();
        let handle_slot = (this as usize as *mut u32).add(HANDLE_DWORD / 4);
        if state == u16::MAX || handle_slot.read_unaligned() != 0 {
            return 0;
        }

        let mut record = [0u8; RECORD_SIZE];
        let record_ptr = record.as_mut_ptr() as usize as u32;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, record_ptr);

        let mut frame_pointer_placeholder = 0u32;
        let frame_pointer_placeholder = &mut frame_pointer_placeholder as *mut u32 as usize as u32;
        let resource_slot = (lf_checker_rt::global::<u32>(RESOURCE_SLOT_GLOBAL)).read_unaligned();
        let _ = lf_checker_rt::callee_cdecl!(2, u32, frame_pointer_placeholder, resource_slot);

        let record_value = (record.as_mut_ptr() as *mut u32).add(RECORD_VALUE / 4);
        record_value.write_unaligned(STARTING_TUNE_BITS);
        let _ = lf_checker_rt::callee_thiscall!(3, u32, this, lf_checker_rt::relocated(TUNE_RESOURCE), handle_slot as usize as u32, record_ptr, u32::MAX, 0, 0);

        if handle_slot.read_unaligned() != 0 {
            let _ = lf_checker_rt::callee_cdecl!(4, u32, lf_checker_rt::relocated(FOLLOWUP_RESOURCE), 1, u32::MAX);
            let submitted_handle = handle_slot.read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(5, u32, submitted_handle, 0, 1, u32::MAX);
            let manager = (lf_checker_rt::global::<u32>(MANAGER_GLOBAL)).read_unaligned();
            let callback = (lf_checker_rt::global::<u32>(CALLBACK_GLOBAL)).read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(6, u32, manager, callback);
        }
        0
    }
});
