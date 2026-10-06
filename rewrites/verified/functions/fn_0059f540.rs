// original: 0x0059F540 memcfg_release_tail (proposed)
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated, tls_slot};

/// Release the cached config head, refresh two objects, and tail into the
/// next stage.
///
/// Takes no arguments. When the flag byte `FLAG` is nonzero it is cleared
/// and the cached head `HEAD` is read: a nonzero head is first passed to the
/// prepare callee, then released through the thread-local object graph
/// (`fs:[0x2c]` -> object -> `+8` vtable-pointer -> target table `+0xC`,
/// thiscall with the head as its argument), and `HEAD` is cleared.
///
/// Then the ready callee is polled (thiscall on `OBJ_A` with `MAGIC`); only
/// its LOW BYTE matters. When set, the reset callee runs on `OBJ_B`, the
/// fetch callee runs on `OBJ_A`, a nonzero fetch answer is dispatched
/// through its own vtable slot `+8` (thiscall with argument 1), and `DONE`
/// is cleared.
///
/// Finally the drain callee runs and the function tail-jumps to the next
/// stage; the tail answer is the u32 return.
///
/// Original: 0x0059F540 (cdecl/0; the inventory size overcovers into the
/// next function, which starts with its own `(an instruction of the original)` after padding -- the
/// tail jump exits before reaching it).
export!(cdecl, rw_0059f540() -> u32 {
    unsafe {
        const FLAG: u32 = 0x0116_0C3A;
        const HEAD: u32 = 0x018B_6E84;
        const DONE: u32 = 0x018B_6C8C;
        const OBJ_A: u32 = 0x0198_1A4C;
        const OBJ_B: u32 = 0x0117_37D0;
        const MAGIC: u32 = 0x3FAA_283B;
        const PREPARE_CALLEE: u32 = 1;
        const TLS_RELEASE_CALLEE: u32 = 2;
        const READY_CALLEE: u32 = 3;
        const RESET_CALLEE: u32 = 4;
        const FETCH_CALLEE: u32 = 5;
        const DISPATCH_CALLEE: u32 = 6;
        const DRAIN_CALLEE: u32 = 7;
        const TAIL_CALLEE: u32 = 8;

        if global::<u8>(FLAG).read() != 0 {
            let head = global::<u32>(HEAD).read();
            global::<u8>(FLAG).write(0);
            if head != 0 {
                let _ = callee_thiscall!(PREPARE_CALLEE, u32, head);
                let p = tls_slot(0);
                let segb = ((p.wrapping_add(8)) as *const u32).read_unaligned();
                let tgtbase = (segb as *const u32).read_unaligned();
                let tgt = ((tgtbase.wrapping_add(0xC)) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                let _ = f(segb, head);
                global::<u32>(HEAD).write(0);
            }
        }
        let ga = callee_thiscall!(READY_CALLEE, u32, relocated(OBJ_A), MAGIC);
        if (ga & 0xFF) != 0 {
            let _ = callee_thiscall!(RESET_CALLEE, u32, relocated(OBJ_B), 0);
            let ans = callee_thiscall!(FETCH_CALLEE, u32, relocated(OBJ_A), MAGIC);
            if ans != 0 {
                let vt2 = (ans as *const u32).read_unaligned();
                let tgt2 = ((vt2.wrapping_add(8)) as *const u32).read_unaligned();
                let g: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt2 as usize);
                let _ = g(ans, 1);
            }
            global::<u32>(DONE).write(0);
        }
        let _ = callee_cdecl!(DRAIN_CALLEE, u32,);
        callee_cdecl!(TAIL_CALLEE, u32,)
    }
});
