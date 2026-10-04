// original: 0x00e6de30 call_store_global_00
/// Call the init helper and store its answer at 0x17ACC5C.
///
/// Invokes the helper (cdecl/2, stubbed by the checker) with the
/// table at 0xF14244 and a zero flag, stores the returned value and
/// returns it.
export!(cdecl, rw_00e6de30() -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32, relocated(0xF14244), 0);
        *global::<u32>(0x17ACC5C) = answer;
        answer
    }
});
