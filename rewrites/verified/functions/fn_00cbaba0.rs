// original: 0x00cbaba0 ped_flag_gated_notify
/// Notify a ped when this task's flag bit is set (2 calls, leaf-like).
///
/// When bit 0 of the flag byte at `[this + 0x2C]` is clear, returns at
/// once (thiscall, one stack argument). Otherwise runs the ready-check
/// callee on the ped block at `a0 + 0xBB0`; when its low byte is non-zero
/// runs the notify callee on the same block with kind `0xFA`. Both
/// callees are intercepted and answered by the checker. No reads or
/// writes of its own and no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00cbaba0(this: u32, a0: u32) -> u32 {
    unsafe {
        /// Flag byte gating the notification.
        const FLAG_OFF: u32 = 0x2C;
        /// Ped block offset inside the argument object.
        const PED_OFF: u32 = 0xBB0;
        /// Notification kind passed to the second callee.
        const KIND: u32 = 0xFA;
        const READY: u32 = 1;
        const NOTIFY: u32 = 2;
        let f = ((this + FLAG_OFF) as *const u8).read();
        if f & 1 == 0 {
            return 0;
        }
        let ped = a0.wrapping_add(PED_OFF);
        let v: u32 = lf_checker_rt::callee_thiscall!(READY, u32, ped);
        if (v & 0xFF) == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, ped, KIND);
        0
    }
});
