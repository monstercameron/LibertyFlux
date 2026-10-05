// original: 0x00c5fed0 ped_task_gate (proposed)

/// Gate deciding whether a ped task may proceed, as a pure predicate over the
/// ped object, a helper object answered by a callee, and three subordinate
/// checks.
///
/// Arguments: one ped pointer (`ped`, cdecl, the only stack word; `ecx` is
/// saved and restored untouched). Returns a boolean in `al` (upper bytes of
/// `eax` are leftovers on some paths, hence the `al` return channel).
///
/// Reads (offsets from `ped`): flag bytes at `+0x218`/`+0x219`, a pointer at
/// `+0x228` whose byte at `+0x55c` must be clear for the early exit, a flag
/// byte at `+0x26c` (bit 2 selects the vehicle branch), a vehicle pointer at
/// `+0xb30` whose dword at `+0x1304` must equal 4, and a state dword at
/// `+0x2b0` (only on the fallthrough path). Callee answers: `A` returns the
/// helper object (`esi`), `B` a gate byte (only its low byte is read), `C` a
/// veto (nonzero returns 0 at once).
///
/// Entry path: when the `+0x218` byte is clear while `+0x219` is set and the
/// `+0x228` object exists with a clear `+0x55c` byte, the result is 0 without
/// calling anything. Otherwise `A`, `B`, `C` run in order; a nonzero `C`
/// vetoes. With the gate byte set and a helper object present, two
/// obfuscated bytes of the helper (xor of adjacent bytes at `+0x290c/0x290e`
/// on the vehicle branch, `+0x28fc/0x28fe` otherwise) must exceed `0x7f` for
/// a 1 result.
///
/// Fallthrough path (gate byte clear): two global words are compared against
/// zero, a manager object answers through callee `D`, and a flag `cl` is
/// derived from callee `E` or, failing that, from the manager byte at
/// `+0x2d9` (or callee `F` when there is no manager). Three further
/// obfuscated helper bytes (`+0x26dc/0x26de`, `+0x26ec/0x26ee`,
/// `+0x28fc/0x28fe`) each return 1 when above `0x7f`; otherwise the result is
/// 0 when the ped state is 8, else `cl`. This path reads unrelocated global
/// addresses, which fault under the checker, so it is implemented but not
/// covered by the proof (see the contract's narrowed record).
///
/// Original: 0x00c5fed0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00c5fed0(ped: u32) -> u32 {
    unsafe {
        const FLAG_A: u32 = 0x218;
        const FLAG_B: u32 = 0x219;
        const SUB_PTR: u32 = 0x228;
        const SUB_GATE: u32 = 0x55c;
        const VEH_SEL_BIT: u8 = 0x04;
        const VEH_FLAGS: u32 = 0x26c;
        const VEH_PTR: u32 = 0xb30;
        const VEH_KIND: u32 = 0x1304;
        const VEH_KIND_WANT: u32 = 4;
        const PED_STATE: u32 = 0x2b0;
        const PED_STATE_IDLE: u32 = 8;
        const XOR_LIMIT: u8 = 0x7f;
        const MGR_WORD1: u32 = 0x1160c74;
        const MGR_WORD2: u32 = 0x1160e78;
        const MGR_OBJ: u32 = 0x103e498;
        const CAL_A: u32 = 1;
        const CAL_B: u32 = 2;
        const CAL_C: u32 = 3;
        const CAL_D: u32 = 4;
        const CAL_E: u32 = 5;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        /// Manager flag for the fallthrough path: the manager byte at
        /// `+0x2d9`, or callee `F` over the helper's embedded object at
        /// `+0x2c08` without one.
        #[inline(always)]
        unsafe fn mgr_flag(mgr: u32, helper: u32) -> bool {
            unsafe {
                if mgr != 0 {
                    ((mgr.wrapping_add(0x2d9)) as *const u8).read() != 0
                } else {
                    lf_checker_rt::callee_thiscall!(6u32, u32, helper.wrapping_add(0x2c08))
                        & 0xff
                        != 0
                }
            }
        }
        /// One obfuscated helper byte: xor of the bytes at the two addresses.
        #[inline(always)]
        unsafe fn xbyte(lo: u32, hi: u32) -> u8 {
            unsafe { rd8(hi) ^ rd8(lo) }
        }

        if rd8(ped.wrapping_add(FLAG_A)) == 0 && rd8(ped.wrapping_add(FLAG_B)) != 0 {
            let sub = rd32(ped.wrapping_add(SUB_PTR));
            if sub != 0 && rd8(sub.wrapping_add(SUB_GATE)) == 0 {
                return 0;
            }
        }
        let helper = lf_checker_rt::callee_thiscall!(CAL_A, u32, ped);
        let gate = lf_checker_rt::callee_thiscall!(CAL_B, u32, ped) & 0xff;
        // Callee C ignores its registers (a bare global read and return);
        // the original passes it whatever the previous stub left in ecx.
        let veto = lf_checker_rt::callee_cdecl!(CAL_C, u32,);
        if veto != 0 {
            return 0;
        }
        if gate != 0 {
            if helper == 0 {
                return 0;
            }
            let vehicle_ok = rd8(ped.wrapping_add(VEH_FLAGS)) & VEH_SEL_BIT != 0
                && {
                    let veh = rd32(ped.wrapping_add(VEH_PTR));
                    veh != 0 && rd32(veh.wrapping_add(VEH_KIND)) == VEH_KIND_WANT
                };
            let v = if vehicle_ok {
                xbyte(helper.wrapping_add(0x290c), helper.wrapping_add(0x290e))
            } else {
                xbyte(helper.wrapping_add(0x28fc), helper.wrapping_add(0x28fe))
            };
            return u32::from(v > XOR_LIMIT);
        }
        // Fallthrough path: not covered by the proof (unrelocated globals).
        // NOTE: the original reads these words through absolute operands
        // without relocation entries, so under the checker it faults before
        // any of this runs; the relocated reads below express the intent.
        let g1_zero = lf_checker_rt::global::<u32>(MGR_WORD1).read_unaligned() == 0;
        if helper == 0 {
            return 0;
        }
        let g2_zero = lf_checker_rt::global::<u32>(MGR_WORD2).read_unaligned() == 0;
        let mgr = lf_checker_rt::callee_thiscall!(CAL_D, u32, lf_checker_rt::relocated(MGR_OBJ));
        let cl: bool;
        if rd8(helper.wrapping_add(0x328d)) != 0 && g2_zero {
            cl = mgr_flag(mgr, helper);
        } else {
            let e = lf_checker_rt::callee_thiscall!(CAL_E, u32, helper) & 0xff;
            if e != 0 {
                cl = true;
            } else if !g2_zero {
                cl = false;
            } else {
                cl = mgr_flag(mgr, helper);
            }
        }
        if xbyte(helper.wrapping_add(0x26dc), helper.wrapping_add(0x26de)) > XOR_LIMIT {
            return 1;
        }
        if g1_zero
            && xbyte(helper.wrapping_add(0x26ec), helper.wrapping_add(0x26ee)) > XOR_LIMIT
        {
            return 1;
        }
        if xbyte(helper.wrapping_add(0x28fc), helper.wrapping_add(0x28fe)) > XOR_LIMIT {
            return 1;
        }
        if rd32(ped.wrapping_add(PED_STATE)) == PED_STATE_IDLE {
            return 0;
        }
        u32::from(cl)
    }
});
