//! Host tests for the lifted slot pools: the edge cases a reader of the
//! code would ask about. These run on the 64-bit host; the differential
//! proof against the verified rewrites lives in `lf-pooldiff`.

use lf_world::pools::{CtxHandle, ElemStamp, PoolVec, SlotPool, registry};

fn pool(flags: &[u8], stride: u32) -> SlotPool {
    SlotPool::from_parts(vec![0u8; flags.len() * stride as usize], flags.to_vec(), stride)
}

#[test]
fn registry_counts_are_pinned() {
    assert_eq!(registry::count(registry::State::Proven), 26);
    assert_eq!(registry::count(registry::State::Lifted), 0);
    assert_eq!(registry::count(registry::State::Missing), 27);
    assert_eq!(registry::ROWS.len(), 53);
}

#[test]
fn create_ctx_alloc_failure_skips_init() {
    let mut init_calls = 0;
    let got = SlotPool::create_ctx(
        0xDEAD,
        0xBEEF,
        &mut |size| {
            assert_eq!(size, 0x1c);
            None
        },
        &mut |_, _, _, _| {
            init_calls += 1;
            CtxHandle::new(1).unwrap()
        },
    );
    assert_eq!(got, None);
    assert_eq!(init_calls, 0);
}

#[test]
fn create_ctx_passes_tag_16() {
    let mut seen = Vec::new();
    let block = CtxHandle::new(0x1000).unwrap();
    let ctx = CtxHandle::new(0x2000).unwrap();
    let got = SlotPool::create_ctx(
        7,
        9,
        &mut |_| Some(block),
        &mut |b: CtxHandle, a0, a1, tag| {
            seen.push((b.get(), a0, a1, tag));
            ctx
        },
    );
    assert_eq!(got.map(|h| h.get()), Some(0x2000));
    assert_eq!(seen, [(0x1000, 7, 9, 16)]);
}

#[test]
fn occupied_only_bit7_kills() {
    let pool = pool(&[0x00, 0x01, 0x40, 0x7F, 0x80, 0x81, 0xC0, 0xFF], 8);
    let live: Vec<bool> = (0..8).map(|i| pool.is_occupied(i)).collect();
    assert_eq!(live, [true, true, true, true, false, false, false, false]);
}

#[test]
#[should_panic(expected = "past 3 slots")]
fn occupied_past_end_panics() {
    pool(&[0, 0, 0], 8).is_occupied(3);
}

#[test]
fn occupied_empty_pool_panics() {
    let r = std::panic::catch_unwind(|| pool(&[], 8).is_occupied(0));
    assert!(r.is_err());
}

#[test]
fn occupied_stride_zero_still_checks_flags() {
    // Every slot aliases the base; the flag byte still decides.
    let pool = pool(&[0x00, 0x80], 0);
    assert!(pool.is_occupied(0));
    assert!(!pool.is_occupied(1));
}

#[test]
fn data_word_reads_entry_plus_4() {
    let entries: Vec<u8> = (0..32u8).collect();
    let pool = SlotPool::from_parts(entries, vec![0x00, 0x00], 16);
    assert_eq!(pool.data_word(0), 0x0706_0504);
    assert_eq!(pool.data_word(1), 0x1716_1514);
}

#[test]
#[should_panic(expected = "dead slot 1")]
fn data_word_dead_panics() {
    let entries: Vec<u8> = (0..32u8).collect();
    SlotPool::from_parts(entries, vec![0x00, 0x80], 16).data_word(1);
}

#[test]
fn data_word_short_stride_bleeds_like_memory() {
    // Stride 4: slot 0's word reaches into slot 1, exactly as addressed memory would.
    let entries: Vec<u8> = (0..12u8).collect();
    let pool = SlotPool::from_parts(entries, vec![0, 0, 0], 4);
    assert_eq!(pool.data_word(0), 0x0706_0504);
    assert_eq!(pool.data_word(1), 0x0B0A_0908);
}

#[test]
fn assign_stores_notifies_and_answers() {
    let mut pool = pool(&[0x00, 0x00], 8);
    let mut log = Vec::new();
    assert!(pool.assign(1, 0xAABB_CCDD, &mut |i| log.push(i)));
    assert_eq!(log, [1]);
    assert_eq!(&pool.entries()[8..12], &[0xDD, 0xCC, 0xBB, 0xAA]);
}

#[test]
fn assign_zero_skips_notify_and_answers_false() {
    let mut pool = pool(&[0x00], 8);
    let mut log = Vec::new();
    assert!(!pool.assign(0, 0, &mut |i| log.push(i)));
    assert!(log.is_empty());
}

#[test]
#[should_panic(expected = "dead slot 0")]
fn assign_dead_panics() {
    pool(&[0x80], 8).assign(0, 1, &mut |_| {});
}

