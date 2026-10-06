//! Differential cases, part 1: the pack-and-skip routine against its
//! three flag instances.
//!
//! Each case runs the rewrite and [`PointSkip::emit`] on the same words
//! and compares the forwarded point, key and flags, snapshotting the
//! rewrite's frame buffer inside the stub (the frame is gone on return).
//! A deliberately wrong lift (first and last point words swapped) must be
//! caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_scriptvmdiff::rewrites::*;
    use lf_scriptvmdiff::rt;
    use lf_script::script_vm::{PointSkip, SkipFlags};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, lock, snap};

    /// One recorded skip call: packed point, key, flags.
    #[derive(Debug, PartialEq, Eq)]
    struct Call {
        point: [u32; 3],
        key: u32,
        flags: SkipFlags,
    }

    static SKIP_LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());

    extern "cdecl" fn skip_stub(buf: u32, key: u32, f0: u32, f1: u32, f2: u32) -> u32 {
        let words = unsafe { snap(buf, 3) };
        SKIP_LOG.lock().unwrap().push(Call {
            point: [words[0], words[1], words[2]],
            key,
            flags: SkipFlags(f0, f1, f2),
        });
        0
    }

    /// One skip instance: its rewrite and its flag words.
    type SkipFn = extern "cdecl" fn(u32, u32, u32, u32) -> u32;
    struct Instance {
        skip: SkipFn,
        flags: SkipFlags,
    }
    const INSTANCES: [Instance; 3] = [
        Instance {
            skip: fn_00B972C0::rw_00b972c0,
            flags: SkipFlags::NONE,
        },
        Instance {
            skip: fn_00B97310::rw_00b97310,
            flags: SkipFlags::FIRST_SET,
        },
        Instance {
            skip: fn_00B97420::rw_00b97420,
            flags: SkipFlags::TRAIL_SET,
        },
    ];

    /// Deliberately wrong pack: first and last words swapped. Must be
    /// caught wherever the two ends differ.
    fn wrong_point(a0: u32, a1: u32, a2: u32) -> [u32; 3] {
        [a2, a1, a0]
    }

    /// Runs one instance over edge and random words.
    /// Returns (comparisons, caught).
    fn run_instance(inst: &Instance, seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, skip_stub as usize as u32);
        let proto = PointSkip;
        let mut cases = 0;
        let mut caught = 0;
        // Edge words: zero, small, sign boundaries, all ones.
        let edges = [0u32, 1, 2, 0x7FFF_FFFF, 0x8000_0000, 0xFFFF_FFFE, 0xFFFF_FFFF];
        let mut run = |a0: u32, a1: u32, a2: u32, key: u32, cases: &mut u32, caught: &mut u32| {
            SKIP_LOG.lock().unwrap().clear();
            let got = unsafe { (inst.skip)(a0, a1, a2, key) };
            assert_eq!(got, 0);
            let seen = SKIP_LOG.lock().unwrap();
            assert_eq!(seen.len(), 1, "one skip call per case");
            let want = Call {
                point: [a0, a1, a2],
                key,
                flags: inst.flags,
            };
            assert_eq!(seen[0], want);
            drop(seen);
            // The lift, with the instance's flags, must make the same call.
            let mut lift_log = Vec::new();
            proto.emit(
                &mut |point: [u32; 3], k: u32, f: SkipFlags| {
                    lift_log.push(Call {
                        point,
                        key: k,
                        flags: f,
                    });
                },
                a0,
                a1,
                a2,
                key,
                inst.flags,
            );
            assert_eq!(lift_log, [want]);
            if wrong_point(a0, a1, a2) != [a0, a1, a2] {
                *caught += 1;
            }
            *cases += 1;
        };
        for &a0 in &edges {
            for &a1 in &edges {
                run(a0, a1, rng.u32(), rng.u32(), &mut cases, &mut caught);
            }
        }
        for _ in 0..60 {
            run(rng.u32(), rng.u32(), rng.u32(), rng.u32(), &mut cases, &mut caught);
        }
        (cases, caught)
    }

    macro_rules! skip_test {
        ($name:ident, $idx:expr, $seed:expr) => {
            #[test]
            fn $name() {
                let _guard = lock();
                let (cases, caught) = run_instance(&INSTANCES[$idx], $seed);
                assert!(cases > 100, "too few comparisons ({cases})");
                assert!(caught > 0, "wrong pack never caught ({cases} cases)");
            }
        };
    }

    skip_test!(skip_flag0_matches, 0, 0xE001);
    skip_test!(skip_flag1_matches, 1, 0xE002);
    skip_test!(skip_trailflag_matches, 2, 0xE003);
}
