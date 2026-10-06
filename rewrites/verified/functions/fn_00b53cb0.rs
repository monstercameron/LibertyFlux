// original: 0x00B53CB0 crmt_manager_init (proposed)

/// Initialise the manager object and all of its sub-objects, then chain.
///
/// When the byte at `this` has bit 0 set, registers the block at +0x1A04
/// (callee 1 on the fixed context, with (+0x1A04, 1)). Then releases the
/// three sub-objects at +0x4, +0x8B8 and +0x11DC through their table
/// slot 2 (callee 2), runs the three block steps (callees 4, 5, 6),
/// stamps the table address at +0x11DC and builds the 32-entry array
/// there (callee 7, first at +0x19E8 then sweeping +0x2F4 down to +0xC
/// relative to +0x11DC), and finishes the small objects (callees 8, 9,
/// 10, with a second stamp at +0x117C). After one last step on +0x8B8
/// (callee 11) the function tail-jumps to the same teardown with +0x4
/// (callee 12), whose answer is returned. The 32-sweep counter is
/// compared signed (a borrow ends it); unsigned it would not terminate.
///
/// Original: 0x00B53CB0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b53cb0(this: u32) -> u32 {
    unsafe {
        const GATE_CTX: u32 = 0x1669d78;
        const STAMP1: u32 = 0xeaf328;
        const STAMP2: u32 = 0xeaf2fc;
        const SUB2: u32 = 0x8b8;
        const SUB3: u32 = 0x11dc;
        const BLOCK: u32 = 0x1a04;
        const REG: u32 = 1;
        const STEP_A: u32 = 4;
        const STEP_B: u32 = 5;
        const STEP_C: u32 = 6;
        const BUILD: u32 = 7;
        const FIN_BIG: u32 = 8;
        const FIN_A: u32 = 9;
        const FIN_B: u32 = 10;
        const LAST: u32 = 11;
        const TAIL: u32 = 12;
        if (((this) as *const u8).read() & 1) != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                REG, u32, lf_checker_rt::relocated(GATE_CTX), this + BLOCK, 1
            );
        }
        for off in [4u32, SUB2, SUB3] {
            let o = this + off;
            let vt = (o as *const u32).read_unaligned();
            let fptr = ((vt + 8) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(fptr as usize);
            let _: u32 = f(o);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(STEP_A, u32, this + BLOCK);
        let _: u32 = lf_checker_rt::callee_thiscall!(STEP_B, u32, this);
        let _: u32 = lf_checker_rt::callee_thiscall!(STEP_C, u32, this + BLOCK);
        let ebp = this + SUB3;
        (ebp as *mut u32).write_unaligned(lf_checker_rt::relocated(STAMP1));
        let _: u32 = lf_checker_rt::callee_thiscall!(BUILD, u32, ebp + 0x80c);
        let mut s = ebp + 0x30c;
        let mut i = 0x1fu32;
        loop {
            s = s.wrapping_sub(0x18);
            let _: u32 = lf_checker_rt::callee_thiscall!(BUILD, u32, s);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(FIN_BIG, u32, ebp);
        let _: u32 = lf_checker_rt::callee_thiscall!(FIN_A, u32, this + 0x11c4);
        let _: u32 = lf_checker_rt::callee_thiscall!(FIN_A, u32, this + 0x11ac);
        let _: u32 = lf_checker_rt::callee_thiscall!(FIN_B, u32, this + 0x119c);
        let _: u32 = lf_checker_rt::callee_thiscall!(FIN_B, u32, this + 0x118c);
        ((this + 0x117c) as *mut u32).write_unaligned(lf_checker_rt::relocated(STAMP2));
        let _: u32 = lf_checker_rt::callee_thiscall!(FIN_A, u32, this + 0x117c);
        let _: u32 = lf_checker_rt::callee_thiscall!(FIN_B, u32, this + 0x116c);
        let _: u32 = lf_checker_rt::callee_thiscall!(LAST, u32, this + SUB2);
        lf_checker_rt::callee_thiscall!(TAIL, u32, this + 4)
    }
});
