// original: 0x00b4fde0 swap_task_pair (proposed)

/// Swap the dummy ped's current task, releasing the displaced one.
///
/// The current task at `this + 0x128`, when present, is asked to yield
/// through its virtual slot at `+0x14` (thiscall on the task with `this`,
/// `flag` and 0). A refusal (zero low byte) ends the call: with a null
/// new task nothing more happens, otherwise the incoming new task is
/// released (virtual slot `+0x00`, thiscall on it with 1); either way 0
/// is returned.
/// On acceptance with `flag` other than 1, or when the kind query (virtual
/// slot `+0x04` on the current task) is non-zero, the current task is
/// released the same way. Otherwise the old task at `this + 0x12C` is
/// released if present, the current task takes its place, a pending
/// callback (word at `+0x18` of the new old task) runs through virtual
/// slot `+0xD0` and the adapter (callee 6, thiscall on that result) when
/// non-null, and the new old task is finalised (callee 7, thiscall on it
/// with 0). Every path that installs `newtask` at `this + 0x128` returns 1.
///
/// Original: 0x00b4fde0 (thiscall, two stack words; boolean in AL).
lf_checker_rt::export!(thiscall, rw_00b4fde0(this: u32, newtask: u32, flag: u32) -> u32 {
    unsafe {
        const CUR: u32 = 0x128;
        const OLD: u32 = 0x12c;
        const YIELD_SLOT: u32 = 0x14;
        const RELEASE_SLOT: u32 = 0x00;
        const KIND_SLOT: u32 = 0x04;
        const CALLBACK: u32 = 0x18;
        const NOTIFY_SLOT: u32 = 0xd0;
        const RELEASE_ARG: u32 = 1;
        const ADAPT: u32 = 6;
        const FINALISE: u32 = 7;
        let cur = ((this + CUR) as *const u32).read_unaligned();
        if cur != 0 {
            let vt = (cur as *const u32).read_unaligned();
            let yield_: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(
                ((vt + YIELD_SLOT) as *const u32).read_unaligned() as usize,
            );
            if yield_(cur, this, flag, 0) & 0xff == 0 {
                if newtask != 0 {
                    let nvt = (newtask as *const u32).read_unaligned();
                    let rel: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        ((nvt + RELEASE_SLOT) as *const u32).read_unaligned() as usize,
                    );
                    rel(newtask, RELEASE_ARG);
                }
                return 0;
            }
            let vt = (cur as *const u32).read_unaligned();
            let kind: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + KIND_SLOT) as *const u32).read_unaligned() as usize,
            );
            if flag != 1 || kind(cur) != 0 {
                let vt = (cur as *const u32).read_unaligned();
                let rel: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    ((vt + RELEASE_SLOT) as *const u32).read_unaligned() as usize,
                );
                rel(cur, RELEASE_ARG);
            } else {
                let old = ((this + OLD) as *const u32).read_unaligned();
                if old != 0 {
                    let ovt = (old as *const u32).read_unaligned();
                    let rel: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        ((ovt + RELEASE_SLOT) as *const u32).read_unaligned() as usize,
                    );
                    rel(old, RELEASE_ARG);
                }
                let cur2 = ((this + CUR) as *const u32).read_unaligned();
                ((this + OLD) as *mut u32).write_unaligned(cur2);
                let cb = ((cur2 + CALLBACK) as *const u32).read_unaligned();
                if cb != 0 {
                    let vt = (this as *const u32).read_unaligned();
                    let notify: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        ((vt + NOTIFY_SLOT) as *const u32).read_unaligned() as usize,
                    );
                    let h = notify(this, cb);
                    lf_checker_rt::callee_thiscall!(ADAPT, u32, h);
                }
                let fin = ((this + OLD) as *const u32).read_unaligned();
                lf_checker_rt::callee_thiscall!(FINALISE, u32, fin, 0);
            }
            ((this + CUR) as *mut u32).write_unaligned(0);
        }
        ((this + CUR) as *mut u32).write_unaligned(newtask);
        1
    }
});
