// original: 0x00cf9eb0 call_task_a80_with_zero
/// Loads the receiver pointer from the input object's field at byte offset 0xA80,
/// then forwards one zero option to the receiver's task method and returns its result.
/// The wrapper is stdcall with one pointer argument; the nested method is thiscall.
lf_checker_rt::export!(stdcall, rw_00cf9eb0(input: *const u8) -> u32 {
    unsafe {
        const RECEIVER_FIELD: usize = 0xA80;
        let receiver = *((input.add(RECEIVER_FIELD)) as *const u32);
        lf_checker_rt::callee_thiscall!(1, u32, receiver, 0)
    }
});
