// original: 0x00B65E60 veh_guarded_slot_rebind
/// Guarded rebind of the slot pair at `[this+0x14]` / `[this+0x18]`.
///
/// When `a0` chains fully (`a0`, `[a0+0x6c]`, its ready byte at +0xe) but the
/// flag at `[this+0x109]` is clear, returns `a0` untouched. Otherwise falls
/// into the shared tail: returns `a0` when `[this+0x18]` is null, else
/// detaches (stubbed, stdcall/1) and resets (stubbed, stdcall/1) the slot
/// address `this+0x14`, moves `[this+0x18]` to `[this+0x14]`, drops (stubbed,
/// stdcall/1) `this+0x18` and zeroes it. Invokes virtual slot 0x114
/// (planted stub, thiscall/1) with 8 on the moved object, notifies (stubbed,
/// stdcall/5) with `(a0,0,0x10b,0,0)`, and when `[obj+0x38]` holds an object
/// whose word at +8 is 0xffff invokes virtual slot 0xac (planted stub,
/// thiscall/0). Finishes with the virtual tail slot 0x30 (planted stub,
/// thiscall/1) with argument 1 and returns its answer; the original
/// overwrites its incoming argument slot with 1 and jumps, the rewrite calls
/// with 1 and returns. Thiscall, one stack word.

export!(thiscall, rw_00b65e60(this: u32, a0: u32) -> u32 {
    unsafe {
        const GUARD: u32 = 0x6c;
        const READY: u32 = 0x0e;
        const FLAG: u32 = 0x109;
        const SLOT14: u32 = 0x14;
        const SLOT18: u32 = 0x18;
        const AUX: u32 = 0x38;
        const TAG: u32 = 8;
        const MAGIC: u16 = 0xffff;
        const NOTIFY: u32 = 0x10b;
        const V_RELEASE: u32 = 0x114;
        const V_MID: u32 = 0xac;
        const V_TAIL: u32 = 0x30;
        if a0 != 0 {
            let n = ((a0 + GUARD) as *const u32).read_unaligned();
            if n != 0 && ((n + READY) as *const u8).read() != 0 {
                if ((this + FLAG) as *const u8).read() == 0 {
                    return a0;
                }
            }
        }
        if ((this + SLOT18) as *const u32).read_unaligned() == 0 {
            return a0;
        }
        let p14 = this + SLOT14;
        let p18 = this + SLOT18;
        let _: u32 = callee_stdcall!(1, u32, p14);
        let _: u32 = callee_stdcall!(2, u32, p14);
        (p14 as *mut u32).write_unaligned((p18 as *const u32).read_unaligned());
        if (p18 as *const u32).read_unaligned() != 0 {
            let _: u32 = callee_stdcall!(3, u32, p18);
        }
        (p18 as *mut u32).write_unaligned(0);
        let obj = (p14 as *const u32).read_unaligned();
        let vt = (obj as *const u32).read_unaligned();
        let rel: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((vt + V_RELEASE) as *const u32).read_unaligned()) as usize);
        let _ = rel(obj, 8);
        let _: u32 = callee_stdcall!(5, u32, a0, 0, NOTIFY, 0, 0);
        let aux = ((obj + AUX) as *const u32).read_unaligned();
        if aux != 0 && ((aux + TAG) as *const u16).read_unaligned() == MAGIC {
            let mid: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute((((vt + V_MID) as *const u32).read_unaligned()) as usize);
            let _ = mid(obj);
        }
        let tail: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((vt + V_TAIL) as *const u32).read_unaligned()) as usize);
        tail(obj, 1)
    }
});

