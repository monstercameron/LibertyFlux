// original: 0x00CA8DB0 CEventHandler::vf21

/// Mark the event seen and refresh the pending task from its fields.
///
/// Sets the seen byte at `event+0x14` (first stack argument), then asks the
/// task source for a task: a null source, or a null answer, clears the
/// pending-task slot at `this+0x0C`, and a live answer is turned into a task
/// from the event's subject (`+0x0C`) and kind (`+0x10`) words and stored
/// there. Returns the stored task. The other stack arguments are not read.
///
/// Original: 0x00CA8DB0 (thiscall, three stack words, only the first read).
lf_checker_rt::export!(thiscall, rw_00ca8db0(this: u32, event: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const SOURCE: u32 = 1;
        const WORKER: u32 = 2;
        const EV_SUBJECT: u32 = 0x0C;
        const EV_KIND: u32 = 0x10;
        const EV_SEEN: u32 = 0x14;
        const PENDING: u32 = 0x0C;
        const SOURCE_GLOBAL: u32 = 0x0167E2A0;
        let kind = (event.wrapping_add(EV_KIND) as *const u32).read_unaligned();
        (event.wrapping_add(EV_SEEN) as *mut u8).write(1);
        let source = lf_checker_rt::global::<u32>(SOURCE_GLOBAL).read_unaligned();
        let subject = (event.wrapping_add(EV_SUBJECT) as *const u32).read_unaligned();
        let handle = lf_checker_rt::callee_thiscall!(SOURCE, u32, source);
        if handle == 0 {
            (this.wrapping_add(PENDING) as *mut u32).write_unaligned(0);
            return 0;
        }
        let task = lf_checker_rt::callee_thiscall!(WORKER, u32, handle, subject, kind);
        (this.wrapping_add(PENDING) as *mut u32).write_unaligned(task);
        task
    }
});
