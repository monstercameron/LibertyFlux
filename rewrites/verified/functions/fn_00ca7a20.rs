// original: 0x00CA7A20 CEventHandler::vf58

/// Decide a ped's reaction to an event: verify the event targets this
/// handler's ped, roll a random chance, and on success build a reaction of a
/// kind picked by cascading probability thresholds, then speak two lines.
///
/// `this` is the event handler (its ped at `+0x04`, the built reaction stored
/// at `+0x0c`). `ev` is the event: slot `0x34` of its function table is asked
/// for a descriptor whose flags at `+0x28`, masked with `0x3C0`, must equal
/// `0xC0`, and the event's ped at `+0x10` must be non-null. The handler's ped
/// must have bit 2 set at `+0x26C` and its link at `+0xB30` must equal the
/// event's ped. Any failed check returns at once (the returned value is
/// whatever the last read left: 0, the masked flags, `0xC0`, the handler ped
/// or the mismatched link).
///
/// On success the event ped is marked with `0x5A` at `+0x12EC`, a random
/// integer is drawn, converted to float and scaled, and compared against the
/// thresholds 0.15, 0.30, 0.45 and 0.60 in order: the first threshold above
/// the scaled roll selects the reaction kind (3, 13, 14, 9). A factory object
/// is fetched; when it exists the reaction is built from (event ped, kind,
/// 1000) and stored at `+0x0c`, otherwise 0 is stored. A roll at or above
/// 0.60 stores nothing. Finally two lines are spoken through the dialog
/// callee (this `ped + 0x570`, ten stack words: the line id, five zeros, -1,
/// 1.0, two zeros); a nonzero low byte from the first line skips the second.
/// The value returned is the last callee answer (or the early-exit value).
///
/// Original: 0x00CA7A20 (thiscall, ecx = handler, three stack words of which
/// only the first, the event, is read).
lf_checker_rt::export!(thiscall, rw_00ca7a20(this: u32, ev: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const EVT_VSLOT: u32 = 0x34;
        const DESC_FLAGS: u32 = 0x28;
        const FLAGS_MASK: u32 = 0x3C0;
        const FLAGS_WANT: u32 = 0xC0;
        const EVT_PED: u32 = 0x10;
        const H_PED: u32 = 0x04;
        const H_REACTION: u32 = 0x0C;
        const PED_READY: u32 = 0x26C;
        const READY_BIT: u8 = 0x04;
        const PED_LINK: u32 = 0xB30;
        const PED_MARK: u32 = 0x12EC;
        const MARK: u32 = 0x5A;
        const KIND_COUNT: u32 = 1000;
        const LINE_THIS_OFF: u32 = 0x570;
        const ONE_BITS: u32 = 0x3F80_0000;
        const G_FACTORY: u32 = 0x0167_E2A0;
        const C_ROLL: u32 = 0x00FE_8684;
        const C_T0: u32 = 0x00FE_87B4;
        const C_T1: u32 = 0x00FE_87E8;
        const C_T2: u32 = 0x00FE_8828;
        const C_T3: u32 = 0x00FE_8858;
        const LINE_A: u32 = 0x00ED_75E0;
        const LINE_B: u32 = 0x00ED_75F4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        // The event must describe the ped this handler owns.
        let vtable = rd32(ev);
        let describe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(EVT_VSLOT)) as usize);
        let desc = describe(ev);
        if desc == 0 {
            return 0;
        }
        let flags = rd32(desc.wrapping_add(DESC_FLAGS)) & FLAGS_MASK;
        if flags != FLAGS_WANT {
            return flags;
        }
        let ped = rd32(ev.wrapping_add(EVT_PED));
        if ped == 0 {
            return FLAGS_WANT;
        }
        let hped = rd32(this.wrapping_add(H_PED));
        if (hped.wrapping_add(PED_READY) as *const u8).read() & READY_BIT == 0 {
            return hped;
        }
        let link = rd32(hped.wrapping_add(PED_LINK));
        if link == 0 {
            return 0;
        }
        if link != ped {
            return link;
        }

        // Mark the ped, roll the chance, pick a kind by threshold cascade.
        wr32(ped.wrapping_add(PED_MARK), MARK);
        let roll: i32 = lf_checker_rt::callee_cdecl!(2, u32,) as i32;
        let scaled = mul(roll as f32, rdf(C_ROLL));
        // Each compare is comiss+jbe: continue while !(threshold > scaled).
        let kind = if rdf(C_T0) > scaled {
            3u32
        } else if rdf(C_T1) > scaled {
            13u32
        } else if rdf(C_T2) > scaled {
            14u32
        } else if rdf(C_T3) > scaled {
            9u32
        } else {
            let hp = rd32(this.wrapping_add(H_PED));
            return speak_two(hp, ped);
        };
        let factory: u32 =
            lf_checker_rt::callee_thiscall!(3, u32, rd32(lf_checker_rt::relocated(G_FACTORY)));
        let made = if factory == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(4, u32, factory, ped, kind, KIND_COUNT)
        };
        wr32(this.wrapping_add(H_REACTION), made);
        let hp = rd32(this.wrapping_add(H_PED));
        speak_two(hp, ped)
    }
});

/// Speak both lines; the low byte of the first answer decides whether the
/// second is spoken. Returns the last answer. (Shared tail of rw_00ca7a20:
/// kept out of line so the two callers stay identical.)
unsafe fn speak_two(hp: u32, _ped: u32) -> u32 {
    unsafe {
        const LINE_THIS_OFF: u32 = 0x570;
        const ONE_BITS: u32 = 0x3F80_0000;
        const LINE_A: u32 = 0x00ED_75E0;
        const LINE_B: u32 = 0x00ED_75F4;
        // Ten stack words each: line id, three zeros, -1, two zeros, 1.0,
        // two zeros. (The original pushes the ped and then overwrites that
        // slot with 1.0, so the ped is never actually passed.)
        let first: u32 = lf_checker_rt::callee_thiscall!(
            5, u32, hp.wrapping_add(LINE_THIS_OFF),
            lf_checker_rt::relocated(LINE_A), 0, 0, 0, 0xFFFF_FFFF, 0, 0, ONE_BITS, 0, 0
        );
        if first & 0xFF != 0 {
            return first;
        }
        lf_checker_rt::callee_thiscall!(
            5, u32, hp.wrapping_add(LINE_THIS_OFF),
            lf_checker_rt::relocated(LINE_B), 0, 0, 0, 0xFFFF_FFFF, 0, 0, ONE_BITS, 0, 0
        )
    }
}
