// original: 0x005D6E60 session_open_probe_dispatched (proposed)

/// Opens a session object through the thread allocator, probes it, and
/// links the result into a global slot.
///
/// `this` is the owner; the stack arguments are a key block `arg0` and a
/// flags word `arg1`. The session state lives in TLS slot 0 (`tls0`).
/// A first object is allocated through the allocator at `[tls0+8]` as
/// `tls[0] -> [+8] -> vtable[+8]` (thiscall: allocator, `0x14, 0x10, 0`;
/// on trials where the earlier swap installed the spare allocator the
/// twin stub answers instead) and zeroed over its first 19 bytes with a
/// read-modify-write of the word at `+0x10` (low half cleared, then the
/// high half). The allocator object is kept live in ECX across that call
/// (the stub preserves it, as the caller requires) and registered with
/// callee 3 (thiscall: allocator, allocator); the registration answer
/// points at the word that receives the fresh object. Callee 4 (thiscall:
/// fresh object, key block) attaches it; both answers are ignored.
///
/// A counter at `[tls0+0x68]` is then decremented, or, when zero, the
/// spare allocator at `[tls0+0x64]` is installed at `[tls0+8]` and the
/// spare slot cleared. Four probe calls (callees 5-8, cdecl: key word,
/// table constant) run in short-circuit order and pick the second
/// allocation size and its follow-up: first probe 0 -> size `0x104` with
/// callee 9; any later probe 0 -> size `0xF0` with callee 11; all
/// non-zero -> size `0xE8` with callee 10. The second allocation repeats
/// the allocator call with `(size, 0x10, 0)`; a null answer clears the
/// object, otherwise the follow-up (thiscall: second object, first
/// object) replaces it. Only zero/non-zero of the probe answers matters
/// (all are `test`/`je` branches, no ordered comparison).
///
/// When the global link slot (file VA `LINK_VA`) is null the object is
/// stored at `[this]`; otherwise callee 12 (thiscall: link slot + 12,
/// last live ECX) runs, its answer points at the word receiving the
/// object, and the object links back at `+8` (a null object faults on
/// both sides here). A zero low byte of `arg1` selects the object over
/// the old link (`cmove`); the winner is stored to the global slot.
/// Finally `[tls0+8]` and `[tls0+0x10]` are compared for equality: equal
/// increments the counter, otherwise the first word rotates into the
/// spare slot and the second becomes current. Returns the current
/// allocator word `[tls0+8]` (reloaded into EAX for the final compare,
/// discarding the last callee answer).
///
/// Original: 0x005D6E60 (thiscall, two stack words, callee pops 8).
lf_checker_rt::export!(thiscall, rw_005D6E60(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const HEAPOBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const LINK_VA: u32 = 0x018B6FAC;
        const PROBE0_VA: u32 = 0x00F907D0;
        const PROBE1_VA: u32 = 0x00F907E4;
        const PROBE2_VA: u32 = 0x00F907E0;
        const PROBE3_VA: u32 = 0x00F907BC;
        const REGISTER: u32 = 3;
        const ATTACH: u32 = 4;
        const DFA: u32 = 5;
        const DFB: u32 = 6;
        const DFC: u32 = 7;
        const DFD: u32 = 8;
        const F1: u32 = 9;
        const F2: u32 = 10;
        const F3: u32 = 11;
        const LINK: u32 = 12;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        let tls0 = lf_checker_rt::tls_slot(TLS_SLOT);
        let heap_obj = rd32(tls0.wrapping_add(HEAPOBJ_OFF));
        let vtable = rd32(heap_obj);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable.wrapping_add(ALLOC_SLOT)) as usize) };
        let mut edi = alloc(heap_obj, 0x14, 0x10, 0);
        if edi != 0 {
            wr32(edi, 0);
            wr8(edi.wrapping_add(4), 0);
            wr32(edi.wrapping_add(8), 0);
            wr32(edi.wrapping_add(0xC), 0);
            wr32(
                edi.wrapping_add(0x10),
                rd32(edi.wrapping_add(0x10)) & 0xFFFF0000,
            );
            wr16(edi.wrapping_add(0x12), 0);
        }
        let reg = lf_checker_rt::callee_thiscall!(REGISTER, u32, heap_obj, heap_obj);
        wr32(reg, edi);
        let _ = lf_checker_rt::callee_thiscall!(ATTACH, u32, edi, arg0);
        let rc = rd32(tls0.wrapping_add(0x68));
        if rc != 0 {
            wr32(tls0.wrapping_add(0x68), rc.wrapping_sub(1));
        } else {
            let spare = rd32(tls0.wrapping_add(0x64));
            wr32(tls0.wrapping_add(8), spare);
            wr32(tls0.wrapping_add(0x64), 0);
        }
        let key = rd32(arg0);
        let heap_now = rd32(tls0.wrapping_add(8));
        let a0 = lf_checker_rt::callee_cdecl!(DFA, u32, key, lf_checker_rt::relocated(PROBE0_VA));
        let (size, fid) = if a0 == 0 {
            (0x104u32, F1)
        } else {
            let a1 =
                lf_checker_rt::callee_cdecl!(DFB, u32, key, lf_checker_rt::relocated(PROBE1_VA));
            if a1 == 0 {
                (0xF0u32, F3)
            } else {
                let a2 = lf_checker_rt::callee_cdecl!(
                    DFC,
                    u32,
                    key,
                    lf_checker_rt::relocated(PROBE2_VA)
                );
                if a2 == 0 {
                    (0xF0u32, F3)
                } else {
                    let a3 = lf_checker_rt::callee_cdecl!(
                        DFD,
                        u32,
                        key,
                        lf_checker_rt::relocated(PROBE3_VA)
                    );
                    if a3 == 0 {
                        (0xF0u32, F3)
                    } else {
                        (0xE8u32, F2)
                    }
                }
            }
        };
        let vt2 = rd32(heap_now);
        let alloc2: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt2.wrapping_add(ALLOC_SLOT)) as usize) };
        let r = alloc2(heap_now, size, 0x10, 0);
        // last_ecx mirrors the caller's live ECX: the stub preserves each
        // callee's entry ECX, so the value pushed later is the entry ECX of
        // whichever call ran last (the follow-up's answer object, or the
        // allocator object when the second allocation answered null).
        let last_ecx: u32;
        if r != 0 {
            let fr = lf_checker_rt::callee_thiscall!(fid, u32, r, edi);
            edi = fr;
            last_ecx = r;
        } else {
            edi = 0;
            last_ecx = heap_now;
        }
        let mut ebx = rd32(lf_checker_rt::relocated(LINK_VA));
        if ebx == 0 {
            wr32(this, edi);
        } else {
            let lr = lf_checker_rt::callee_thiscall!(LINK, u32, ebx.wrapping_add(0xC), last_ecx);
            wr32(lr, edi);
            wr32(edi.wrapping_add(8), ebx);
            ebx = rd32(lf_checker_rt::relocated(LINK_VA));
        }
        if (arg1 as u8) == 0 {
            ebx = edi;
        }
        wr32(lf_checker_rt::relocated(LINK_VA), ebx);
        let ea = rd32(tls0.wrapping_add(8));
        let ec = rd32(tls0.wrapping_add(0x10));
        if ea == ec {
            let c = rd32(tls0.wrapping_add(0x68));
            wr32(tls0.wrapping_add(0x68), c.wrapping_add(1));
        } else {
            wr32(tls0.wrapping_add(0x64), ea);
            wr32(tls0.wrapping_add(8), ec);
        }
        ea
    }
});
