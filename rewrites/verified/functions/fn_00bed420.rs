// original: 0x00BED420 clear_task_secondary_state

/// Clear the embedded task's status byte and issue its virtual notification
/// unless the mode word equals two. Then inspect bit 27 of the source flags:
/// when set, return the shifted flag value; otherwise call two cdecl helpers
/// with the source pointer and zero, returning the second helper's result.
/// The embedded object begins at `source + 0x80`; its status byte is at
/// `+0x3C`, its flags are at `source + 0x24`, and the mode word is a relocated
/// global. The method is thiscall with two stack arguments.
lf_checker_rt::export!(thiscall, rw_00BED420(this: u32, source: u32, _unused: u32) -> u32 {
    const MODE_VA: u32 = 0x011F70D4;
    const EMBEDDED_TASK: u32 = 0x80;
    const STATUS_BYTE: u32 = 0x3C;
    const SOURCE_FLAGS: u32 = 0x24;
    const CALL_OBJECT_FIELD: u32 = 0x50;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        let embedded = source + EMBEDDED_TASK;
        if lf_checker_rt::global::<u32>(MODE_VA).read_unaligned() != 2 {
            write_u8(embedded + STATUS_BYTE, 0);
            let forwarded_field = read_u32(this + CALL_OBJECT_FIELD);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, embedded, 0, forwarded_field);
        }

        let shifted_flags = read_u32(source + SOURCE_FLAGS) >> 0x1B;
        if shifted_flags & 1 != 0 {
            shifted_flags
        } else {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, source, 0);
            lf_checker_rt::callee_cdecl!(3, u32, source, 0)
        }
    }
});
