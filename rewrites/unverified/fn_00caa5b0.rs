// original: 0x00CAA5B0 CEventHandler::vf28

/// Refresh the pending task unless the owner is flagged or has no subject.
///
/// When the owner's (`this+0x04`) flag byte at `+0x26C` has bit 2 set, the
/// handler keeps its task and returns the owner. Otherwise reads the subject
/// at `event+0x0C` (first stack argument); a null subject also keeps the
/// task and returns the event. Otherwise the task source is asked: a null
/// source, or a null answer, clears the pending-task slot at `this+0x0C`,
/// and a live answer is turned into a task for the subject and stored there.
/// Returns the stored task on those paths. The other stack arguments are not
/// read.
///
/// Original: 0x00CAA5B0 (thiscall, three stack words, only the first read).
lf_checker_rt::export!(thiscall, rw_00caa5b0(this: u32, event: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const SOURCE: u32 = 1;
        const WORKER: u32 = 2;
        const OWNER: u32 = 0x04;
        const OWNER_FLAGS: u32 = 0x26C;
        const KEEP_FLAG: u8 = 4;
        const EV_SUBJECT: u32 = 0x0C;
        const PENDING: u32 = 0x0C;
        const SOURCE_GLOBAL: u32 = 0x0167E2A0;
        let owner = (this.wrapping_add(OWNER) as *const u32).read_unaligned();
        let flags = (owner.wrapping_add(OWNER_FLAGS) as *const u8).read();
        if flags & KEEP_FLAG != 0 {
            return owner;
        }
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
        let task = lf_checker_rt::callee_thiscall!(WORKER, u32, handle, subject);
        (this.wrapping_add(PENDING) as *mut u32).write_unaligned(task);
        task
    }
});
