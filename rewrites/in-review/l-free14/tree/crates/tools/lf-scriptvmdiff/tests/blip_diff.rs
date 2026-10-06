//! Differential cases, part 4: the blip marker table against its two
//! routines.
//!
//! Each case builds real 32-bit rows (flag byte, kind word, two stored
//! positions) behind a planted row-pointer table, runs the rewrite and
//! the matching [`BlipTable`] method on the same handle, and compares
//! the lookup calls, the apply calls or the written words, and the
//! return. Deliberately wrong lifts (kind 6 for kind 7; swapped position
//! selection) must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_scriptvmdiff::rewrites::*;
    use lf_scriptvmdiff::rt;
    use lf_script::script_vm::{APPLY_TAG, BlipRow, BlipTable, LOOKUP_MISS};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{GLOB_VA, Rng, TABLE_VA, addr, lock, snap};

    /// Row image size: the kind word ends at byte 76.
    const ROW_LEN: usize = 80;

    /// Writes a little-endian word into a row image.
    fn put(buf: &mut [u8], off: usize, v: u32) {
        buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    /// One recorded lookup call: the handle.
    static LOOKUP_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    /// Scripted lookup answers, popped per call.
    static LOOKUP_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());

    extern "cdecl" fn lookup_stub(handle: u32) -> u32 {
        LOOKUP_LOG.lock().unwrap().push(handle);
        LOOKUP_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    /// One recorded apply call: tag, handle, packed position.
    #[derive(Debug, PartialEq, Eq)]
    struct ApplyCall {
        tag: u32,
        handle: u32,
        pos: [u32; 3],
    }

    static APPLY_LOG: Mutex<Vec<ApplyCall>> = Mutex::new(Vec::new());

    extern "cdecl" fn apply_stub(tag: u32, handle: u32, buf: u32) -> u32 {
        let words = unsafe { snap(buf, 3) };
        APPLY_LOG.lock().unwrap().push(ApplyCall {
            tag,
            handle,
            pos: [words[0], words[1], words[2]],
        });
        0
    }

    /// Builds one 32-bit row image from a lifted row.
    fn row_image(row: &BlipRow) -> Box<[u8; ROW_LEN]> {
        let mut img = Box::new([0u8; ROW_LEN]);
        img[8] = u8::from(row.flag);
        put(img.as_mut(), 0x20, row.pos_b[0]);
        put(img.as_mut(), 0x24, row.pos_b[1]);
        put(img.as_mut(), 0x30, row.pos_a[0]);
        put(img.as_mut(), 0x34, row.pos_a[1]);
        put(img.as_mut(), 0x38, row.pos_a[2]);
        put(img.as_mut(), 0x48, row.kind);
        img
    }

    /// Scripted rows: flag mixes, kinds across and around the gate,
    /// distinct positions.
    fn script_rows(rng: &mut Rng) -> Vec<BlipRow> {
        // Kinds: below, inside and above the 4/5/7 gate, plus extremes.
        let kinds = [0u32, 1, 3, 4, 5, 6, 7, 8, 0xFFFF_FFFF];
        let mut rows = Vec::new();
        for (i, &kind) in kinds.iter().enumerate() {
            rows.push(BlipRow {
                flag: i % 2 == 0,
                kind,
                pos_a: [rng.u32(), rng.u32(), rng.u32() | 1],
                pos_b: [rng.u32(), rng.u32()],
            });
        }
        // A few fully random rows, flags mixed.
        for i in 0..3 {
            rows.push(BlipRow {
                flag: i % 2 == 1,
                kind: rng.u32(),
                pos_a: [rng.u32(), rng.u32(), rng.u32() | 1],
                pos_b: [rng.u32(), rng.u32()],
            });
        }
        rows
    }

    /// The wrong kind gate: 6 applies instead of 7. Must be caught
    /// wherever the selected row's kind is 6 or 7.
    fn wrong_applies(kind: u32) -> bool {
        matches!(kind, 4 | 5 | 6)
    }

    /// Runs the apply routine. Returns (comparisons, caught).
    fn run_apply(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, lookup_stub as *const () as u32);
        rt::set_callee(2, apply_stub as *const () as u32);
        let rows = script_rows(&mut rng);
        let images: Vec<Box<[u8; ROW_LEN]>> = rows.iter().map(row_image).collect();
        let table: Vec<u32> = images.iter().map(|img| addr(img.as_ref())).collect();
        rt::set_relocated(TABLE_VA, addr(&table[0]));
        let (mut cases, mut caught) = (0, 0);
        // Negative answers return at once, without touching the table.
        let mut negatives = vec![0xFFFF_FFFFu32, 0xFFFF_FFFE, 0x8000_0000, 0x8000_0001];
        for _ in 0..4 {
            negatives.push(rng.u32() | 0x8000_0000);
        }
        for &id in &negatives {
            let handle = rng.u32();
            unsafe { rt::global::<u32>(GLOB_VA).write(rng.below(rows.len() as u32)) };
            LOOKUP_LOG.lock().unwrap().clear();
            LOOKUP_SCRIPT.lock().unwrap().clear();
            LOOKUP_SCRIPT.lock().unwrap().push_back(id);
            APPLY_LOG.lock().unwrap().clear();
            let (x, y, z) = (rng.u32(), rng.u32(), rng.u32());
            let got = unsafe { fn_00B913A0::rw_00b913a0(handle, x, y, z) };
            assert_eq!(got, 0);
            assert_eq!(*LOOKUP_LOG.lock().unwrap(), [handle]);
            assert!(APPLY_LOG.lock().unwrap().is_empty());
            // The lift agrees: no apply call.
            let table_lift = BlipTable::new(rows.clone(), 0);
            let mut lift_lookup = Vec::new();
            let mut lift_apply = Vec::new();
            table_lift.apply_offset(
                &mut |h: u32| {
                    lift_lookup.push(h);
                    id
                },
                &mut |tag: u32, h: u32, pos: [u32; 3]| {
                    lift_apply.push(ApplyCall { tag, handle: h, pos });
                },
                handle,
                x,
                y,
                z,
            );
            assert_eq!(lift_lookup, [handle]);
            assert!(lift_apply.is_empty());
            cases += 1;
        }
        // Valid ids across every row, with every fallback row.
        for id in 0..rows.len() as u32 {
            for fb in 0..rows.len() as u32 {
                let handle = rng.u32();
                let (x, y, z) = (rng.u32(), rng.u32(), rng.u32());
                unsafe { rt::global::<u32>(GLOB_VA).write(fb) };
                LOOKUP_LOG.lock().unwrap().clear();
                LOOKUP_SCRIPT.lock().unwrap().clear();
                LOOKUP_SCRIPT.lock().unwrap().push_back(id);
                APPLY_LOG.lock().unwrap().clear();
                let got = unsafe { fn_00B913A0::rw_00b913a0(handle, x, y, z) };
                assert_eq!(got, 0);
                assert_eq!(*LOOKUP_LOG.lock().unwrap(), [handle]);
                let selected = if rows[id as usize].flag {
                    &rows[id as usize]
                } else {
                    &rows[fb as usize]
                };
                let applies = matches!(selected.kind, 4 | 5 | 7);
                let want = if applies {
                    vec![ApplyCall {
                        tag: APPLY_TAG,
                        handle,
                        pos: [x, y, z],
                    }]
                } else {
                    vec![]
                };
                assert_eq!(*APPLY_LOG.lock().unwrap(), want, "id={id} fb={fb}");
                let table_lift = BlipTable::new(rows.clone(), fb);
                let mut lift_lookup = Vec::new();
                let mut lift_apply = Vec::new();
                table_lift.apply_offset(
                    &mut |h: u32| {
                        lift_lookup.push(h);
                        id
                    },
                    &mut |tag: u32, h: u32, pos: [u32; 3]| {
                        lift_apply.push(ApplyCall { tag, handle: h, pos });
                    },
                    handle,
                    x,
                    y,
                    z,
                );
                assert_eq!(lift_lookup, *LOOKUP_LOG.lock().unwrap());
                assert_eq!(lift_apply, *APPLY_LOG.lock().unwrap());
                if wrong_applies(selected.kind) != applies {
                    caught += 1;
                }
                cases += 1;
            }
        }
        std::hint::black_box(&table);
        std::hint::black_box(&images);
        (cases, caught)
    }

    /// Reads one word the rewrite wrote through a passed address.
    unsafe fn get_at(at: u32) -> u32 {
        unsafe { (at as *const u32).read_unaligned() }
    }

    /// Runs the read routine. Returns (comparisons, caught).
    fn run_read(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, lookup_stub as *const () as u32);
        let rows = script_rows(&mut rng);
        let images: Vec<Box<[u8; ROW_LEN]>> = rows.iter().map(row_image).collect();
        let table: Vec<u32> = images.iter().map(|img| addr(img.as_ref())).collect();
        rt::set_relocated(TABLE_VA, addr(&table[0]));
        let (mut cases, mut caught) = (0, 0);
        let mut run = |id: u32, cases: &mut u32, caught: &mut u32| {
            let handle = rng.u32();
            LOOKUP_LOG.lock().unwrap().clear();
            LOOKUP_SCRIPT.lock().unwrap().clear();
            LOOKUP_SCRIPT.lock().unwrap().push_back(id);
            // The output slot starts as garbage the rewrite must overwrite.
            let mut out = Box::new([0xA5A5_A5A5u32; 4]);
            let out_addr = addr(out.as_ref());
            let got = unsafe { fn_00B921C0::rw_00b921c0(handle, out_addr) };
            assert_eq!(got, out_addr, "the rewrite answers the out pointer");
            assert_eq!(*LOOKUP_LOG.lock().unwrap(), [handle]);
            // Raw reads: the rewrite wrote through the address, invisibly
            // to the borrow checker.
            let seen = unsafe {
                [
                    get_at(out_addr),
                    get_at(out_addr.wrapping_add(4)),
                    get_at(out_addr.wrapping_add(8)),
                    get_at(out_addr.wrapping_add(12)),
                ]
            };
            let want = if id == LOOKUP_MISS {
                [0, 0, 0, 0]
            } else {
                let row = &rows[id as usize];
                if row.flag {
                    [row.pos_a[0], row.pos_a[1], row.pos_a[2], 0]
                } else {
                    [row.pos_b[0], row.pos_b[1], 0, 0]
                }
            };
            assert_eq!(seen, want, "id={id:#x}");
            // The lift answers the same words.
            let table_lift = BlipTable::new(rows.clone(), 0);
            let mut lift_lookup = Vec::new();
            let lift_seen = table_lift.read_offset(
                &mut |h: u32| {
                    lift_lookup.push(h);
                    id
                },
                handle,
            );
            assert_eq!(lift_lookup, [handle]);
            assert_eq!(lift_seen, seen);
            // Wrong lift: the swapped selection (flag set reads the
            // unflagged side). Differs wherever the two sides disagree.
            if id != LOOKUP_MISS {
                let row = &rows[id as usize];
                let wrong = if row.flag {
                    [row.pos_b[0], row.pos_b[1], 0, 0]
                } else {
                    [row.pos_a[0], row.pos_a[1], row.pos_a[2], 0]
                };
                if wrong != want {
                    *caught += 1;
                }
            }
            *cases += 1;
            std::hint::black_box(&mut out);
        };
        run(LOOKUP_MISS, &mut cases, &mut caught);
        for id in 0..rows.len() as u32 {
            run(id, &mut cases, &mut caught);
            // Twice more with fresh garbage, to shake out stale reads.
            run(id, &mut cases, &mut caught);
        }
        std::hint::black_box(&table);
        std::hint::black_box(&images);
        (cases, caught)
    }

    #[test]
    fn apply_matches() {
        let _guard = lock();
        let (cases, caught) = run_apply(0xD001);
        assert!(cases > 100, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong kind gate never caught ({cases} cases)");
    }

    #[test]
    fn read_matches() {
        let _guard = lock();
        let (cases, caught) = run_read(0xD002);
        assert!(cases > 20, "too few comparisons ({cases})");
        assert!(caught > 0, "swapped selection never caught ({cases} cases)");
    }
}
