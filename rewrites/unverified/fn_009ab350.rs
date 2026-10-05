// original: 0x009AB350 audWeatherAudioEntity::audWeatherAudioEntity

/// Constructor of the weather audio entity: runs the base-class constructor
/// on `this`, installs the class vtable pointer, then constructs every member
/// sub-object in layout order.
///
/// `this` points to a 0x1658-byte object (base `audGtaAudioEntity` at +0x00).
/// The members fall into two interleaved families by construction routine:
/// "curve" members built 0x28 bytes apart and "slot" members built 0x1c bytes
/// apart, each family appearing both as unrolled singles and as counted loops
/// (`dec`/`jns`, so a start value of N means N+1 constructions).
///
/// Construction order: base at +0x00, vtable store, slot member at +0x20,
/// 30 curve members from +0x60, 30 curve members from +0x510, one curve at
/// +0x9c0, two slots at +0x9e8/+0xa04, 12 slots from +0xa54, five curves from
/// +0xc24, a slot at +0xcf4, a curve at +0xd5c, a slot at +0xfb0, 16 slots
/// from +0x11e0, 4 slots from +0x13b4, 4 slots from +0x1444, and 11 curves
/// from +0x14b4 (with one 0x38 step at 0x157c->0x15b4 where a slot is
/// skipped). Every construction call takes only the member address in
/// ECX and returns nothing used. Returns `this`.
///
/// Original: 0x009AB350 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009ab350(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E92D34;
        const BASE_CTOR: u32 = 1;
        const SLOT_CTOR: u32 = 2;
        const CURVE_CTOR: u32 = 3;
        const SLOT_CTOR_2: u32 = 4;

        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));

        lf_checker_rt::callee_thiscall!(SLOT_CTOR, u32, this.wrapping_add(0x20));

        let mut member = this.wrapping_add(0x60);
        for _ in 0..30 {
            lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, member);
            member = member.wrapping_add(0x28);
        }
        member = this.wrapping_add(0x510);
        for _ in 0..30 {
            lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, member);
            member = member.wrapping_add(0x28);
        }

        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x9c0));
        lf_checker_rt::callee_thiscall!(SLOT_CTOR_2, u32, this.wrapping_add(0x9e8));
        lf_checker_rt::callee_thiscall!(SLOT_CTOR_2, u32, this.wrapping_add(0xa04));

        member = this.wrapping_add(0xa54);
        for _ in 0..12 {
            lf_checker_rt::callee_thiscall!(SLOT_CTOR_2, u32, member);
            member = member.wrapping_add(0x1c);
        }

        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0xc24));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0xc4c));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0xc7c));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0xca4));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0xccc));
        lf_checker_rt::callee_thiscall!(SLOT_CTOR_2, u32, this.wrapping_add(0xcf4));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0xd5c));
        lf_checker_rt::callee_thiscall!(SLOT_CTOR_2, u32, this.wrapping_add(0xfb0));

        member = this.wrapping_add(0x11e0);
        for _ in 0..16 {
            lf_checker_rt::callee_thiscall!(SLOT_CTOR_2, u32, member);
            member = member.wrapping_add(0x1c);
        }
        member = this.wrapping_add(0x13b4);
        for _ in 0..4 {
            lf_checker_rt::callee_thiscall!(SLOT_CTOR_2, u32, member);
            member = member.wrapping_add(0x1c);
        }
        member = this.wrapping_add(0x1444);
        for _ in 0..4 {
            lf_checker_rt::callee_thiscall!(SLOT_CTOR_2, u32, member);
            member = member.wrapping_add(0x1c);
        }

        // Eleven unrolled singles; the stride is 0x28 throughout except for
        // one 0x38 step (0x157c -> 0x15b4), so they stay unrolled here too.
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x14b4));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x14dc));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x1504));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x152c));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x1554));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x157c));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x15b4));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x15dc));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x1604));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x162c));
        lf_checker_rt::callee_thiscall!(CURVE_CTOR, u32, this.wrapping_add(0x1654));

        this
    }
});
