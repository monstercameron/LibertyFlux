// original: 0x00CA7BC0 CEventHandler::vf35

/// Route an event by owner check and kind: clear, build, or dispatch.
///
/// Polls slot 0x34 of the event's (first stack argument) virtual table: a
/// null first answer, or a second answer equal to the handler's owner at
/// `this+0x04`, ends the call with no store. Otherwise the kind word at
/// `event+0x10` decides: kinds 0xC8 and 0x3A7 clear the pending-task slot at
/// `this+0x0C`; kind 0x398 asks the task source for a task built from the
/// event's `+0x20` block and two global floats (a null source or answer
/// clears the slot instead); any other kind is forwarded to the handler's
/// own virtual slot 0x130 with (kind, event+0x20) and stores nothing. No
/// return value; the other two stack arguments are not read.
///
/// Original: 0x00CA7BC0 (thiscall, three stack words, only the first read).
lf_checker_rt::export!(thiscall, rw_00ca7bc0(this: u32, event: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const SOURCE: u32 = 3;
        const BUILDER: u32 = 4;
        const OWNER: u32 = 0x04;
        const PENDING: u32 = 0x0C;
        const EV_KIND: u32 = 0x10;
        const EV_BLOCK: u32 = 0x20;
        const POLL_SLOT: u32 = 0x34;
        const DISPATCH_SLOT: u32 = 0x130;
        const CLEAR_A: u32 = 0xC8;
        const BUILD_KIND: u32 = 0x398;
        const CLEAR_B: u32 = 0x3A7;
        const SOURCE_GLOBAL: u32 = 0x0167E2A0;
        const FLOAT_A_GLOBAL: u32 = 0x00ED7E70;
        const FLOAT_B_GLOBAL: u32 = 0x00ED7E68;
        unsafe fn poll(event: u32) -> u32 {
            unsafe {
                let vtable = (event as *const u32).read_unaligned();
                let slot = (vtable.wrapping_add(POLL_SLOT) as *const u32).read_unaligned();
                let get: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                get(event)
            }
        }
        if poll(event) == 0 {
            return 0;
        }
        let owner = (this.wrapping_add(OWNER) as *const u32).read_unaligned();
        if poll(event) == owner {
            return 0;
        }
        let kind = (event.wrapping_add(EV_KIND) as *const u32).read_unaligned();
        if kind == CLEAR_A || kind == CLEAR_B {
            (this.wrapping_add(PENDING) as *mut u32).write_unaligned(0);
            return 0;
        }
        if kind == BUILD_KIND {
            let source = lf_checker_rt::global::<u32>(SOURCE_GLOBAL).read_unaligned();
            let handle = lf_checker_rt::callee_thiscall!(SOURCE, u32, source);
            if handle == 0 {
                (this.wrapping_add(PENDING) as *mut u32).write_unaligned(0);
                return 0;
            }
            let fa = lf_checker_rt::global::<u32>(FLOAT_A_GLOBAL).read_unaligned();
            let fb = lf_checker_rt::global::<u32>(FLOAT_B_GLOBAL).read_unaligned();
            let task = lf_checker_rt::callee_thiscall!(
                BUILDER, u32, handle, event.wrapping_add(EV_BLOCK), fb, fa);
            (this.wrapping_add(PENDING) as *mut u32).write_unaligned(task);
            return 0;
        }
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(DISPATCH_SLOT) as *const u32).read_unaligned();
        let dispatch: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        dispatch(this, kind, event.wrapping_add(EV_BLOCK));
        0
    }
});
