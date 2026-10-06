//! Differential cases, part 2: creation, occupancy, data, assignment,
//! offset lookup and the indexed store.
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares returns and every effect
//! (written bytes, published globals, callee call logs). Each method has
//! a deliberately wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_pooldiff::rewrites::*;
    use lf_pooldiff::rt;
    use lf_world::pools::{CtxHandle, SlotPool};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{AUX_VA, CTX_VA, MGR_VA, Rng, SCALE_VA, addr, lock, put_u32};

    /// Dead-slot bit, as the rewrites test it.
    const DEAD: u8 = 0x80;

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_world::pools::SlotPool;

        /// Tests bit 6 instead of bit 7.
        pub fn occupied_bit6(pool: &SlotPool, index: u32) -> bool {
            pool.flags()[index as usize] & 0x40 == 0
        }

        /// Reads the head word instead of entry + 4.
        pub fn data_head(pool: &SlotPool, index: u32) -> u32 {
            let off = index as usize * pool.stride() as usize;
            u32::from_le_bytes(pool.entries()[off..off + 4].try_into().unwrap())
        }

        /// Notifies on zero values instead of non-zero ones.
        pub fn assign_notify_zero(
            pool: &mut SlotPool,
            index: u32,
            value: u32,
            log: &mut Vec<u32>,
        ) -> bool {
            let off = index as usize * pool.stride() as usize;
            let mut entries = pool.entries().to_vec();
            entries[off..off + 4].copy_from_slice(&value.to_le_bytes());
            *pool = SlotPool::from_parts(entries, pool.flags().to_vec(), pool.stride());
            if value == 0 {
                log.push(index);
            }
            value != 0
        }

        /// Tests bit 6 instead of bit 7 for the slot flag.
        pub fn slot_bit6(pool: &SlotPool, offset: u32) -> Option<usize> {
            let count = pool.slot_count() as u32;
            let end = count.wrapping_sub(1).wrapping_mul(pool.stride());
            if offset > end {
                return None;
            }
            let index = offset / pool.stride();
            if offset % pool.stride() != 0 {
                return None;
            }
            match pool.flags().get(index as usize) {
                Some(f) if f & 0x40 == 0 => Some(index as usize),
                _ => None,
            }
        }

        /// Forgets the table base offset.
        pub fn cell_no_base(scale: u32, row: u32) -> u32 {
            scale.wrapping_mul(25).wrapping_add(row)
        }

        /// Allocates 32 bytes instead of 28.
        pub const CREATE_SIZE: u32 = 0x20;

        /// Decides survival on the whole word instead of the low byte.
        pub fn survives_full_word(answer: u32) -> bool {
            answer != 0
        }
    }

    // Recording stubs for the rewrite side. Each test plants the stubs
    // its rewrites call; the serial lock keeps the logs exact.
    static NOTIFY_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "cdecl" fn notify_stub(index: u32) -> u32 {
        NOTIFY_LOG.lock().unwrap().push(index);
        0
    }

    static ALLOC_SIZES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static ALLOC_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn alloc_stub(size: u32) -> u32 {
        ALLOC_SIZES.lock().unwrap().push(size);
        ALLOC_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    static INIT_LOG: Mutex<Vec<(u32, u32, u32, u32)>> = Mutex::new(Vec::new());
    static INIT_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "thiscall" fn init_stub(block: u32, a0: u32, a1: u32, tag: u32) -> u32 {
        INIT_LOG.lock().unwrap().push((block, a0, a1, tag));
        INIT_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    static SURVIVES_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static SURVIVES_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn survives_stub(index: u32, aux: u32) -> u32 {
        SURVIVES_LOG.lock().unwrap().push((index, aux));
        SURVIVES_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    static EVICT_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "cdecl" fn evict_stub(index: u32) -> u32 {
        EVICT_LOG.lock().unwrap().push(index);
        0
    }

    static REFRESH_TABLE: Mutex<u32> = Mutex::new(0);
    static REFRESH_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static REFRESH_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    /// Answers the translated address of the scripted row: `table + row*4`.
    extern "thiscall" fn refresh_stub(ctx: u32) -> u32 {
        REFRESH_LOG.lock().unwrap().push(ctx);
        let row = REFRESH_SCRIPT.lock().unwrap().pop_front().unwrap_or(0);
        REFRESH_TABLE
            .lock()
            .unwrap()
            .wrapping_add(row.wrapping_mul(4))
    }

    fn clear_logs() {
        NOTIFY_LOG.lock().unwrap().clear();
        ALLOC_SIZES.lock().unwrap().clear();
        ALLOC_SCRIPT.lock().unwrap().clear();
        INIT_LOG.lock().unwrap().clear();
        INIT_SCRIPT.lock().unwrap().clear();
        REFRESH_LOG.lock().unwrap().clear();
        REFRESH_SCRIPT.lock().unwrap().clear();
        SURVIVES_LOG.lock().unwrap().clear();
        SURVIVES_SCRIPT.lock().unwrap().clear();
        EVICT_LOG.lock().unwrap().clear();
    }

    /// Plants the given slot global over test memory and builds the
    /// matching lift. Returns the context address and the lift pool.
    /// The rewrite-side boxes are leaked so their addresses stay valid;
    /// each case leaks at most a few hundred bytes.
    fn plant_pool_entries(
        n: usize,
        stride: u32,
        flags: &[u8],
        entries: Vec<u8>,
        slot_va: u32,
    ) -> (u32, SlotPool) {
        assert_eq!(entries.len(), n * stride as usize);
        let pool = SlotPool::from_parts(entries.clone(), flags.to_vec(), stride);
        let entries_box = entries.into_boxed_slice();
        let flags_box = flags.to_vec().into_boxed_slice();
        let mut ctx = Box::new([0u8; 28]);
        put_u32(&mut ctx[..], 0x00, addr(&entries_box[0]));
        put_u32(&mut ctx[..], 0x04, addr(&flags_box[0]));
        put_u32(&mut ctx[..], 0x08, n as u32);
        put_u32(&mut ctx[..], 0x0c, stride);
        let ctx_addr = addr(&ctx[0]);
        unsafe { rt::global::<u32>(slot_va).write(ctx_addr) };
        std::mem::forget(entries_box);
        std::mem::forget(flags_box);
        std::mem::forget(ctx);
        (ctx_addr, pool)
    }

    /// Plants over fresh random entries (see above).
    fn plant_pool(
        n: usize,
        stride: u32,
        flags: &[u8],
        fill: &mut Rng,
        slot_va: u32,
    ) -> (u32, SlotPool) {
        let mut entries = vec![0u8; n * stride as usize];
        fill.bytes(&mut entries);
        plant_pool_entries(n, stride, flags, entries, slot_va)
    }

    #[test]
    fn create_ctx_matches() {
        let _guard = lock();
        clear_logs();
        rt::set_callee(1, alloc_stub as usize as u32);
        rt::set_callee(2, init_stub as usize as u32);
        let mut rng = Rng(0xC0E4);
        let mut cases = 0;
        let mut caught = 0;
        // (arguments, allocation succeeds, initialiser answers its block).
        for _ in 0..32 {
            let a0 = rng.u32();
            let a1 = rng.u32();
            for &succeeds in &[true, false] {
                for &echo in &[true, false] {
                    let block = Box::leak(Box::new([0u8; 28]));
                    let block_addr = addr(&block[0]);
                    let ctx_addr = if echo {
                        block_addr
                    } else {
                        let other = Box::leak(Box::new([0u8; 28]));
                        addr(&other[0])
                    };
                    ALLOC_SCRIPT
                        .lock()
                        .unwrap()
                        .push_back(if succeeds { block_addr } else { 0 });
                    // Script the initialiser answer only when it runs:
                    // a failed allocation leaves the queue untouched.
                    if succeeds {
                        INIT_SCRIPT.lock().unwrap().push_back(ctx_addr);
                    }
                    let mut lift_alloc_sizes = Vec::new();
                    let mut lift_init_args = Vec::new();
                    let scripted = if succeeds {
                        CtxHandle::new(block_addr)
                    } else {
                        None
                    };
                    let scripted_ctx = CtxHandle::new(ctx_addr).unwrap();
                    let lift = SlotPool::create_ctx(
                        a0,
                        a1,
                        &mut |size: u32| {
                            lift_alloc_sizes.push(size);
                            scripted
                        },
                        &mut |block: CtxHandle, x0, x1, tag| {
                            lift_init_args.push((block.get(), x0, x1, tag));
                            scripted_ctx
                        },
                    );
                    let before_alloc = ALLOC_SIZES.lock().unwrap().len();
                    let before_init = INIT_LOG.lock().unwrap().len();
                    let got = unsafe { fn_008E0240::rw_008e0240(a0, a1) };
                    let published = unsafe { rt::global::<u32>(CTX_VA).read() };
                    assert_eq!(got, published, "create must publish its answer");
                    assert_eq!(got, lift.map_or(0, |h| h.get()), "a0={a0:#x} a1={a1:#x}");
                    assert_eq!(
                        &ALLOC_SIZES.lock().unwrap()[before_alloc..],
                        &lift_alloc_sizes
                    );
                    assert_eq!(lift_alloc_sizes, [28u32]);
                    assert_eq!(&INIT_LOG.lock().unwrap()[before_init..], &lift_init_args);
                    if succeeds {
                        assert_eq!(lift_init_args, [(block_addr, a0, a1, 16u32)]);
                    } else {
                        assert!(lift_init_args.is_empty());
                    }
                    // The wrong size never matches the rewrite's call.
                    if lift_alloc_sizes != [wrong::CREATE_SIZE] {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases >= 100, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong create size never caught ({cases} cases)");
    }

    #[test]
    fn is_occupied_matches() {
        let _guard = lock();
        clear_logs();
        let mut rng = Rng(0x0CC4);
        let mut cases = 0;
        let mut caught = 0;
        for &(n, stride) in &[(1usize, 4u32), (2, 8), (3, 12), (8, 16), (9, 20), (16, 32)] {
            let mut patterns: Vec<Vec<u8>> = vec![
                vec![0x00; n],
                vec![0x80; n],
                vec![0x40; n],
                vec![0xFF; n],
                (0..n)
                    .map(|i| if i % 2 == 0 { 0x01 } else { 0xFE })
                    .collect(),
            ];
            for _ in 0..3 {
                let mut p = vec![0u8; n];
                rng.bytes(&mut p);
                patterns.push(p);
            }
            for flags in &patterns {
                let (_, pool) = plant_pool(n, stride, flags, &mut rng, CTX_VA);
                for index in 0..n as u32 {
                    let got = unsafe { fn_008E0310::rw_008e0310(index) };
                    let lift = pool.is_occupied(index);
                    assert_eq!(got, u32::from(lift), "n={n} index={index} flags={flags:?}");
                    if wrong::occupied_bit6(&pool, index) != lift {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong occupied bit never caught ({cases} cases)"
        );
    }

    #[test]
    fn data_word_matches() {
        let _guard = lock();
        clear_logs();
        let mut rng = Rng(0xDA7A);
        let mut cases = 0;
        let mut caught = 0;
        for &(n, stride) in &[(1usize, 8u32), (2, 4), (3, 12), (8, 16), (9, 8), (16, 32)] {
            let mut patterns: Vec<Vec<u8>> = vec![
                vec![0x00; n],
                (0..n)
                    .map(|i| if i % 2 == 0 { 0x00 } else { 0x80 })
                    .collect(),
                vec![0x40; n],
            ];
            for _ in 0..6 {
                let mut p = vec![0u8; n];
                rng.bytes(&mut p);
                patterns.push(p);
            }
            for flags in &patterns {
                let (_, pool) = plant_pool(n, stride, flags, &mut rng, CTX_VA);
                for index in 0..n as u32 {
                    let off = index as usize * stride as usize;
                    if off + 8 > pool.entries().len() {
                        continue; // past the store: the lift panics (host-pinned).
                    }
                    if flags[index as usize] & DEAD != 0 {
                        continue; // dead: both sides fault (host pins the lift panic).
                    }
                    let got = unsafe { fn_008E0210::rw_008e0210(index) };
                    let lift = pool.data_word(index);
                    assert_eq!(got, lift, "n={n} index={index} stride={stride}");
                    if wrong::data_head(&pool, index) != lift {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        // One pinned case where head and word differ on purpose.
        {
            let entries = vec![
                0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, // head, then word
                0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44,
            ];
            let pool = SlotPool::from_parts(entries.clone(), vec![0x00, 0x00], 8);
            let entries_box = entries.into_boxed_slice();
            let flags_box = vec![0x00u8, 0x00].into_boxed_slice();
            let mut ctx = Box::new([0u8; 28]);
            put_u32(&mut ctx[..], 0x00, addr(&entries_box[0]));
            put_u32(&mut ctx[..], 0x04, addr(&flags_box[0]));
            put_u32(&mut ctx[..], 0x0c, 8);
            let ctx_addr = addr(&ctx[0]);
            unsafe { rt::global::<u32>(CTX_VA).write(ctx_addr) };
            for index in 0..2 {
                let got = unsafe { fn_008E0210::rw_008e0210(index) };
                assert_eq!(got, pool.data_word(index));
                assert_ne!(wrong::data_head(&pool, index), got);
                caught += 1;
                cases += 1;
            }
            std::hint::black_box((&entries_box, &flags_box, &ctx));
            let _ = ctx_addr;
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong data offset never caught ({cases} cases)");
    }

    #[test]
    fn assign_matches() {
        let _guard = lock();
        clear_logs();
        rt::set_callee(1, notify_stub as usize as u32);
        let mut rng = Rng(0xA551);
        let mut cases = 0;
        let mut caught = 0;
        let values = [
            0u32,
            1,
            2,
            0xFF,
            0x100,
            0x7FFF_FFFF,
            0x8000_0000,
            0xFFFF_FFFF,
        ];
        for &(n, stride) in &[(1usize, 4u32), (3, 8), (8, 16), (9, 12)] {
            for pattern in 0..4 {
                let flags: Vec<u8> = (0..n)
                    .map(|i| match pattern {
                        0 => 0x00,
                        1 => {
                            if i % 2 == 0 {
                                0x00
                            } else {
                                0x80
                            }
                        }
                        2 => 0x40,
                        _ => rng.u32() as u8,
                    })
                    .collect();
                for index in 0..n as u32 {
                    if flags[index as usize] & DEAD != 0 {
                        continue; // dead: both sides fault (host pins the lift panic).
                    }
                    for &value in &values {
                        let (_, mut pool) = plant_pool(n, stride, &flags, &mut rng, CTX_VA);
                        // The rewrite works its own boxes; find them back
                        // through the context the planter published.
                        let ctx_addr = unsafe { rt::global::<u32>(CTX_VA).read() };
                        let base = unsafe { get_u32_at(ctx_addr) };
                        let before = NOTIFY_LOG.lock().unwrap().len();
                        let got = unsafe { fn_008DFEA0::rw_008dfea0(index, value) };
                        let rw_log = NOTIFY_LOG.lock().unwrap()[before..].to_vec();
                        let mut lift_log = Vec::new();
                        let lift = pool.assign(index, value, &mut |i| lift_log.push(i));
                        assert_eq!(got, u32::from(lift), "n={n} index={index} value={value:#x}");
                        assert_eq!(rw_log, lift_log, "notify log must match");
                        // Every entry byte the rewrite wrote must match.
                        let off = index as usize * stride as usize;
                        let rw_head = unsafe { get_u32_at(base.wrapping_add(off as u32)) };
                        assert_eq!(rw_head, value);
                        assert_eq!(&pool.entries()[off..off + 4], &value.to_le_bytes());
                        // Wrong notifier direction.
                        let (_, mut wpool) = plant_pool(n, stride, &flags, &mut rng, CTX_VA);
                        let mut wlog = Vec::new();
                        wrong::assign_notify_zero(&mut wpool, index, value, &mut wlog);
                        if wlog != rw_log {
                            caught += 1;
                        }
                        cases += 1;
                    }
                }
            }
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong notify direction never caught ({cases} cases)"
        );
    }

    /// Reads one word from test memory the rewrite wrote.
    unsafe fn get_u32_at(addr: u32) -> u32 {
        unsafe { (addr as *const u32).read_unaligned() }
    }

    #[test]
    fn slot_at_offset_matches() {
        let _guard = lock();
        clear_logs();
        let mut rng = Rng(0x0FF5);
        let mut cases = 0;
        let mut caught = 0;
        for &(n, stride) in &[
            (1usize, 1u32),
            (2, 3),
            (3, 4),
            (5, 5),
            (8, 4),
            (8, 7),
            (9, 13),
            (16, 16),
            (16, 32),
        ] {
            let end = (n as u32 - 1).wrapping_mul(stride);
            let mut patterns: Vec<Vec<u8>> = vec![
                vec![0x00; n],
                vec![0x80; n],
                vec![0x40; n],
                (0..n)
                    .map(|i| if i % 2 == 0 { 0x00 } else { 0x80 })
                    .collect(),
            ];
            for _ in 0..2 {
                let mut p = vec![0u8; n];
                rng.bytes(&mut p);
                patterns.push(p);
            }
            for flags in &patterns {
                let mut entries = vec![0u8; n * stride as usize];
                rng.bytes(&mut entries);
                let pool = SlotPool::from_parts(entries.clone(), flags.clone(), stride);
                let entries_box = entries.into_boxed_slice();
                let flags_box = flags.clone().into_boxed_slice();
                let base = addr(&entries_box[0]);
                let desc = Box::new([base, addr(&flags_box[0]), n as u32, stride]);
                let this = addr(&desc[0]);
                let mut offsets = vec![
                    0u32,
                    1,
                    2,
                    stride.wrapping_sub(1),
                    stride,
                    stride.wrapping_add(1),
                    stride.wrapping_mul(2),
                    end.wrapping_sub(1),
                    end,
                    end.wrapping_add(1),
                    end.wrapping_add(stride),
                    0x7FFF_FFFF,
                    0x8000_0000,
                    0xFFFF_FFFE,
                    0xFFFF_FFFF,
                ];
                for _ in 0..16 {
                    offsets.push(rng.below(end.wrapping_add(stride).wrapping_add(1)));
                }
                for _ in 0..8 {
                    offsets.push(rng.u32());
                }
                for &offset in &offsets {
                    let val = base.wrapping_add(offset);
                    let got = unsafe { fn_00A8ACE0::rw_00a8ace0(this, val) };
                    let lift = pool.slot_at_offset(offset);
                    assert_eq!(
                        got & 1,
                        u32::from(lift.is_some()),
                        "n={n} stride={stride} offset={offset:#x}"
                    );
                    match lift {
                        Some(i) => assert_eq!(
                            got,
                            ((i as u32) & 0xFFFF_FF00) | 1,
                            "index residue must match"
                        ),
                        None if offset > end => {
                            assert_eq!(got, val & 0xFFFF_FF00, "range residue must match")
                        }
                        None => assert_eq!(
                            got,
                            (offset / stride) & 0xFFFF_FF00,
                            "quotient residue must match"
                        ),
                    }
                    if wrong::slot_bit6(&pool, offset) != lift {
                        caught += 1;
                    }
                    cases += 1;
                }
                std::hint::black_box((&desc, &entries_box, &flags_box));
            }
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong lookup bit never caught ({cases} cases)");
    }

    #[test]
    fn indexed_store_matches() {
        let _guard = lock();
        clear_logs();
        rt::set_callee(1, refresh_stub as usize as u32);
        let mut rng = Rng(0x1D3);
        let mut cases = 0;
        let mut caught = 0;
        // Distinct cell contents pin the cell index exactly.
        let table: Vec<u32> = (0..512u32)
            .map(|i| i.wrapping_mul(0x9E37_79B1).wrapping_add(0x1234_5678))
            .collect();
        let table_box = table.clone().into_boxed_slice();
        let table_addr = addr(&table_box[0]);
        *REFRESH_TABLE.lock().unwrap() = table_addr;
        for &(n, stride) in &[(2usize, 16u32), (4, 20), (8, 16), (9, 32)] {
            for &scale in &[0u32, 1, 2, 3, 5, 8] {
                unsafe { rt::global::<u32>(SCALE_VA).write(scale) };
                for next in [0xFFFF_FFFFu32, 0, 1, 7, 0x7FFF_FFFF, 0xFFFF_FFFE, rng.u32()] {
                    for row in [0u32, 1, 7, 22, 30, 40] {
                        let flags = vec![0x00; n];
                        let (ctx_addr, pool) = plant_pool(n, stride, &flags, &mut rng, CTX_VA);
                        let base = unsafe { get_u32_at(ctx_addr) };
                        // Successor at entry + 0x0c of slot 1.
                        let slot_off = stride as usize;
                        unsafe {
                            (base.wrapping_add(slot_off as u32 + 12) as *mut u32)
                                .write_unaligned(next);
                        }
                        let mut entries = pool.entries().to_vec();
                        entries[slot_off + 12..slot_off + 16].copy_from_slice(&next.to_le_bytes());
                        let pool = SlotPool::from_parts(entries, flags, stride);
                        // No-wrap check: the proof's address arithmetic
                        // must stay below 4G (see the registry).
                        let answer = table_addr.wrapping_add(row.wrapping_mul(4));
                        let total = scale as u64 * 100 + answer as u64 + 0x58;
                        assert!(total < 0x1_0000_0000, "test layout wrapped");
                        // Script the row only when the refresh runs: a
                        // terminal successor leaves the queue untouched.
                        if next != 0xFFFF_FFFF {
                            REFRESH_SCRIPT.lock().unwrap().push_back(row);
                        }
                        let before = REFRESH_LOG.lock().unwrap().len();
                        // The rewrite writes through the address, invisibly
                        // to the borrow checker: pass and read it raw so the
                        // compiler cannot assume the word is unchanged.
                        let mut out_rw = 0xA11CE_u32;
                        let out_addr = std::ptr::addr_of_mut!(out_rw) as usize as u32;
                        let got = unsafe { fn_008E0880::rw_008e0880(1, out_addr) };
                        out_rw = unsafe { get_u32_at(out_addr) };
                        let rw_calls = REFRESH_LOG.lock().unwrap().len() - before;
                        let mut lift_rows = vec![row];
                        let mut out_lift = 0xA11CE_u32;
                        let lift = pool.indexed_store(
                            1,
                            scale,
                            &table,
                            &mut || lift_rows.remove(0),
                            &mut out_lift,
                        );
                        assert_eq!(
                            got,
                            u32::from(lift),
                            "scale={scale} next={next:#x} row={row}"
                        );
                        assert_eq!(out_rw, out_lift, "out bytes must match");
                        if next == 0xFFFF_FFFF {
                            assert_eq!(rw_calls, 0, "no refresh past the end");
                            assert_eq!(out_rw, 0xA11CE, "out untouched past the end");
                        } else {
                            assert_eq!(rw_calls, 1, "exactly one refresh");
                            assert_eq!(REFRESH_LOG.lock().unwrap()[before], ctx_addr);
                            let expect_cell =
                                table[(scale * 25 + row + 0x16) as usize].wrapping_add(next);
                            assert_eq!(out_lift, expect_cell);
                            let wrong_cell =
                                table[wrong::cell_no_base(scale, row) as usize].wrapping_add(next);
                            if wrong_cell != out_rw {
                                caught += 1;
                            }
                        }
                        cases += 1;
                    }
                }
            }
        }
        std::hint::black_box(&table_box);
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong table base never caught ({cases} cases)");
    }

    #[test]
    fn release_matches() {
        let _guard = lock();
        clear_logs();
        rt::set_callee(1, survives_stub as usize as u32);
        rt::set_callee(2, evict_stub as usize as u32);
        let mut rng = Rng(0x9E1E);
        let mut cases = 0;
        let mut caught = 0;
        let rcs = [
            0u32,
            1,
            2,
            3,
            0x7FFF_FFFF,
            0x8000_0000,
            0x8000_0001,
            0xFFFF_FFFE,
            0xFFFF_FFFF,
        ];
        let auxes = [0u32, 1, 0x1234_5678, 0xFFFF_FFFF];
        // Survival answers span full-word/low-byte agreement (0, 1, MAX)
        // and disagreement (0x100, 0xFF00: nonzero with a zero low byte).
        let survivals = [0u32, 1, 2, 0xFF, 0x100, 0x101, 0x1FF, 0xFF00, 0xFFFF_FFFF];
        for &(n, stride) in &[(2usize, 16u32), (4, 8), (5, 20)] {
            for pattern in 0..3 {
                let flags: Vec<u8> = (0..n)
                    .map(|i| match pattern {
                        0 => 0x00,
                        1 => {
                            if i % 2 == 0 {
                                0x00
                            } else {
                                0x80
                            }
                        }
                        _ => rng.u32() as u8,
                    })
                    .collect();
                for index in 0..n as u32 {
                    for &rc in &rcs {
                        for &aux in &auxes {
                            for &surv in &survivals {
                                let mut entries = vec![0u8; n * stride as usize];
                                rng.bytes(&mut entries);
                                let off = index as usize * stride as usize;
                                entries[off + 4..off + 8].copy_from_slice(&rc.to_le_bytes());
                                let (mgr_addr, mut pool) =
                                    plant_pool_entries(n, stride, &flags, entries, MGR_VA);
                                unsafe { rt::global::<u32>(AUX_VA).write(aux) };
                                let base = unsafe { get_u32_at(mgr_addr) };
                                // The survives answer is scripted only when
                                // the count falls to zero or below (signed).
                                let falls = (rc.wrapping_sub(1) as i32) <= 0;
                                let live = flags[index as usize] & DEAD == 0;
                                if live && falls {
                                    SURVIVES_SCRIPT.lock().unwrap().push_back(surv);
                                }
                                let s_before = SURVIVES_LOG.lock().unwrap().len();
                                let e_before = EVICT_LOG.lock().unwrap().len();
                                let got = unsafe { fn_00BE7A70::rw_00be7a70(index) };
                                let rw_survives = SURVIVES_LOG.lock().unwrap()[s_before..].to_vec();
                                let rw_evicts = EVICT_LOG.lock().unwrap()[e_before..].to_vec();
                                assert_eq!(got, 0, "release answers nothing");
                                let rw_rc =
                                    unsafe { get_u32_at(base.wrapping_add(off as u32 + 4)) };
                                let mut lift_survives = Vec::new();
                                let mut lift_evicts = Vec::new();
                                pool.release(
                                    index,
                                    aux,
                                    &mut |i, a| {
                                        lift_survives.push((i, a));
                                        (surv & 0xFF) != 0
                                    },
                                    &mut |i| lift_evicts.push(i),
                                );
                                assert_eq!(rw_survives, lift_survives, "survives log");
                                assert_eq!(rw_evicts, lift_evicts, "evict log");
                                let lift_rc = u32::from_le_bytes(
                                    pool.entries()[off + 4..off + 8].try_into().unwrap(),
                                );
                                assert_eq!(rw_rc, lift_rc, "refcount bytes");
                                if !live {
                                    assert!(rw_survives.is_empty() && rw_evicts.is_empty());
                                    assert_eq!(rw_rc, rc, "dead slots untouched");
                                }
                                if live
                                    && falls
                                    && wrong::survives_full_word(surv) != ((surv & 0xFF) != 0)
                                {
                                    caught += 1;
                                }
                                cases += 1;
                            }
                        }
                    }
                }
            }
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong survives rule never caught ({cases} cases)"
        );
    }
}
