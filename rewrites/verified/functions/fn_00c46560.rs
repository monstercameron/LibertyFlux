// original: 0x00c46560 CCamFinal::vf5 (symbols)
/// Select the final camera's target and refresh the shared snapshot.
///
/// Searches the shared target table (`TABLE_GLOBAL`, `COUNT_GLOBAL`
/// entries) for the entry whose id word at `ID_OFF` matches the wanted
/// id at `this + WANT_ID`; when none matches, or the table is empty,
/// returns 0. Otherwise checks the candidate through its vtable slot
/// `0x14` (callee 1): on refusal, or when any of the three floats at
/// `this + FLOATS` is not exactly zero (the original compares each
/// against 0.0 with `ucomiss`, folds the zero and parity flags with
/// `(an instruction of the original)` and jumps on parity, which is taken exactly for
/// less, greater or unordered, i.e. nonzero or NaN; both signed zeros
/// count as zero), runs the refresh below; when all three are zero
/// returns 1 at once. The refresh
/// runs the member update (callee 2), then proceeds unless one of the
/// shared gates says otherwise (`GATE_A` set, the tick pair
/// `TICK_A`/`TICK_B` disagreeing, or `GATE_B` holding `GATE_B_SKIP`):
/// it takes the shared snapshot block (`SNAP_GLOBAL`), allocating it
/// (callee 3, cdecl) when null, copies this camera's 128-byte block at
/// `SNAP_SRC_OFF` into it (callee 4), and clears the dirty byte at
/// `DIRTY`. Either way it then runs the tail update (callee 5) and
/// returns 1 in the low byte.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c46560(this: u32) -> u32 {
    const COUNT_GLOBAL: u32 = 0x0118d7f4;
    const TABLE_GLOBAL: u32 = 0x0118d7f0;
    const ID_OFF: u32 = 0x53c;
    const WANT_ID: u32 = 0x1b0;
    const TARGET: u32 = 0x1ac;
    const VT_CHECK: u32 = 0x14;
    const FLOATS: [u32; 3] = [0x40, 0x44, 0x48];
    const GATE_A: u32 = 0x011f7060;
    const TICK_A: u32 = 0x012088b4;
    const TICK_B: u32 = 0x00f1c040;
    const GATE_B: u32 = 0x01037720;
    const GATE_B_SKIP: u32 = 0x12;
    const SNAP_GLOBAL: u32 = 0x011f7068;
    const SNAP_SIZE: u32 = 0x80;
    const DIRTY: u32 = 0x1a8;
    const CHECK: u32 = 1;
    const MEMBER_UPDATE: u32 = 2;
    const ALLOC: u32 = 3;
    const SNAPSHOT: u32 = 4;
    const TAIL_UPDATE: u32 = 5;
    unsafe {
        let count = lf_checker_rt::global::<u16>(COUNT_GLOBAL).read_unaligned() as i32;
        if count <= 0 {
            return 0;
        }
        let want = ((this + WANT_ID) as *const u32).read_unaligned();
        let table = lf_checker_rt::global::<u32>(TABLE_GLOBAL).read_unaligned();
        let mut found = 0u32;
        let mut found_at_count = false;
        let mut i = 0i32;
        while i < count {
            let e = ((table + (i as u32).wrapping_mul(4)) as *const u32).read_unaligned();
            if ((e + ID_OFF) as *const u32).read_unaligned() == want {
                found = e;
                found_at_count = true;
                break;
            }
            i += 1;
        }
        if !found_at_count {
            return 0;
        }
        if found == 0 {
            return 0;
        }
        let target = ((this + TARGET) as *const u32).read_unaligned();
        let vtable = (target as *const u32).read_unaligned();
        let check: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable + VT_CHECK) as *const u32).read_unaligned() as usize,
        );
        let mut refresh = check(target) & 0xff == 0;
        if !refresh {
            // `ucomiss f, 0.0; lahf; (an instruction of the original); jp` is taken exactly
            // when f is not equal to zero (less, greater or unordered),
            // and plain `!=` reproduces that including NaN and -0.0.
            let mut any_set = false;
            for off in FLOATS {
                let w = ((this + off) as *const u32).read_unaligned();
                any_set |= f32::from_bits(w) != 0.0;
            }
            refresh = any_set;
        }
        if !refresh {
            return 1;
        }
        lf_checker_rt::callee_thiscall!(MEMBER_UPDATE, u32, this);
        let gate_a = lf_checker_rt::global::<u32>(GATE_A).read_unaligned();
        if gate_a != 1 {
            let ta = lf_checker_rt::global::<u32>(TICK_A).read_unaligned();
            let tb = lf_checker_rt::global::<u32>(TICK_B).read_unaligned();
            if ta == tb {
                let gb = lf_checker_rt::global::<u32>(GATE_B).read_unaligned();
                if gb != GATE_B_SKIP {
                    let snap = lf_checker_rt::global::<u32>(SNAP_GLOBAL).read_unaligned();
                    if snap == 0 {
                        let m = lf_checker_rt::callee_cdecl!(ALLOC, u32, SNAP_SIZE);
                        lf_checker_rt::global::<u32>(SNAP_GLOBAL).write_unaligned(m);
                        lf_checker_rt::callee_thiscall!(SNAPSHOT, u32, this, m);
                    } else {
                        lf_checker_rt::callee_thiscall!(SNAPSHOT, u32, this, snap);
                    }
                    ((this + DIRTY) as *mut u8).write(0);
                }
            }
        }
        lf_checker_rt::callee_thiscall!(TAIL_UPDATE, u32, this);
    }
    1
});
