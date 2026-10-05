// original: 0x008850a0 stream_counter_add_a (proposed)
/// Add to counter A and report the addition through the primary object.
///
/// Adds `amount` to the shared counter (global at file address `0x115a3fc`),
/// then reports (`amount`, `detail`, 0) to the primary object's slot-2
/// handler (object from the global at `0x115a3f4`, handler from its table at
/// `+0x08`; intercepted callee 1, thiscall: object in `ecx`, three stack
/// arguments).
///
/// Original: cdecl, two stack arguments, no return value.
lf_checker_rt::export!(cdecl, rw_008850a0(amount: u32, detail: u32) -> u32 {
    unsafe {
        const PRIMARY: u32 = 0x0115_a3f4;
        const COUNTER: u32 = 0x0115_a3fc;
        const HANDLER_SLOT: u32 = 0x08;
        let obj = (lf_checker_rt::global::<u32>(PRIMARY) as *const u32).read_unaligned();
        let counter = lf_checker_rt::global::<u32>(COUNTER) as *mut u32;
        counter.write_unaligned(counter.read_unaligned().wrapping_add(amount));
        let table = (obj as *const u32).read_unaligned();
        let handler = ((table + HANDLER_SLOT) as *const u32).read_unaligned();
        let report: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(handler as usize);
        report(obj, amount, detail, 0);
        0
    }
});
