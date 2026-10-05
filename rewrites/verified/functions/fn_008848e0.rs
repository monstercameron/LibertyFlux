// original: 0x008848e0 stream_mgr_create (proposed)
/// Create a manager entry: allocate its slot, attach it, and initialise its payload.
///
/// Takes the manager lock at `mgr+0x45c` (intercepted callee 1, thiscall:
/// scratch in `ecx`, lock address as the stack argument) and fails with -1
/// when the entry count at `mgr+0x448` has reached `MAX_ENTRIES` (8).
/// Otherwise allocates a double-aux slot for (`mgr`, `owner`) (intercepted
/// callee 2, cdecl, two arguments), converts it to a pool index
/// (intercepted callee 3, cdecl, one argument), and attaches that index to
/// the owner (intercepted callee 4, thiscall: `owner` in `ecx`, index as the
/// stack argument); a zero reply frees the slot (intercepted callee 6,
/// cdecl, one argument) and fails with -1. On success records the index in
/// the handle table at `mgr+0x208`, fills the payload vectors with `1.0f`
/// (`count = [slot+0x10] & 0xf` words at `[slot+0x08]` and at the partner
/// vector, which is `[slot+0x08] + 4` when `count == 1` and `[slot+0x0c]`
/// otherwise; nothing when `count == 0`), bumps the entry count, releases
/// the lock (intercepted callee 5, thiscall, scratch in `ecx`), and returns
/// the entry's index. The scratch area is two zero words, matching the
/// checker's zero stack fill.
///
/// Original: thiscall, one stack argument, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_008848e0(mgr: u32, owner: u32) -> u32 {
    unsafe {
        const HANDLE_TABLE: u32 = 0x208;
        const ENTRY_COUNT: u32 = 0x448;
        const MGR_LOCK: u32 = 0x45c;
        const MAX_ENTRIES: u32 = 8;
        const PAYLOAD: u32 = 0x08;
        const PARTNER: u32 = 0x0c;
        const KIND_WORD: u32 = 0x10;
        const KIND_MASK: u32 = 0x0f;
        const ONE_BITS: u32 = 0x3f80_0000;
        const LOCK_CALLEE: u32 = 1;
        const ALLOC_CALLEE: u32 = 2;
        const INDEX_CALLEE: u32 = 3;
        const ATTACH_CALLEE: u32 = 4;
        const UNLOCK_CALLEE: u32 = 5;
        const SLOT_FREE_CALLEE: u32 = 6;
        let mut scratch = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOCK_CALLEE,
            u32,
            scratch.as_mut_ptr() as u32,
            mgr.wrapping_add(MGR_LOCK)
        );
        let fail = |scratch: &mut [u32; 2]| {
            let _: u32 =
                lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, scratch.as_mut_ptr() as u32);
            0xFFFF_FFFFu32
        };
        let count = ((mgr + ENTRY_COUNT) as *const u32).read_unaligned();
        if count >= MAX_ENTRIES {
            return fail(&mut scratch);
        }
        let slot: u32 = lf_checker_rt::callee_cdecl!(ALLOC_CALLEE, u32, mgr, owner);
        let index: u32 = lf_checker_rt::callee_cdecl!(INDEX_CALLEE, u32, slot);
        let attached: u32 =
            lf_checker_rt::callee_thiscall!(ATTACH_CALLEE, u32, owner, index);
        if attached as u8 == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(SLOT_FREE_CALLEE, u32, slot);
            return fail(&mut scratch);
        }
        ((mgr + HANDLE_TABLE + count.wrapping_mul(2)) as *mut u16)
            .write_unaligned(index as u16);
        let kind = ((slot + KIND_WORD) as *const u32).read_unaligned() & KIND_MASK;
        let base = ((slot + PAYLOAD) as *const u32).read_unaligned();
        let partner = if kind == 1 {
            base.wrapping_add(4)
        } else {
            ((slot + PARTNER) as *const u32).read_unaligned()
        };
        if kind != 0 {
            // The original biases the partner by `-base` and writes pairs.
            let biased = partner.wrapping_sub(base);
            let mut at = base;
            let mut done = 0u32;
            while done < kind {
                (biased.wrapping_add(at) as *mut u32).write_unaligned(ONE_BITS);
                (at as *mut u32).write_unaligned(ONE_BITS);
                at = at.wrapping_add(4);
                done += 1;
            }
        }
        let entry = ((mgr + ENTRY_COUNT) as *const u32).read_unaligned();
        ((mgr + ENTRY_COUNT) as *mut u32).write_unaligned(entry.wrapping_add(1));
        let _: u32 =
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, scratch.as_mut_ptr() as u32);
        entry
    }
});
