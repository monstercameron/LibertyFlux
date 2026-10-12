// original: 0x00bdff90 CTaskSimpleWaitForBus::~CTaskSimpleWaitForBus__deleting
/// Apply the base task destruction and conditionally notify the script context. `this` is the task pointer in ECX; `flags` is the one caller-cleaned word, and only bit 0 is tested. The base destructor is called first. When bit 0 is set, the script-context pointer is loaded from the global slot and the task is passed as the single stack argument. The function returns the original task pointer. Calling convention: thiscall with one 32-bit stack word.
lf_checker_rt::export!(thiscall, rw_00bdff90(this: u32, flags: u32) -> u32 {
    const SCRIPT_CONTEXT_POINTER_VA: u32 = 0x0167_E2A0;
    const FLAG_REQUESTS_SCRIPT_CLEANUP: u32 = 0x01;
    const BASE_DESTRUCTOR: u32 = 1;
    const SIGNAL_SCRIPT_CONTEXT: u32 = 2;
    let _base_result = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
    if flags & FLAG_REQUESTS_SCRIPT_CLEANUP != 0 {
        let script_context = unsafe { lf_checker_rt::global::<u32>(SCRIPT_CONTEXT_POINTER_VA).read_unaligned() };
        let _signal_result = lf_checker_rt::callee_thiscall!(SIGNAL_SCRIPT_CONTEXT, u32, script_context, this);
    }
    this
});
