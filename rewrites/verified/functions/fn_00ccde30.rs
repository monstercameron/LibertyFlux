// original: 0x00ccde30 CTaskComplexDie::vf6
/// Return the die task's blend duration: 25.0 or -1.0 on the x87 stack.
///
/// Returns 25.0 early when bit 1 of `this+0xc` is set. Otherwise the ped
/// argument must be non-null with a zero byte at `+0x212`, the subtask at
/// `this+8` non-null answering 0x116 through its virtual slot at `+0xc`,
/// the ped check (thiscall on the ped) answering zero, and the info block
/// at subtask `+0x14` non-null with neither word at `+0xc`/`+0x10` equal
/// to -1: then 25.0. Any miss falls through to the helper at `ped+0x6c`,
/// which must be non-null with a non-zero byte at `+0xe`, else -1.0.
/// Thiscall with one stack argument.
export!(thiscall, rw_00ccde30(this: u32, ped: u32) -> f32 {
    unsafe {
        const SLOW_SECS: u32 = 0x00fe8b40;
        const FAST_SECS: u32 = 0x00fe8d94;
        const FLAG_OFF: u32 = 0x0c;
        const SUB_OFF: u32 = 0x08;
        const VT_SLOT: u32 = 0x0c;
        const WANT: u32 = 0x116;
        const PED_FLAG_OFF: u32 = 0x212;
        const INFO_OFF: u32 = 0x14;
        const HELP_OFF: u32 = 0x6c;
        const HELP_FLAG_OFF: u32 = 0x0e;
        if (this.wrapping_add(FLAG_OFF) as *const u32).read_unaligned() >> 1 & 1 != 0 {
            return f32::from_bits(*global::<u32>(SLOW_SECS));
        }
        if ped == 0 {
            return f32::from_bits(*global::<u32>(FAST_SECS));
        }
        let mut slow = false;
        if (ped.wrapping_add(PED_FLAG_OFF) as *const u8).read() == 0 {
            let sub = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
            if sub != 0 {
                let vtab = (sub as *const u32).read_unaligned();
                let slot = (vtab.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                if f(sub) == WANT {
                    let t: u32 = callee_thiscall!(2, u32, ped);
                    if t & 0xff == 0 {
                        let info = (sub.wrapping_add(INFO_OFF) as *const u32).read_unaligned();
                        if info != 0
                            && (info.wrapping_add(0x10) as *const u32).read_unaligned()
                                != 0xffff_ffff
                            && (info.wrapping_add(0x0c) as *const u32).read_unaligned()
                                != 0xffff_ffff
                        {
                            slow = true;
                        }
                    }
                }
            }
        }
        if !slow {
            let h = (ped.wrapping_add(HELP_OFF) as *const u32).read_unaligned();
            if h != 0 && (h.wrapping_add(HELP_FLAG_OFF) as *const u8).read() != 0 {
                slow = true;
            }
        }
        f32::from_bits(*global::<u32>(if slow { SLOW_SECS } else { FAST_SECS }))
    }
});
