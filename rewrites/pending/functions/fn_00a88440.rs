// original: 0x00A88440 cam.followped.blur.cap
/// Reload the follow-camera blur limits from settings.
///
/// When the settings gate is open, fetches each named limit through the
/// settings reader, applies the grouped ones, commits each group through
/// its finalizer, and stores the trailing cap to the blur-cap word.
/// Returns the trailing cap bits, or the untouched entry register when the
/// gate is closed (that register is fixed by the contract).
export!(cdecl, rw_a88440() -> u32 {
    const OBJ: u32 = 0x015D_E394;
    const GATE: u32 = 0x015D_E3A4;
    const CAP_OUT: u32 = 0x0104_8AE4;
    // Entry eax on the gate-closed path; the original returns it untouched
    // and the contract fixes it to this value.
    const ENTRY_EAX: u32 = 0x1234_5678;
    if unsafe { global::<u8>(GATE).read() } == 0 {
        return ENTRY_EAX;
    }
    let obj = relocated(OBJ);
    // The applier pointer targets a scratch slot the original never wrote;
    // under the contract's zero stack fill it reads 0 on both sides.
    let scratch = 0u32;
    let at = &scratch as *const u32 as u32;
    // Group 1: four limits, four applies, one commit.
    let g1 = callee_thiscall!(1, f32, obj, relocated(0x00EA_2454), 0);
    let g2 = callee_thiscall!(2, f32, obj, relocated(0x00EA_246C), 0);
    let g3 = callee_thiscall!(3, f32, obj, relocated(0x00EA_2490), 0);
    let g4 = callee_thiscall!(4, f32, obj, relocated(0x00EA_24B8), 0);
    let r1 = callee_thiscall!(21, u32, obj, at, relocated(0x00EA_24D8));
    let r2 = callee_thiscall!(22, u32, obj, at, relocated(0x00EA_24F0));
    let r3 = callee_thiscall!(23, u32, obj, at, relocated(0x00EA_250C));
    let r4 = callee_thiscall!(24, u32, obj, at, relocated(0x00EA_2528));
    // Finalizer arguments list top-of-stack first, matching push order.
    callee_cdecl!(30, u32, r4, r3, r2, r1, g4.to_bits(), g3.to_bits(), g2.to_bits(), g1.to_bits());
    // Group 2: five limits, two applies, one commit.
    let g5 = callee_thiscall!(5, f32, obj, relocated(0x00EA_2540), 0);
    let g6 = callee_thiscall!(6, f32, obj, relocated(0x00EA_2558), 0);
    let g7 = callee_thiscall!(7, f32, obj, relocated(0x00EA_257C), 0);
    let g8 = callee_thiscall!(8, f32, obj, relocated(0x00EA_25A4), 0);
    let g9 = callee_thiscall!(9, f32, obj, relocated(0x00EA_25C0), 0);
    let r5 = callee_thiscall!(25, u32, obj, at, relocated(0x00EA_25E0));
    let r6 = callee_thiscall!(26, u32, obj, at, relocated(0x00EA_25FC));
    callee_cdecl!(31, u32, r6, r5, g9.to_bits(), g8.to_bits(), g7.to_bits(), g6.to_bits(), g5.to_bits());
    // Group 3: two limits, three applies, one commit.
    let g10 = callee_thiscall!(10, f32, obj, relocated(0x00EA_2614), 0);
    let g11 = callee_thiscall!(11, f32, obj, relocated(0x00EA_2630), 0);
    let r7 = callee_thiscall!(27, u32, obj, at, relocated(0x00EA_2654));
    let r8 = callee_thiscall!(28, u32, obj, at, relocated(0x00EA_2674));
    let r9 = callee_thiscall!(29, u32, obj, at, relocated(0x00EA_2694));
    callee_cdecl!(32, u32, r9, r8, r7, g11.to_bits(), g10.to_bits());
    // Group 4: five limits straight to the commit.
    let g12 = callee_thiscall!(12, f32, obj, relocated(0x00EA_26B0), 0);
    let g13 = callee_thiscall!(13, f32, obj, relocated(0x00EA_26C4), 0);
    let g14 = callee_thiscall!(14, f32, obj, relocated(0x00EA_26E4), 0);
    let g15 = callee_thiscall!(15, f32, obj, relocated(0x00EA_26FC), 0);
    let g16 = callee_thiscall!(16, f32, obj, relocated(0x00EA_271C), 0);
    callee_cdecl!(33, u32, g16.to_bits(), g15.to_bits(), g14.to_bits(), g13.to_bits(), g12.to_bits());
    // Group 5: three limits straight to the commit.
    let g17 = callee_thiscall!(17, f32, obj, relocated(0x00EA_2734), 0);
    let g18 = callee_thiscall!(18, f32, obj, relocated(0x00EA_274C), 0);
    let g19 = callee_thiscall!(19, f32, obj, relocated(0x00EA_2768), 0);
    callee_cdecl!(34, u32, g19.to_bits(), g18.to_bits(), g17.to_bits());
    // Trailing cap: stored to the cap word and returned.
    let cap = callee_thiscall!(20, f32, obj, relocated(0x00EA_277C), 0);
    let bits = cap.to_bits();
    unsafe { global::<u32>(CAP_OUT).write(bits) };
    bits
});
