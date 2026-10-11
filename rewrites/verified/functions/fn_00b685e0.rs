// original: 0x00B685E0 heli_tail_break_effects

/// For a live component table and a nonnegative component selector, first
/// check whether the component is already marked broken, then resolve its
/// fragment and optional fragment member. If a member exists, emit the part
/// trail and play the helicopter tail-break sound. All paths after the second
/// lookup clear `this + 0x1efc`; early exits leave it untouched. The five
/// direct helpers are intercepted by the checker. This thiscall routine is
/// semantically void, so the path-dependent EAX value is not compared.
lf_checker_rt::export!(thiscall, rw_00B685E0(this: u32) -> () {
    unsafe {
        const COMPONENT_TABLE: u32 = 0xdc4;
        const COMPONENT_SELECTOR: u32 = 0x1ef2;
        const BREAK_STATE: u32 = 0x1efc;
        const BITSET_CONTAINS: u32 = 1;
        const LOOKUP_PART: u32 = 2;
        const FRAGMENT_MEMBER: u32 = 3;
        const EMIT_PART_TRAIL: u32 = 4;
        const PLAY_BREAK_SOUND: u32 = 5;
        const VEHICLE_FX_THIS_VA: u32 = 0x016d_9f58;

        let table = (this.wrapping_add(COMPONENT_TABLE) as *const u32).read_unaligned();
        if table == 0 {
            return;
        }
        let raw_selector = (this.wrapping_add(COMPONENT_SELECTOR) as *const u8).read_unaligned();
        let selector = raw_selector as i8;
        if selector < 0 {
            return;
        }
        let selector_word = selector as i32 as u32;
        let already_broken = lf_checker_rt::callee_thiscall!(
            BITSET_CONTAINS, u32, table, selector_word
        );
        if (already_broken as u8) != 0 {
            return;
        }
        let part = lf_checker_rt::callee_thiscall!(LOOKUP_PART, u32, table, selector_word);
        if part != 0 {
            let member = lf_checker_rt::callee_cdecl!(FRAGMENT_MEMBER, u32, part);
            if member != 0 {
                let fx_this = lf_checker_rt::relocated(VEHICLE_FX_THIS_VA);
                let _ = lf_checker_rt::callee_thiscall!(EMIT_PART_TRAIL, u32, fx_this, member);
                let audio_this = this.wrapping_add(0x210);
                let _ = lf_checker_rt::callee_thiscall!(PLAY_BREAK_SOUND, u32, audio_this);
            }
        }
        (this.wrapping_add(BREAK_STATE) as *mut u32).write_unaligned(0);
    }
});
