// original: 0x00953a90 sync_live_flag
/// Reconcile the live flag with the sampler through the notifier chain.
///
/// When the override byte is set the sampler is forced on or off directly.
/// Otherwise the stored live word and the flag word are compared; on a
/// mismatch the sampler is re-armed and, when it reports idle, the live word
/// clears and the notifier chain selects its event id by the updater answer.
/// No return value is defined; the low byte observed by callers is incidental.
export!(cdecl, rw_00953a90() -> u32 {
    unsafe {
        if *global::<u8>(0x11F6953) != 0 {
            if *global::<u32>(0x11F6FA8) != 0 {
                return 0;
            }
            let _: u32 = callee_cdecl!(0, u32, 1, 0);
            return 0;
        }
        let live = *global::<u32>(0x1160EB8);
        if (live != 0) == (*global::<u32>(0x11F6FA8) != 0) {
            return 0;
        }
        if live == 0 {
            let _: u32 = callee_cdecl!(0, u32, 0, 0);
            return 0;
        }
        let answer: u32 = callee_cdecl!(0, u32, 1, 0);
        if answer & 0xFF != 0 {
            return 0;
        }
        *global::<u32>(0x1160EB8) = 0;
        if *global::<u8>(0x1160C3D) != 0 {
            return 0;
        }
        let updated: u32 = callee_cdecl!(1, u32,);
        if updated & 0xFF != 0 {
            let _: u32 = callee_cdecl!(2, u32, 0x2C);
        } else {
            let _: u32 = callee_cdecl!(2, u32, 0x2B);
        }
        0
    }
});
