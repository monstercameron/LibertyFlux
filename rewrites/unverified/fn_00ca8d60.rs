// original: 0x00CA8D60 CEventHandler::vf73

/// Refresh the pending task from a flagged owner's event fields.
///
/// When the owner (`this+0x04`) is null the handler keeps its task and
/// returns zero; when the owner's flag byte at `+0x26C` lacks bit 2 it keeps
/// its task and returns the owner. Otherwise the task source is asked: a
/// null source, or a null answer, clears the pending-task slot at `this+0x0C`,
/// and a live answer is turned into a task from the event's subject
/// (`+0x0C`), kind (`+0x10`) and float (`+0x18`, bitwise) words and stored
/// there. Returns the stored task on those paths. The other two stack
/// arguments are not read.
///
/// Original: 0x00CA8D60 (thiscall, three stack words, only the first read).
lf_checker_rt::export!(thiscall, rw_00ca8d60(this: u32, event: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const SOURCE: u32 = 1;
        const WORKER: u32 = 2;
        const OWNER: u32 = 0x04;
        const OWNER_FLAGS: u32 = 0x26C;
        const REFRESH_FLAG: u8 = 4;
        const EV_SUBJECT: u32 = 0x0C;
        const EV_KIND: u32 = 0x10;
        const EV_FLOAT: u32 = 0x18;
        const PENDING: u32 = 0x0C;
        const SOURCE_GLOBAL: u32 = 0x0167E2A0;
        let owner = (this.wrapping_add(OWNER) as *const u32).read_unaligned();
        if owner == 0 {
            return 0;
        }
        let flags = (owner.wrapping_add(OWNER_FLAGS) as *const u8).read();
        if flags & REFRESH_FLAG == 0 {
            return owner;
        }
        let source = lf_checker_rt::global::<u32>(SOURCE_GLOBAL).read_unaligned();
        let handle = lf_checker_rt::callee_thiscall!(SOURCE, u32, source);
        if handle == 0 {
            (this.wrapping_add(PENDING) as *mut u32).write_unaligned(0);
            return 0;
        }
        let subject = (event.wrapping_add(EV_SUBJECT) as *const u32).read_unaligned();
        let kind = (event.wrapping_add(EV_KIND) as *const u32).read_unaligned();
        let fbits = (event.wrapping_add(EV_FLOAT) as *const u32).read_unaligned();
        let task = lf_checker_rt::callee_thiscall!(WORKER, u32, handle, subject, kind, fbits, 0);
        (this.wrapping_add(PENDING) as *mut u32).write_unaligned(task);
        task
    }
});