#[test]
fn slot_at_offset_edges() {
    // 4 slots of stride 8: offsets 0, 8, 16, 24 live; end is 24.
    let pool = pool(&[0x00, 0x00, 0x80, 0x00], 8);
    assert_eq!(pool.slot_at_offset(0), Some(0));
    assert_eq!(pool.slot_at_offset(8), Some(1));
    assert_eq!(pool.slot_at_offset(16), None); // dead
    assert_eq!(pool.slot_at_offset(24), Some(3));
    assert_eq!(pool.slot_at_offset(1), None); // misaligned
    assert_eq!(pool.slot_at_offset(25), None); // past the end
    assert_eq!(pool.slot_at_offset(u32::MAX), None);
}

#[test]
fn slot_at_offset_single_slot() {
    let pool = pool(&[0x00], 16);
    assert_eq!(pool.slot_at_offset(0), Some(0));
    assert_eq!(pool.slot_at_offset(16), None);
}

#[test]
fn slot_at_offset_stride_one() {
    let pool = pool(&[0x00, 0x80, 0x00], 1);
    assert_eq!(pool.slot_at_offset(0), Some(0));
    assert_eq!(pool.slot_at_offset(1), None);
    assert_eq!(pool.slot_at_offset(2), Some(2));
    assert_eq!(pool.slot_at_offset(3), None);
}

#[test]
fn slot_at_offset_empty_pool_panics() {
    let r = std::panic::catch_unwind(|| pool(&[], 8).slot_at_offset(0));
    assert!(r.is_err());
}

#[test]
#[should_panic(expected = "faults on the division")]
fn slot_at_offset_stride_zero_panics() {
    pool(&[0x00], 0).slot_at_offset(0);
}

#[test]
fn cursor_restart_and_walk() {
    let pool = pool(&[0x00, 0x80, 0x00, 0x80], 8);
    let mut cursor = -1;
    assert_eq!(pool.cursor_step(&mut cursor), Some(2));
    assert_eq!(cursor, 2);
    assert_eq!(pool.cursor_step(&mut cursor), Some(0));
    assert_eq!(cursor, 0);
    assert_eq!(pool.cursor_step(&mut cursor), None);
    assert_eq!(cursor, -1);
}

#[test]
fn cursor_deeply_negative_restarts() {
    let pool = pool(&[0x00], 8);
    let mut cursor = i32::MIN;
    assert_eq!(pool.cursor_step(&mut cursor), Some(0));
}

#[test]
fn cursor_all_dead_ends_at_top() {
    let pool = pool(&[0x80, 0x80, 0x80], 8);
    let mut cursor = 2;
    assert_eq!(pool.cursor_step(&mut cursor), None);
    assert_eq!(cursor, -1);
}

#[test]
fn cursor_empty_pool() {
    let pool = pool(&[], 8);
    let mut cursor = -1;
    assert_eq!(pool.cursor_step(&mut cursor), None);
    assert_eq!(cursor, -1);
}

#[test]
fn cursor_above_count_panics() {
    let r = std::panic::catch_unwind(|| {
        let pool = pool(&[0x00, 0x00], 8);
        let mut cursor = 3;
        pool.cursor_step(&mut cursor)
    });
    assert!(r.is_err());
}

#[test]
fn cursor_at_count_steps_to_top() {
    let pool = pool(&[0x80, 0x00], 8);
    let mut cursor = 2;
    assert_eq!(pool.cursor_step(&mut cursor), Some(1));
}

#[test]
fn indexed_store_terminal_successor_calls_nothing() {
    let mut entries = vec![0u8; 32];
    entries[12..16].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    let pool = SlotPool::from_parts(entries, vec![0, 0], 16);
    let table = vec![0x1111_1111u32; 64];
    let mut out = 0xBEEF;
    let mut calls = 0;
    let got = pool.indexed_store(0, 0, &table, &mut || {
        calls += 1;
        0
    }, &mut out);
    assert!(!got);
    assert_eq!(calls, 0);
    assert_eq!(out, 0xBEEF);
}

#[test]
fn indexed_store_cell_math() {
    // scale 2, row 3: cell 2*25+3+0x16 = 75; plus successor 5.
    let mut entries = vec![0u8; 32];
    entries[12..16].copy_from_slice(&5u32.to_le_bytes());
    let pool = SlotPool::from_parts(entries, vec![0, 0], 16);
    let table: Vec<u32> = (0..128).collect();
    let mut out = 0;
    let got = pool.indexed_store(0, 2, &table, &mut || 3, &mut out);
    assert!(got);
    assert_eq!(out, 75 + 5);
}

