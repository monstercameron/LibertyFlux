// original: 0x00DA64D0 task_dispatch_via_slot_d0 (proposed)

/// Shared dispatch helper: call virtual slot 0xD0 of the object given as
/// the stack argument and return its answer.
///
/// The object address arrives on the stack (not in ECX); the slot address
/// is loaded through the object's table exactly like the original, so both
/// sides land on the same intercepted callee. Original: one stack word,
/// callee cleanup.
lf_checker_rt::export!(stdcall, rw_00da64d0(obj: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 0xD0;

        let table = (obj as *const u32).read();
        let slot = ((table + SLOT_OFF) as *const u32).read();
        let target: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        target(obj)
    }
});
