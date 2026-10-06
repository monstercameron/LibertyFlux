//! Differential cases: [`HandlerSlots::swap`] against its twelve instances.
//!
//! Each case plants one shared slot, every save cell of the chain, and
//! one replacement address, runs the rewrite and the lift on the same
//! words, and compares the answer, the shared slot, the unit's save cell
//! (rebuilt from the lifted answer word for word, so the translation is
//! proven and not assumed), and that every other save cell is untouched.
//! A deliberately wrong lift (parking and answering the new handler
//! instead of the displaced one) runs through the same contract and must
//! be caught. 32-bit target only.
//!
//! [`HandlerSlots::swap`]: lf_network::net_handler::HandlerSlots::swap

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_nethandler_diff::rewrites::*;
    use lf_nethandler_diff::rt;
    use lf_network::net_handler::{Handler, HandlerSlots};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock};

    /// One swap instance: the rewrite plus its three words.
    type SwapFn = extern "cdecl" fn() -> u32;
    struct Instance {
        swap: SwapFn,
        slot: u32,
        save: u32,
        new_va: u32,
    }
    const INSTANCES: [Instance; 12] = [
        Instance {
            swap: fn_00E5E600::rw_00e5e600,
            slot: 0x017A_CD24,
            save: 0x0110_EA1C,
            new_va: 0x0110_EA18,
        },
        Instance {
            swap: fn_00E5E620::rw_00e5e620,
            slot: 0x017A_D1B8,
            save: 0x0110_E9E4,
            new_va: 0x0110_E9D8,
        },
        Instance {
            swap: fn_00E5E640::rw_00e5e640,
            slot: 0x017A_D1B8,
            save: 0x0110_E9D4,
            new_va: 0x0110_E9C8,
        },
        Instance {
            swap: fn_00E5E660::rw_00e5e660,
            slot: 0x017A_D1B8,
            save: 0x0110_E9C4,
            new_va: 0x0110_E9B8,
        },
        Instance {
            swap: fn_00E5E680::rw_00e5e680,
            slot: 0x017A_D1B8,
            save: 0x0110_E9A4,
            new_va: 0x0110_E998,
        },
        Instance {
            swap: fn_00E5E6A0::rw_00e5e6a0,
            slot: 0x017A_D1B8,
            save: 0x0110_E974,
            new_va: 0x0110_E968,
        },
        Instance {
            swap: fn_00E5E6C0::rw_00e5e6c0,
            slot: 0x017A_D1B8,
            save: 0x0110_EA2C,
            new_va: 0x0110_EA20,
        },
        Instance {
            swap: fn_00E5E6E0::rw_00e5e6e0,
            slot: 0x017A_D1B8,
            save: 0x0110_E9F4,
            new_va: 0x0110_E9E8,
        },
        Instance {
            swap: fn_00E5E700::rw_00e5e700,
            slot: 0x017A_D1B8,
            save: 0x0110_EA3C,
            new_va: 0x0110_EA30,
        },
        Instance {
            swap: fn_00E5E720::rw_00e5e720,
            slot: 0x017A_D1B8,
            save: 0x0110_EA14,
            new_va: 0x0110_EA08,
        },
        Instance {
            swap: fn_00E5E740::rw_00e5e740,
            slot: 0x017A_D1B8,
            save: 0x0110_EA4C,
            new_va: 0x0110_EA40,
        },
        Instance {
            swap: fn_00E5E760::rw_00e5e760,
            slot: 0x017A_D1B8,
            save: 0x0110_E9B4,
            new_va: 0x0110_E9A8,
        },
    ];
    /// The eleven-unit chain's save cells, in unit order.
    const SAVES_B: [u32; 11] = [
        0x0110_E9E4,
        0x0110_E9D4,
        0x0110_E9C4,
        0x0110_E9A4,
        0x0110_E974,
        0x0110_EA2C,
        0x0110_E9F4,
        0x0110_EA3C,
        0x0110_EA14,
        0x0110_EA4C,
        0x0110_E9B4,
    ];

    /// Deliberately wrong lift: parks and answers the NEW handler instead
    /// of the displaced one (the old/new mix-up this shuffle invites).
    /// Runs through the same contract and must be caught.
    struct WrongSlots<const N: usize> {
        current: u32,
        saves: [u32; N],
    }

    impl<const N: usize> WrongSlots<N> {
        fn swap(&mut self, unit: usize, new: u32) -> u32 {
            self.current = new;
            self.saves[unit] = new;
            new
        }
    }

    /// Reads one word the rewrite wrote. Raw reads: the rewrite wrote
    /// through the address, invisibly to the borrow checker.
    unsafe fn get_word(va: u32) -> u32 {
        unsafe { rt::global::<u32>(va).read() }
    }

    /// Plants one global word.
    unsafe fn put_word(va: u32, v: u32) {
        unsafe { rt::global::<u32>(va).write(v) };
    }

    /// Runs one instance over old/new word pairs. Returns (comparisons, caught).
    fn run_instance<const N: usize>(
        unit: usize,
        inst: &Instance,
        save_vas: [u32; N],
        seed: u32,
    ) -> (u32, u32) {
        let mut rng = Rng(seed);
        // A real test address, so address-shaped words flow through too.
        let keepalive = Box::leak(Box::new(seed));
        let real_addr = addr(keepalive);
        assert_ne!(real_addr, 0, "test address must be nonzero");
        let mut olds = vec![
            0u32,
            1,
            2,
            0x7FFF_FFFF,
            0x8000_0000,
            0xFFFF_FFFE,
            0xFFFF_FFFF,
            real_addr,
        ];
        for _ in 0..12 {
            olds.push(rng.u32());
        }
        let mut news = vec![0u32, 1, real_addr, 0xFFFF_FFFF];
        for _ in 0..8 {
            news.push(rng.u32());
        }
        let mut cases = 0;
        let mut caught = 0;
        for &old in &olds {
            for &new in &news {
                // Distinct garbage per cell, so a stray write is visible.
                let mut garbage = [0u32; N];
                for (cell, slot) in garbage.iter_mut().enumerate() {
                    let g = rng.u32();
                    *slot = g;
                    unsafe { put_word(save_vas[cell], g) };
                }
                unsafe { put_word(inst.slot, old) };
                rt::set_relocated(inst.new_va, new);

                let ret_rw = unsafe { (inst.swap)() };
                let slot_rw = unsafe { get_word(inst.slot) };
                let save_rw = unsafe { get_word(inst.save) };

                // Lift from the same words: two setup swaps plant the
                // installed word and the unit's save garbage exactly.
                // (The untouched cells start clear on the lift side, so
                // they are compared before/after instead of to garbage.)
                let mut lift = HandlerSlots::<N>::new();
                lift.swap(unit, Handler::new(garbage[unit]));
                lift.swap(unit, Handler::new(old));
                let before: Vec<Option<Handler>> =
                    (0..N).map(|cell| lift.saved(cell)).collect();
                let ret_lift = lift.swap(unit, Handler::new(new));
                let ret_lift_raw = Handler::raw_or_zero(ret_lift);
                let slot_lift_raw = Handler::raw_or_zero(lift.current());
                let save_lift_raw = Handler::raw_or_zero(lift.saved(unit));
                assert_eq!(ret_lift_raw, ret_rw, "old={old:#x} new={new:#x}");
                assert_eq!(slot_lift_raw, slot_rw, "old={old:#x} new={new:#x}");
                assert_eq!(save_lift_raw, save_rw, "old={old:#x} new={new:#x}");
                // Every other cell untouched, on both sides.
                for (cell, &va) in save_vas.iter().enumerate() {
                    if cell == unit {
                        continue;
                    }
                    assert_eq!(
                        unsafe { get_word(va) },
                        garbage[cell],
                        "rewrite touched cell {cell}"
                    );
                    assert_eq!(lift.saved(cell), before[cell], "lift touched cell {cell}");
                }

                // The wrong lift through the same contract.
                let mut wrong = WrongSlots {
                    current: old,
                    saves: garbage,
                };
                let w_ret = wrong.swap(unit, new);
                if (w_ret, wrong.current, wrong.saves[unit]) != (ret_rw, slot_rw, save_rw) {
                    caught += 1;
                }
                cases += 1;
            }
        }
        std::hint::black_box(keepalive);
        (cases, caught)
    }

    macro_rules! swap_test {
        ($name:ident, $idx:expr, $unit:expr, $n:expr, $saves:expr, $seed:expr) => {
            #[test]
            fn $name() {
                let _guard = lock();
                let (cases, caught) =
                    run_instance::<$n>($unit, &INSTANCES[$idx], $saves, $seed);
                assert!(cases > 200, "too few comparisons ({cases})");
                assert!(caught > 0, "wrong lift never caught ({cases} cases)");
            }
        };
    }

    swap_test!(swap_600_matches, 0, 0, 1, [0x0110_EA1C], 0xC001);
    swap_test!(swap_620_matches, 1, 0, 11, SAVES_B, 0xC002);
    swap_test!(swap_640_matches, 2, 1, 11, SAVES_B, 0xC003);
    swap_test!(swap_660_matches, 3, 2, 11, SAVES_B, 0xC004);
    swap_test!(swap_680_matches, 4, 3, 11, SAVES_B, 0xC005);
    swap_test!(swap_6a0_matches, 5, 4, 11, SAVES_B, 0xC006);
    swap_test!(swap_6c0_matches, 6, 5, 11, SAVES_B, 0xC007);
    swap_test!(swap_6e0_matches, 7, 6, 11, SAVES_B, 0xC008);
    swap_test!(swap_700_matches, 8, 7, 11, SAVES_B, 0xC009);
    swap_test!(swap_720_matches, 9, 8, 11, SAVES_B, 0xC00A);
    swap_test!(swap_740_matches, 10, 9, 11, SAVES_B, 0xC00B);
    swap_test!(swap_760_matches, 11, 10, 11, SAVES_B, 0xC00C);
}