#[test]
fn indexed_store_wraps_cell_plus_successor() {
    let mut entries = vec![0u8; 32];
    entries[12..16].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    // Terminal successor answers false even at MAX; use MAX-1 for the wrap.
    entries[12..16].copy_from_slice(&0xFFFF_FFFEu32.to_le_bytes());
    let pool = SlotPool::from_parts(entries, vec![0, 0], 16);
    let table = vec![7u32; 64];
    let mut out = 0;
    assert!(pool.indexed_store(0, 0, &table, &mut || 0, &mut out));
    assert_eq!(out, 7u32.wrapping_add(0xFFFF_FFFE));
}

#[test]
#[should_panic(expected = "dead slot 0")]
fn indexed_store_dead_panics() {
    let pool = pool(&[0x80], 16);
    let table = vec![0u32; 64];
    let mut out = 0;
    pool.indexed_store(0, 0, &table, &mut || 0, &mut out);
}

#[test]
fn from_parts_rejects_ragged_entries() {
    let r = std::panic::catch_unwind(|| SlotPool::from_parts(vec![0u8; 10], vec![0, 0], 8));
    assert!(r.is_err());
}

fn stamp() -> ElemStamp {
    ElemStamp::new(0xE9AA_BBCC).unwrap()
}

#[test]
fn vec_alloc_size_edges() {
    assert_eq!(PoolVec::alloc_size(0, 0x24), 4);
    assert_eq!(PoolVec::alloc_size(1, 0x24), 0x28);
    assert_eq!(PoolVec::alloc_size(8, 16), 8 * 16 + 4);
    // Multiply overflow saturates.
    assert_eq!(PoolVec::alloc_size(u32::MAX, 0x24), u32::MAX);
    assert_eq!(PoolVec::alloc_size(u32::MAX, 1), u32::MAX);
    // Add carry saturates: count * 1 + 4 overflows.
    assert_eq!(PoolVec::alloc_size(u32::MAX - 3, 1), u32::MAX);
    assert_eq!(PoolVec::alloc_size(u32::MAX - 4, 1), u32::MAX - 4 + 4);
    // Just below the multiply edge.
    let q = (u32::MAX - 4) / 0x24;
    assert_eq!(PoolVec::alloc_size(q, 0x24), q * 0x24 + 4);
    assert_eq!(PoolVec::alloc_size(q + 1, 0x24), u32::MAX);
}

#[test]
fn vec_init_empty_is_prefix_only() {
    let v = PoolVec::init(0, 0x24, stamp(), &mut |size| {
        assert_eq!(size, 4);
        Some(vec![0u8; 4])
    })
    .unwrap();
    assert_eq!(v.buf(), &[0, 0, 0, 0]);
    assert_eq!(v.count(), 0);
    assert_eq!(v.end_offset(), 4);
}

#[test]
fn vec_init_stamps_every_slot() {
    let v = PoolVec::init(3, 16, stamp(), &mut |size| {
        assert_eq!(size, 3 * 16 + 4);
        Some(vec![0u8; size as usize])
    })
    .unwrap();
    assert_eq!(v.count(), 3);
    assert_eq!(v.end_offset(), 3 * 16 + 4);
    for i in 0..3 {
        let off = 4 + i * 16;
        assert_eq!(&v.buf()[off..off + 4], &[0xCC, 0xBB, 0xAA, 0xE9]);
    }
    // Bodies past the stamps are backing as it came.
    assert_eq!(&v.buf()[8..16], &[0; 8]);
}

#[test]
fn vec_init_failure_is_none() {
    let mut sizes = Vec::new();
    let got = PoolVec::init(9, 32, stamp(), &mut |size| {
        sizes.push(size);
        None
    });
    assert_eq!(got, None);
    assert_eq!(sizes, [9 * 32 + 4]);
}

#[test]
fn vec_init_rejects_short_backing() {
    let r = std::panic::catch_unwind(|| {
        PoolVec::init(4, 16, stamp(), &mut |_| Some(vec![0u8; 8]))
    });
    assert!(r.is_err());
}

#[test]
fn vec_init_stride_zero_single_slot() {
    // One slot of stride 0: the stamp would land past the 4-byte
    // buffer, where the original writes past its allocation.
    let r = std::panic::catch_unwind(|| {
        PoolVec::init(1, 0, stamp(), &mut |size| {
            assert_eq!(size, 4);
            Some(vec![0u8; 4])
        })
    });
    assert!(r.is_err());
}

#[test]
fn vec_init_narrow_stride_overruns_like_the_original() {
    // Stride 1 with 8 slots: the last stamp would leave the buffer, where
    // the original writes past its allocation.
    let r = std::panic::catch_unwind(|| {
        PoolVec::init(8, 1, stamp(), &mut |size| Some(vec![0u8; size as usize]))
    });
    assert!(r.is_err());
}
