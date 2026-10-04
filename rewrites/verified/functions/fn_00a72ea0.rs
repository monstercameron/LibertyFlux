// original: 0x00a72ea0 CTaskComplexPlayerIdles::vf5 (symbols)
/// Runs the sub-task hook, then an identity chain that decides whether a
/// state probe runs; always 1 unless the hook vetoes.
///
/// `thiscall`: object in ECX, three stack words (owner, spare, target),
/// callee pops 12. The hook is the family's usual one (skipped when bit 0
/// of `sub+0xc` is set, else virtual slot `+0x14` with the three words;
/// zero vetoes with a 0 return; a hook that ran latches bit 1). Then: a
/// null link at `owner+0x2c4` is done; otherwise the sign-extended word
/// at `link+0x2e` must equal the owner's identity query (virtual slot
/// `+0x12c`), else done. A null target, a target query (slot `+4`) other
/// than 0x20, a null detail at `target+0x10`, or a detail query
/// (slot `+0xc`) outside {0x3C5, 0x640} all run the state probe
/// (callee 5, thiscall on `owner+0x2b0`); matching 0x3C5 or 0x640 skips
/// it. Every path past the hook returns 1.
lf_checker_rt::export!(thiscall, rw_00a72ea0(this: u32, owner: u32, _x: u32, target: u32) -> u8 {
    unsafe {
        const SUB_OFF: u32 = 0x8;
        const FLAG_OFF: u32 = 0xc;
        const HOOK_SLOT: u32 = 0x14;
        const LINK_OFF: u32 = 0x2c4;
        const TAG_OFF: u32 = 0x2e;
        const IDENT_SLOT: u32 = 0x12c;
        const KIND_WANT: u32 = 0x20;
        const KIND_SLOT: u32 = 0x4;
        const DETAIL_OFF: u32 = 0x10;
        const STATE_SLOT: u32 = 0xc;
        const STATE_A: u32 = 0x3c5;
        const STATE_B: u32 = 0x640;
        const PROBE_OFF: u32 = 0x2b0;
        let sub = ((this + SUB_OFF) as *const u32).read_unaligned();
        if ((sub + FLAG_OFF) as *const u8).read() & 1 == 0 {
            let vt = (sub as *const u32).read_unaligned();
            let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(
                    (((vt + HOOK_SLOT) as *const u32).read_unaligned()) as usize,
                );
            if hook(sub, owner, _x, target) as u8 == 0 {
                return 0;
            }
            let w = ((sub + FLAG_OFF) as *const u32).read_unaligned();
            ((sub + FLAG_OFF) as *mut u32).write_unaligned(w | 2);
        }
        let link = ((owner + LINK_OFF) as *const u32).read_unaligned();
        if link != 0 {
            let tag =
                (((link + TAG_OFF) as *const u16).read_unaligned()) as i16 as i32 as u32;
            let vt = (owner as *const u32).read_unaligned();
            let ident: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                (((vt + IDENT_SLOT) as *const u32).read_unaligned()) as usize,
            );
            if tag == ident(owner) && {
                let mut probe = true;
                if target != 0 {
                    let vt = (target as *const u32).read_unaligned();
                    let kind: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                        (((vt + KIND_SLOT) as *const u32).read_unaligned()) as usize,
                    );
                    if kind(target) == KIND_WANT {
                        let det = ((target + DETAIL_OFF) as *const u32).read_unaligned();
                        if det != 0 {
                            let vt = (det as *const u32).read_unaligned();
                            let state: extern "thiscall" fn(u32) -> u32 =
                                core::mem::transmute(
                                    (((vt + STATE_SLOT) as *const u32).read_unaligned())
                                        as usize,
                                );
                            let s = state(det);
                            if s == STATE_A || s == STATE_B {
                                probe = false;
                            }
                        }
                    }
                }
                probe
            } {
                lf_checker_rt::callee_thiscall!(5, u32, owner.wrapping_add(PROBE_OFF));
            }
        }
        1
    }
});
