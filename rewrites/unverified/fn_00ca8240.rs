// original: 0x00CA8240 CEventHandler::vf23

/// Refresh the pending task from the event's subject, if there is one.
///
/// Reads the subject pointer at `event+0x0C` (first stack argument). A null
/// subject leaves the handler untouched and returns the event itself (the
/// original's eax). Otherwise the task source is asked for a task: a null
/// source, or a null answer, clears the pending-task slot at `this+0x0C`,
/// and a live answer is turned into a task for the subject and stored there.
/// Returns the stored task on those paths. The other stack arguments are not
/// read.
///
/// Original: 0x00CA8240 (thiscall, three stack words, only the first read).
lf_checker_rt::export!(thiscall, rw_00ca8240(this: u32, event: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const SOURCE: u32 = 1;
        const WORKER: u32 = 2;
        const EV_SUBJECT: u32 = 0x0C;
        const PENDING: u32 = 0x0C;
        const SOURCE_GLOBAL: u32 = 0x0167E2A0;
        let subject = (event.wrapping_add(EV_SUBJECT) as *const u32).read_unaligned();
        if subject == 0 {
            return event;
        }
        let source = lf_checker_rt::global::<u32>(SOURCE_GLOBAL).read_unaligned();
        let handle = lf_checker_rt::callee_thiscall!(SOURCE, u32, source);
        if handle == 0 {
            (this.wrapping_add(PENDING) as *mut u32).write_unaligned(0);
            return 0;
        }
        let task = lf_checker_rt::callee_thiscall!(WORKER, u32, handle, subject, 0);
        (this.wrapping_add(PENDING) as *mut u32).write_unaligned(task);
        task
    }
});
