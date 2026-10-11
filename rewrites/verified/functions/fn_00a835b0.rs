// original: 0x00a835b0 CAnimTaskInfo::vf7

/// Build and log an animation-task report using the live animation manager. The manager pointer is read from the global slot; fields at +0x14 and +0x18 and two local output words are passed to its thiscall helper. The helper's first output word is logged, followed by the byte at +0x28, the f32 at +0x24 converted to an f64 stack pair, and the word at +0x1c. The caller stack-frame pointers are skipped in call comparison; their initial contents are snapshotted, the helper's output writes are scripted, and the resulting value is observed in a later logger argument.
lf_checker_rt::export!(thiscall, rw_00a835b0(this: u32, logger_context: u32) -> () {
        const ANIMATION_MANAGER_GLOBAL: u32 = 0x016DD63C;
        const REPORT_VALUE_FORMAT: u32 = 0x00EA04BC;
        const REPORT_STATE_FORMAT: u32 = 0x00EA04CC;
        const REPORT_AMOUNT_FORMAT: u32 = 0x00EA04DC;
        const REPORT_FLAGS_FORMAT: u32 = 0x00EA04EC;
        const INITIAL_REPORT_VALUE: u32 = 0x00EA04AC;
        const INITIAL_REPORT_LABEL: u32 = 0x00EA049C;

        let mut report_value = lf_checker_rt::relocated(INITIAL_REPORT_VALUE);
        let mut report_label = lf_checker_rt::relocated(INITIAL_REPORT_LABEL);
        let manager = unsafe {
            (lf_checker_rt::relocated(ANIMATION_MANAGER_GLOBAL) as *const u32).read_unaligned()
        };
        let _ = lf_checker_rt::callee_thiscall!(
            1, u32, manager,
            unsafe { (this.wrapping_add(0x14) as *const u32).read_unaligned() },
            unsafe { (this.wrapping_add(0x18) as *const u32).read_unaligned() },
            (&mut report_label as *mut u32) as u32,
            (&mut report_value as *mut u32) as u32
        );
        let _ = lf_checker_rt::callee_cdecl!(
            2, u32, logger_context, 1u32, 1u32,
            lf_checker_rt::relocated(REPORT_VALUE_FORMAT), report_value
        );
        let _ = lf_checker_rt::callee_cdecl!(
            2, u32, logger_context, 1u32, 1u32,
            lf_checker_rt::relocated(REPORT_STATE_FORMAT),
            unsafe { (this.wrapping_add(0x28) as *const u8).read() as u32 }
        );

        let amount = core::hint::black_box(unsafe {
            f32::from_bits((this.wrapping_add(0x24) as *const u32).read_unaligned())
        });
        let amount_bits = (amount as f64).to_bits();
        let _ = lf_checker_rt::callee_cdecl!(
            3, u32, logger_context, 1u32, 1u32,
            lf_checker_rt::relocated(REPORT_AMOUNT_FORMAT),
            amount_bits as u32, (amount_bits >> 32) as u32
        );
        let _ = lf_checker_rt::callee_cdecl!(
            2, u32, logger_context, 1u32, 1u32,
            lf_checker_rt::relocated(REPORT_FLAGS_FORMAT),
            unsafe { (this.wrapping_add(0x1c) as *const u32).read_unaligned() }
        );
});
