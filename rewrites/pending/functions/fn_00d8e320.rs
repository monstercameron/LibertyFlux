// original: 0x00d8e320 audio_entity_reinit_gate
/// Refresh the entity unless a global override flag is set.
///
/// Asks the hooked controller (reached through its data-table slot) about
/// the global handle; a nonzero answer, or the pair of fallback flags both
/// set, selects the refresh. Either override byte set afterward vetoes it
/// and ends the call with the combined flag byte. Otherwise the status word
/// is cleared, the two refresh passes run, and control tails into the shared
/// finaliser. Returns the finaliser's answer on the refresh path.
lf_rs89_rt::export!(thiscall, rw_00d8e320(this: *mut u8) -> u32 {
    unsafe {
        let arg = *lf_rs89_rt::global::<u32>(0x17ACCD8);
        let target = *lf_rs89_rt::global::<u32>(0xE733DC);
        let ask: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(target as usize);
        let selected = if ask(this as u32, arg) != 0 {
            1u32
        } else if *lf_rs89_rt::global::<u8>(0x105B48F) != 0
            && *lf_rs89_rt::global::<u8>(0x17ED8D1) != 0
        {
            1
        } else {
            0
        };
        let flags = selected
            | (*lf_rs89_rt::global::<u8>(0x1173590) as u32)
            | (*lf_rs89_rt::global::<u8>(0x1173591) as u32);
        if flags != 0 {
            return flags;
        }
        *(this.wrapping_add(4) as *mut u32) = 0;
        lf_rs89_rt::callee_thiscall!(2, u32, this as u32);
        lf_rs89_rt::callee_thiscall!(3, u32, this as u32);
        lf_rs89_rt::callee_thiscall!(4, u32, this as u32)
    }
});
