// original: 0x00951F80 sweep_slot_table (proposed)

/// Sweep a 17-slot handle table, releasing each live entry twice.
///
/// Unless slot 0 is already null (which skips straight to the epilogue),
/// visits the 17 dword slots at `SLOTS`. A null slot does nothing. A live
/// slot calls its table's slot-0 function (callee 1, thiscall with the
/// handle in ECX and 1 on the stack; the contract plants the stub address
/// in the shared slot and the rewrite calls the stub directly), then
/// releases the slot's value word at `VALS` through callee 2 (one stack
/// word) and zeroes both words. The first 11 iterations (signed index below
/// 0) also zero an aux word at `AUX` and a flag byte at `FLAGS` + index;
/// the last 6 zero a dword and a byte through two running pointers instead.
/// The defensive abort past index 5 is unreachable (the loop bound caps the
/// index at 5) and is omitted. The epilogue copies `AUX`[0] to its shadow,
/// copies one word, and clears three bytes. No return channel.
///
/// Original: 0x00951F80 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00951F80() -> u32 {
    unsafe {
        const SLOTS: u32 = 0x11F6FA8;
        const VALS: u32 = 0x11F6F38;
        const AUX: u32 = 0x11F6F7C;
        const FLAGS: u32 = 0x11F6FFB;
        const SHADOW: u32 = 0x120F294;
        const SHADOW_B: u32 = 0x120F298;
        const OUT_B: u32 = 0x11FA010;
        const OUT_W: u32 = 0x11FA00C;
        const SRC_W: u32 = 0x11F7000;
        const CNT_A: u32 = 0x11F6FFB;
        const CNT_B: u32 = 0x11F6FFD;
        const SLOT_FN: u32 = 1;
        const RELEASE: u32 = 2;
        if (lf_checker_rt::relocated(SLOTS) as *const u32).read() != 0 {
            let mut ebp = lf_checker_rt::relocated(0x11F6FD4);
            let mut ebx = lf_checker_rt::relocated(0x11F700D);
            let mut i = 0u32;
            let mut s = -11i32;
            while s + 11 < 17 {
                let slot =
                    lf_checker_rt::relocated(SLOTS).wrapping_add(i.wrapping_mul(4));
                let h = (slot as *const u32).read();
                if h != 0 {
                    lf_checker_rt::callee_thiscall!(SLOT_FN, u32, h, 1);
                    let vslot = lf_checker_rt::relocated(VALS).wrapping_add(i.wrapping_mul(4));
                    let v = (vslot as *const u32).read();
                    (slot as *mut u32).write(0);
                    lf_checker_rt::callee_cdecl!(RELEASE, u32, v);
                    (vslot as *mut u32).write(0);
                    if s < 0 {
                        (lf_checker_rt::relocated(AUX).wrapping_add(i.wrapping_mul(4))
                            as *mut u32)
                            .write(0);
                        (lf_checker_rt::relocated(FLAGS).wrapping_add(s as u32) as *mut u8)
                            .write(0);
                    } else {
                        (ebp as *mut u32).write(0);
                        (ebx as *mut u8).write(0);
                    }
                }
                s += 1;
                i += 1;
                ebp = ebp.wrapping_add(4);
                ebx = ebx.wrapping_add(1);
            }
        }
        let a = (lf_checker_rt::relocated(AUX) as *const u32).read();
        (lf_checker_rt::global::<u32>(SHADOW) as *mut u32).write(a);
        (lf_checker_rt::global::<u8>(SHADOW_B) as *mut u8).write(0);
        (lf_checker_rt::global::<u8>(OUT_B) as *mut u8).write(0);
        let b = (lf_checker_rt::global::<u32>(SRC_W) as *const u32).read();
        (lf_checker_rt::global::<u32>(OUT_W) as *mut u32).write(b);
        (lf_checker_rt::global::<u8>(CNT_A) as *mut u8).write(0);
        (lf_checker_rt::global::<u8>(CNT_B) as *mut u8).write(0);
        0
    }
});
