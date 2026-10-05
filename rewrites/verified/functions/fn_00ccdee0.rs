// original: 0x00ccdee0 CTaskComplexDie::vf7
/// Report the die task's completion request, returning the sink's answer.
///
/// Returns 0 unless every gate passes: bit 1 of `this+0xc` clear, the ped
/// argument non-null with a zero byte at `+0x212`, the subtask at `this+8`
/// non-null answering 0x116 through its virtual slot at `+0xc`, the ped
/// check (thiscall on the ped) answering zero, and the info block at
/// subtask `+0x14` non-null with neither word at `+0xc`/`+0x10` equal to
/// -1. Then the manager handle (thiscall on the global pointer) returning
/// non-null forwards the two info words and 1000.0 to the sink (thiscall
/// on the handle), whose answer is returned. (The original's dead reload
/// on the miss path reads mapped memory to no effect and is omitted.)
/// Thiscall with one stack argument.
export!(thiscall, rw_00ccdee0(this: u32, ped: u32) -> u32 {
    unsafe {
        const MGR_G: u32 = 0x0171faf4;
        const FLAG_OFF: u32 = 0x0c;
        const SUB_OFF: u32 = 0x08;
        const VT_SLOT: u32 = 0x0c;
        const WANT: u32 = 0x116;
        const PED_FLAG_OFF: u32 = 0x212;
        const INFO_OFF: u32 = 0x14;
        const RATE: u32 = 0x447a0000;
        if (this.wrapping_add(FLAG_OFF) as *const u32).read_unaligned() >> 1 & 1 != 0 {
            return 0;
        }
        if ped == 0 {
            return 0;
        }
        if (ped.wrapping_add(PED_FLAG_OFF) as *const u8).read() != 0 {
            return 0;
        }
        let sub = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        if sub == 0 {
            return 0;
        }
        let vtab = (sub as *const u32).read_unaligned();
        let slot = (vtab.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if f(sub) != WANT {
            return 0;
        }
        let t: u32 = callee_thiscall!(2, u32, ped);
        if t & 0xff != 0 {
            return 0;
        }
        let info = (sub.wrapping_add(INFO_OFF) as *const u32).read_unaligned();
        if info == 0 {
            return 0;
        }
        let w10 = (info.wrapping_add(0x10) as *const u32).read_unaligned();
        if w10 == 0xffff_ffff {
            return 0;
        }
        let w0c = (info.wrapping_add(0x0c) as *const u32).read_unaligned();
        if w0c == 0xffff_ffff {
            return 0;
        }
        let mgr = *global::<u32>(MGR_G);
        let h: u32 = callee_thiscall!(3, u32, mgr);
        if h == 0 {
            return 0;
        }
        callee_thiscall!(4, u32, h, w10, w0c, RATE)
    }
});
