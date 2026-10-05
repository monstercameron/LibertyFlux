// original: 0x00A4B740 CVehicle::vf98

/// Virtual method 98: gates on checks, then runs a worker chain over a table
/// entry.
///
/// Calls the first callee (thiscall, `this` in `ecx`, the argument object on
/// the stack); a non-zero low byte skips the gates. Otherwise returns unless
/// the argument's link at `+LINK` (0x6C) is present and flagged (byte
/// `+0x0E` non-zero), the adjusted word `[arg + ADJ] + ADJ_OFF` (0x224,
/// +0x2E0) is
/// non-zero, and the second callee (thiscall, that word in `ecx`, `(0x2C5,
/// 0)`) answers non-zero. Then resolves `info = [TABLE_ENTRY(model) +
/// INFO]` (0xC4) from the global pointer table by the signed word at `this +
/// MODEL` (0x2E), tags the incoming argument slot with `TAG` (0x175) and
/// calls the worker (cdecl, `(info, &slot, arg, this, 0, 0, 0, 1)`; the slot
/// address is skipped as a stack address and its contents snapshotted). Calls
/// the fourth callee (thiscall, `[arg + CTX]` (0x78) in `ecx`, `(worker_ans,
/// TAG)`) and returns unless it answers zero; then calls the fifth
/// (thiscall, same `ecx`, `(worker_ans, TAG, 4.0f)`). Returns nothing
/// defined. (The tag store hits the dead incoming-arg slot, which a Rust
/// rewrite cannot address: the contract switches the stack check off. The
/// rewrite models the slot with a local holding the tag.)
///
/// Original: 0x00A4B740 (thiscall, two stack words), five callees.
lf_checker_rt::export!(thiscall, rw_00A4B740(this: u32, arg: u32, _a1: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x6C;
        const LINK_FLAG: u32 = 0x0E;
        const ADJ: u32 = 0x224;
        const ADJ_OFF: u32 = 0x2E0;
        const CTX: u32 = 0x78;
        const MODEL: u32 = 0x2E;
        const TABLE: u32 = 0x01295CD8;
        const INFO: u32 = 0xC4;
        const TAG: u32 = 0x175;
        const MODE: u32 = 0x2C5;
        const K4: f32 = f32::from_bits(0x4080_0000); // 4.0
        let gate: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, arg);
        if (gate & 0xFF) == 0 {
            let link = ((arg + LINK) as *const u32).read_unaligned();
            if link == 0 || ((link + LINK_FLAG) as *const u8).read() == 0 {
                return 0;
            }
            let adj = ((arg + ADJ) as *const u32)
                .read_unaligned()
                .wrapping_add(ADJ_OFF);
            if adj == 0 {
                return 0;
            }
            let probe: u32 =
                lf_checker_rt::callee_thiscall!(2, u32, adj, MODE, 0);
            if (probe & 0xFF) == 0 {
                return 0;
            }
        }
        let model = ((this + MODEL) as *const i16).read_unaligned() as i32;
        let slot = lf_checker_rt::relocated(TABLE)
            .wrapping_add((model * 4) as u32);
        let entry = (slot as *const u32).read_unaligned();
        let info =
            ((entry.wrapping_add(INFO)) as *const u32).read_unaligned();
        let tag_slot = TAG;
        let work: u32 = lf_checker_rt::callee_cdecl!(
            3,
            u32,
            info,
            &tag_slot as *const u32 as u32,
            arg,
            this,
            0,
            0,
            0,
            1
        );
        let ctx = ((arg + CTX) as *const u32).read_unaligned();
        let act: u32 =
            lf_checker_rt::callee_thiscall!(4, u32, ctx, work, TAG);
        if act == 0 {
            lf_checker_rt::callee_thiscall!(
                5, u32, ctx, work, TAG, K4.to_bits()
            );
        }
        0
    }
});
