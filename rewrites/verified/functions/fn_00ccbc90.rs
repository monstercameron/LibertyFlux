// original: 0x00ccbc90 CTaskComplexMoveAboutInjured::vf19
/// Start the injured-move task: seed the ped's timer and a random duration.
///
/// Copies the ped's timer word (`ped+0xb94`) into `this+0x1c`, then draws
/// a random word: values at or above 0x3fff (signed) reseed the ped timer
/// to 0x40, smaller ones to 0x3f. The manager handle (thiscall on the
/// global pointer) returning non-null forwards (5, 0x65, 1) to the starter
/// whose answer (or 0) is returned. A second random draw, converted to
/// float, is scaled by the read-only constants (((r * K1) * 20.0) - 10.0)
/// + 20.0 in the original's order and stored to `this+0x18`. Thiscall.
export!(thiscall, rw_00ccbc90(this: u32, ped: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        const MGR_G: u32 = 0x0167e2a0;
        const TIMER_OFF: u32 = 0x1c;
        const PED_TIMER_OFF: u32 = 0xb94;
        const DUR_OFF: u32 = 0x18;
        const DRAW_CUT: i32 = 0x3fff;
        const SEED_LO: u32 = 0x3f;
        const K1_G: u32 = 0x00fe8684;
        const K2_G: u32 = 0x00fe8b38;
        const K3_G: u32 = 0x00fe8b08;
        let prev = (ped.wrapping_add(PED_TIMER_OFF) as *const u32).read_unaligned();
        (this.wrapping_add(TIMER_OFF) as *mut u32).write_unaligned(prev);
        let draw1: u32 = callee_cdecl!(1, u32,);
        let seed = if (draw1 as i32) >= DRAW_CUT { SEED_LO + 1 } else { SEED_LO };
        (ped.wrapping_add(PED_TIMER_OFF) as *mut u32).write_unaligned(seed);
        let mgr = *global::<u32>(MGR_G);
        let h: u32 = callee_thiscall!(2, u32, mgr);
        let out = if h != 0 {
            callee_thiscall!(3, u32, h, 5u32, 0x65u32, 1u32)
        } else {
            0
        };
        let draw2: u32 = callee_cdecl!(1, u32,);
        let k1 = f32::from_bits(*global::<u32>(K1_G));
        let k2 = f32::from_bits(*global::<u32>(K2_G));
        let k3 = f32::from_bits(*global::<u32>(K3_G));
        let v = add(sub(mul(mul((draw2 as i32) as f32, k1), k2), k3), k2);
        (this.wrapping_add(DUR_OFF) as *mut u32).write_unaligned(v.to_bits());
        out
    }
});
