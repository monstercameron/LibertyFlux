// original: 0x00CAB430 event_cap_construct (proposed)

/// Construct a capped-float event record in place and return its address.
///
/// Runs the base constructor on `this`, installs the record's virtual table,
/// then stores the two float arguments (as bit patterns) at `+0x34` and
/// `+0x38` with zero words around them. The second value is clamped down to
/// the global cap: when it is not above the cap word, the cap word is stored
/// instead (an unordered comparison stores the cap, matching `comiss`/`ja`).
/// All float movement is bitwise; no arithmetic is performed.
///
/// Original: 0x00CAB430 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00cab430(this: u32, first_bits: u32, second_bits: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00ED89CC;
        const CAP_GLOBAL: u32 = 0x00FE88E8;
        const FIRST: u32 = 0x34;
        const SECOND: u32 = 0x38;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this.wrapping_add(FIRST) as *mut u32).write_unaligned(first_bits);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x30) as *mut u32).write_unaligned(0);
        (this.wrapping_add(SECOND) as *mut u32).write_unaligned(second_bits);
        (this.wrapping_add(0x3C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x40) as *mut u8).write(0);
        (this.wrapping_add(0x44) as *mut u32).write_unaligned(0);
        let cap = lf_checker_rt::global::<u32>(CAP_GLOBAL).read_unaligned();
        let above = f32::from_bits(second_bits) > f32::from_bits(cap);
        if !above {
            (this.wrapping_add(SECOND) as *mut u32).write_unaligned(cap);
        }
        this
    }
});
