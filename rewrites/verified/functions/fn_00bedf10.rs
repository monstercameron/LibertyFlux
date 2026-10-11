// original: 0x00BEDF10 build_task_resource_snapshot

/// Build a task resource snapshot in `this` from `source`. A four-word local
/// buffer is zero-initialized, passed first to a thiscall helper with `source`
/// as its object, then passed to a cdecl helper that returns a 64-bit value.
/// The result's low and high dwords go to `this + 0x38` and `this + 0x3C`;
/// source dwords at `+0x30`, `+0x34`, and `+0x38` are copied to `this + 0x2C`,
/// `+0x30`, and `+0x34`. The function returns the last copied source dword.
/// The original is a thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00BEDF10(this: u32, source: u32) -> u32 {
    const LOCAL_WORDS: usize = 4;
    const RESOURCE_INPUTS: [u32; 3] = [0x30, 0x34, 0x38];
    const RESOURCE_OUTPUTS: [u32; 3] = [0x2C, 0x30, 0x34];
    const RESULT_LOW: u32 = 0x38;
    const RESULT_HIGH: u32 = 0x3C;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let mut local_result = [0u32; LOCAL_WORDS];
        let local_pointer = local_result.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, source, local_pointer);
        let result = lf_checker_rt::callee_cdecl!(2, u64, local_pointer);
        write_u32(this + RESULT_LOW, result as u32);
        write_u32(this + RESULT_HIGH, (result >> 32) as u32);

        let mut return_value = 0;
        for index in 0..RESOURCE_INPUTS.len() {
            return_value = read_u32(source + RESOURCE_INPUTS[index]);
            write_u32(this + RESOURCE_OUTPUTS[index], return_value);
        }
        return_value
    }
});
