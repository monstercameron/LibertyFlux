// original: 0x008850e0 stream_counter_sub (proposed)
/// Settle counter A through the primary object and report the settlement.
///
/// Asks the primary object (global at file address `0x115a3f4`) for its
/// settle value through its slot-6 handler (table at `+0x18`; intercepted
/// callee 1, thiscall: object in `ecx`, `key` as the stack argument),
/// subtracts the answer from the shared counter (global at `0x115a3fc`),
/// then reports `key` to the slot-3 handler (table at `+0x0c`; intercepted
/// callee 2, thiscall: object in `ecx`, `key` as the stack argument).
///
/// Original: cdecl, one stack argument, no return value.
lf_checker_rt::export!(cdecl, rw_008850e0(key: u32) -> u32 {
    unsafe {
        const PRIMARY: u32 = 0x0115_a3f4;
        const COUNTER: u32 = 0x0115_a3fc;
        const SETTLE_SLOT: u32 = 0x18;
        const REPORT_SLOT: u32 = 0x0c;
        let obj = (lf_checker_rt::global::<u32>(PRIMARY) as *const u32).read_unaligned();
        let table = (obj as *const u32).read_unaligned();
        let settle = ((table + SETTLE_SLOT) as *const u32).read_unaligned();
        let ask: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(settle as usize);
        let value = ask(obj, key);
        let counter = lf_checker_rt::global::<u32>(COUNTER) as *mut u32;
        counter.write_unaligned(counter.read_unaligned().wrapping_sub(value));
        let table = (obj as *const u32).read_unaligned();
        let handler = ((table + REPORT_SLOT) as *const u32).read_unaligned();
        let report: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(handler as usize);
        report(obj, key);
        0
    }
});
