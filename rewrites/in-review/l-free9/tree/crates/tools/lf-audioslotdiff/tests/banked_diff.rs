//! Differential cases, part 1: the banked voice slots.
//!
//! Each case plants the slot object, the row-pointer table and (for the
//! probe) the node buffers, runs the rewrite and the lift on the same
//! inputs, and compares returns, every written byte and every callee
//! call in order, rebuilding each 32-bit node address from the lifted
//! key per case. Each method has a deliberately wrong lift that must be
//! caught. 32-bit target only.
//!
//! All trials stay inside the planted stores; out-of-range indexes are
//! the lift's documented panic domain (host tests), never run here.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::audio_slot::banked::{
        BankedSlots, NO_SLOT, ROW_SLOT, ROW_STRIDE, VoiceBankFile, VoiceNode,
    };
    use lf_audioslotdiff::rewrites::*;
    use lf_audioslotdiff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{TABLE_VA, STRIDE_VA, Rng, addr, get_u32, lock, put_u32};

    /// Object bytes: covers the bank, slots, parameter and indexed byte.
    const OBJ: usize = 0xC0;
    /// Modelled slot bytes (`+0x48` onwards).
    const SLOTS: usize = 4;
    /// Banks covered by the planted row table.
    const BANKS: u32 = 4;
    /// Row-table bytes: last row word plus one word.
    const ROWTAB: usize = 3 * 0x6F40 + 0x6F10 + 4;

    /// Fresh view of a test image; rebuilt after every rewrite call so no
    /// pre-call borrow is read back (the compiler would forward it).
    unsafe fn image(base: u32, len: usize) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(base as *mut u8, len) }
    }

    /// Plants the row-table buffer with `rows` at the bank offsets.
    fn plant_rows(buf: &mut [u8], rows: &[u32]) {
        for (b, row) in rows.iter().enumerate() {
            let off = b as u32 * ROW_STRIDE + ROW_SLOT;
            put_u32(buf, off as usize, *row);
        }
    }

    /// Builds the lift's object from the planted image.
    fn lift_obj(initial: &[u8]) -> BankedSlots {
        BankedSlots::from_parts(
            initial[0x40],
            initial[0x48..0x48 + SLOTS].to_vec(),
            get_u32(initial, 0x54),
            initial[0xB0],
        )
    }

    /// Builds the lift's bank file from the planted values.
    fn lift_file(stride: u32, table: u32, rows: Vec<u32>) -> VoiceBankFile {
        VoiceBankFile::from_parts(stride, table, rows)
    }

    #[test]
    fn lookup_store_matches() {
        let _guard = lock();
        let mut rng = Rng(0xACC0);
        let mut caught = 0;
        // Strides tried: unit, small, pool-like, large, random, zero
        // (zero only with a null value: the early path divides nothing).
        for trial in 0..64u32 {
            let bank = rng.below(BANKS) as u8;
            let index = rng.below(SLOTS as u32);
            let stride = match trial % 7 {
                0 => 1,
                1 => 2,
                2 => 0x70,
                3 => 0x100,
                4 => 0,
                5 => rng.u32() | 1,
                _ => rng.u32(),
            };
            let row = if trial % 3 == 0 { 0 } else { rng.u32() };
            let mut rows = vec![0u32; BANKS as usize];
            rows[bank as usize] = row;
            let value = match trial % 6 {
                0 => 0,
                1 => row.wrapping_add(rng.below(300).wrapping_mul(stride)),
                2 => row.wrapping_sub(1),
                3 => rng.u32(),
                4 => 0,
                _ => row,
            };
            if stride == 0 && value != 0 {
                continue; // the divide faults; the host suite pins the panic
            }
            let mut initial = vec![0u8; OBJ];
            rng.bytes(&mut initial);
            initial[0x40] = bank;
            let mut rowtab = vec![0u8; ROWTAB];
            rng.bytes(&mut rowtab);
            plant_rows(&mut rowtab, &rows);
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let table_box: Box<[u8]> = rowtab.into_boxed_slice();
            let table = addr(&table_box[0]);
            rt::set_global(TABLE_VA, table);
            rt::set_global(STRIDE_VA, stride);
            let ret = unsafe { fn_0088ACC0::rw_0088acc0(this as *mut u8, index, value) };
            assert_eq!(ret, index, "trial {trial}: answer is the index");
            let mut lift = lift_obj(&initial);
            let file = lift_file(stride, table, rows.clone());
            let out = lift.lookup_store(&file, index, value);
            assert_eq!(out, index, "trial {trial}: lift answers the index");
            let after = unsafe { image(this, OBJ) }.to_vec();
            let mut expect = initial.clone();
            expect[0x48 + index as usize] = if value == 0 {
                NO_SLOT
            } else {
                // stride is nonzero here (stride 0 joins the null-value path).
                value.wrapping_sub(row).wrapping_div(stride) as u8
            };
            assert_eq!(after, expect, "trial {trial}: stored byte matches");
            // Wrong version: divide the raw value, forgetting the row base.
            let wrong = if value == 0 {
                NO_SLOT
            } else {
                value.wrapping_div(stride) as u8
            };
            if wrong != expect[0x48 + index as usize] {
                caught += 1;
            }
            let _ = table_box;
        }
        assert!(caught > 0, "forget-row-base mutant was never caught");
    }

    #[test]
    fn node_lookup_matches() {
        let _guard = lock();
        let mut rng = Rng(0x0410);
        let mut caught = 0;
        for trial in 0..48u32 {
            let bank = rng.below(BANKS) as u8;
            let idx = rng.below(SLOTS as u32);
            let stride = if trial % 4 == 0 { 0 } else { rng.u32() };
            let mut rows = vec![0u32; BANKS as usize];
            for r in rows.iter_mut() {
                *r = rng.u32();
            }
            if trial % 5 == 0 {
                rows[bank as usize] = 0;
            }
            let mut initial = vec![0u8; OBJ];
            rng.bytes(&mut initial);
            initial[0x40] = bank;
            // Empty, near-empty and live slots all appear.
            initial[0x48 + idx as usize] = match trial % 5 {
                0 => NO_SLOT,
                1 => 0xFE,
                2 => 0,
                _ => rng.below(0xFE) as u8,
            };
            let mut rowtab = vec![0u8; ROWTAB];
            rng.bytes(&mut rowtab);
            plant_rows(&mut rowtab, &rows);
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let table_box: Box<[u8]> = rowtab.into_boxed_slice();
            let table = addr(&table_box[0]);
            rt::set_global(TABLE_VA, table);
            rt::set_global(STRIDE_VA, stride);
            let ret = unsafe { fn_00890410::rw_00890410(this, idx) };
            let lift = lift_obj(&initial);
            let file = lift_file(stride, table, rows.clone());
            let out = lift.node_lookup(&file, idx);
            let slot = initial[0x48 + idx as usize];
            if slot == NO_SLOT {
                assert_eq!(ret, 0, "trial {trial}: empty slot answers null");
                assert_eq!(out, None, "trial {trial}: lift answers None");
            } else {
                let expect =
                    stride.wrapping_mul(u32::from(slot)).wrapping_add(rows[bank as usize]);
                assert_eq!(ret, expect, "trial {trial}: node address matches");
                assert_eq!(
                    out,
                    Some(VoiceNode {
                        bank,
                        slot
                    }),
                    "trial {trial}: lift answers the key"
                );
                // The translation is proven, not assumed: the address
                // rebuilds from the lifted key.
                assert_eq!(
                    file.node_addr(out.unwrap()),
                    expect,
                    "trial {trial}: address rebuilds from the key"
                );
            }
            let after = unsafe { image(this, OBJ) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the lookup writes nothing");
            // Wrong version: 0xFE counts as empty too.
            let wrong_none = slot == NO_SLOT || slot == 0xFE;
            if wrong_none != out.is_none() {
                caught += 1;
            }
            let _ = table_box;
        }
        assert!(caught > 0, "fe-is-empty mutant was never caught");
    }

    #[test]
    fn op_forward_matches() {
        let _guard = lock();
        let mut rng = Rng(0xDD40);
        let mut caught = 0;
        for trial in 0..48u32 {
            let bank = rng.below(BANKS) as u8;
            let a1 = rng.u32();
            let answer = rng.u32();
            // Slot, stride and row: empty, null-target and live mixes.
            let (slot0, stride, row) = match trial % 6 {
                0 => (NO_SLOT, rng.u32(), rng.u32()),
                1 => (0, rng.u32(), 0), // null target via zero row
                2 => {
                    // null target via wrap: stride + row == 0
                    let s = rng.u32() | 1;
                    (1, s, s.wrapping_neg())
                }
                3 => (0, 0, rng.u32()),
                _ => (rng.below(0xFE) as u8, rng.u32(), rng.u32()),
            };
            let mut rows = vec![0u32; BANKS as usize];
            for r in rows.iter_mut() {
                *r = rng.u32();
            }
            rows[bank as usize] = row;
            let mut initial = vec![0u8; OBJ];
            rng.bytes(&mut initial);
            initial[0x40] = bank;
            initial[0x48] = slot0;
            let mut rowtab = vec![0u8; ROWTAB];
            rng.bytes(&mut rowtab);
            plant_rows(&mut rowtab, &rows);
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let table_box: Box<[u8]> = rowtab.into_boxed_slice();
            let table = addr(&table_box[0]);
            rt::set_global(TABLE_VA, table);
            rt::set_global(STRIDE_VA, stride);
            rt::set_script(&[(1, StubKind::Thiscall2, vec![answer])]);
            let ret = unsafe { fn_0089DD40::rw_0089dd40(this, a1) };
            let calls = rt::take_calls();
            let lift = lift_obj(&initial);
            let file = lift_file(stride, table, rows.clone());
            let mut seen: Vec<(VoiceNode, u32)> = Vec::new();
            let out = lift.op_forward(&file, a1, &mut |node: VoiceNode, v: u32| {
                seen.push((node, v));
                answer
            });
            let target = row.wrapping_add(stride.wrapping_mul(u32::from(slot0)));
            if slot0 == NO_SLOT || target == 0 {
                assert_eq!(ret, 0, "trial {trial}: early path answers 0");
                assert_eq!(out, 0, "trial {trial}: lift answers 0");
                assert!(calls.is_empty(), "trial {trial}: no call runs");
                assert!(seen.is_empty(), "trial {trial}: lift calls nothing");
            } else {
                assert_eq!(ret, answer, "trial {trial}: answer matches");
                assert_eq!(out, answer, "trial {trial}: lift answers it too");
                assert_eq!(calls, vec![(1, vec![target, a1])], "trial {trial}: one op call");
                assert_eq!(seen.len(), 1, "trial {trial}: lift calls once");
                assert_eq!(
                    file.node_addr(seen[0].0),
                    target,
                    "trial {trial}: address rebuilds from the key"
                );
                assert_eq!(seen[0].1, a1, "trial {trial}: argument matches");
            }
            let after = unsafe { image(this, OBJ) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the forward writes nothing");
            // Wrong version: the null-target check dropped.
            let wrong_calls = u32::from(slot0 != NO_SLOT);
            if (seen.len() as u32) != wrong_calls && target == 0 && slot0 != NO_SLOT {
                caught += 1;
            }
            let _ = table_box;
        }
        assert!(caught > 0, "drop-null-check mutant was never caught");
    }

    #[test]
    fn retrigger_matches() {
        let _guard = lock();
        let mut rng = Rng(0xD4B0);
        let mut caught = 0;
        for trial in 0..48u32 {
            let bank = rng.below(BANKS) as u8;
            let a1 = rng.u32();
            let param = rng.u32();
            let chained = rng.u32();
            let (slot0, stride, row) = match trial % 6 {
                0 => (NO_SLOT, rng.u32(), rng.u32()),
                1 => (0, rng.u32(), 0),
                2 => {
                    let s = rng.u32() | 1;
                    (1, s, s.wrapping_neg())
                }
                3 => (0, 0, 0),
                _ => (rng.below(0xFE) as u8, rng.u32(), rng.u32()),
            };
            let mut rows = vec![0u32; BANKS as usize];
            for r in rows.iter_mut() {
                *r = rng.u32();
            }
            rows[bank as usize] = row;
            let mut initial = vec![0u8; OBJ];
            rng.bytes(&mut initial);
            initial[0x40] = bank;
            initial[0x48] = slot0;
            put_u32(&mut initial, 0x54, param);
            let mut rowtab = vec![0u8; ROWTAB];
            rng.bytes(&mut rowtab);
            plant_rows(&mut rowtab, &rows);
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let table_box: Box<[u8]> = rowtab.into_boxed_slice();
            let table = addr(&table_box[0]);
            rt::set_global(TABLE_VA, table);
            rt::set_global(STRIDE_VA, stride);
            rt::set_script(&[
                (1, StubKind::Thiscall3, vec![0]),
                (2, StubKind::Thiscall2, vec![0]),
                (3, StubKind::Thiscall1, vec![chained]),
            ]);
            let ret = unsafe { fn_0089D4B0::rw_0089d4b0(this, a1) };
            let calls = rt::take_calls();
            let lift = lift_obj(&initial);
            let file = lift_file(stride, table, rows.clone());
            struct Ops {
                log: Vec<(u8, VoiceNode, u32)>,
                chained: u32,
            }
            impl lf_audio::audio_slot::banked::Retrigger for Ops {
                fn setup(&mut self, node: VoiceNode, p: u32) {
                    self.log.push((1, node, p));
                }
                fn retrigger(&mut self, node: VoiceNode, v: u32) {
                    self.log.push((2, node, v));
                }
                fn chain(&mut self) -> u32 {
                    self.chained
                }
            }
            let mut ops = Ops {
                log: Vec::new(),
                chained,
            };
            let out = lift.retrigger(&file, a1, &mut ops);
            let target = row.wrapping_add(stride.wrapping_mul(u32::from(slot0)));
            if slot0 == NO_SLOT {
                assert_eq!(ret, 0xFF, "trial {trial}: empty slot answers 0xFF");
                assert_eq!(out, 0xFF, "trial {trial}: lift answers 0xFF");
                assert!(calls.is_empty(), "trial {trial}: no call runs");
            } else if target == 0 {
                assert_eq!(ret, table, "trial {trial}: null target answers the table");
                assert_eq!(out, table, "trial {trial}: lift answers the table");
                assert!(calls.is_empty(), "trial {trial}: no call runs");
            } else {
                assert_eq!(ret, chained, "trial {trial}: chain answer matches");
                assert_eq!(out, chained, "trial {trial}: lift answers it too");
                assert_eq!(
                    calls,
                    vec![
                        (1, vec![target, param, 0]),
                        (2, vec![target, a1]),
                        (3, vec![this]),
                    ],
                    "trial {trial}: setup, retrigger, chain in order"
                );
                assert_eq!(ops.log.len(), 2, "trial {trial}: lift logs two node calls");
                for (id, node, v) in &ops.log {
                    assert_eq!(
                        file.node_addr(*node),
                        target,
                        "trial {trial}: call {id} address rebuilds"
                    );
                    let _ = v;
                }
                assert_eq!(ops.log[0], (1, ops.log[0].1, param));
                assert_eq!(ops.log[1], (2, ops.log[1].1, a1));
            }
            let after = unsafe { image(this, OBJ) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the retrigger writes nothing");
            // Wrong version: the chain answer dropped, 0 returned.
            if out != 0 && chained != 0 && slot0 != NO_SLOT && target != 0 {
                caught += 1;
            }
            let _ = table_box;
        }
        assert!(caught > 0, "drop-chain-answer mutant was never caught");
    }

    #[test]
    fn probe_matches() {
        let _guard = lock();
        let mut rng = Rng(0x0DC0);
        let mut caught = 0;
        // Node stride: room for the tag half-word and the live byte.
        for trial in 0..40u32 {
            let stride = if trial % 2 == 0 { 0x80 } else { 0x100 };
            let bank = rng.below(BANKS) as u8;
            let indexed_byte = rng.below(0xFF) as u8;
            let slot0 = match trial % 6 {
                0 => NO_SLOT,
                1 => 0,
                2 => 2,
                _ => rng.below(0xFF) as u8,
            };
            let slot1 = match trial % 6 {
                0 => 1,
                1 => NO_SLOT,
                2 => rng.below(0xFF) as u8,
                _ => rng.below(0xFF) as u8,
            };
            // Node buffers: one row's worth of slots at the stride.
            let node_len = 0xFFusize * stride as usize;
            let mut nodes = vec![0u8; node_len];
            rng.bytes(&mut nodes);
            // Plant liveness and tags for the three bytes in play.
            let live = trial % 3 != 0;
            nodes[indexed_byte as usize * stride as usize + 0x72] = u8::from(live);
            let tag = |slot: u8| -> u16 {
                if slot == slot0 {
                    [1, 2, 3, 0][trial as usize % 4]
                } else if slot == slot1 {
                    [2, 1, 0, 5][trial as usize % 4]
                } else {
                    7
                }
            };
            for s in [slot0, slot1] {
                if s != NO_SLOT {
                    let at = s as usize * stride as usize + 6;
                    nodes[at..at + 2].copy_from_slice(&tag(s).to_le_bytes());
                }
            }
            // Offer answers: low-byte edges (0x100 latches nothing).
            let offer0 = [0x100u32, 1, 0, 0x1FF][trial as usize % 4];
            let offer1 = [0u32, 0x100, 1, 0x200][trial as usize % 4];
            let mut rows = vec![0u32; BANKS as usize];
            for r in rows.iter_mut() {
                *r = rng.u32();
            }
            let mut rowtab = vec![0u8; ROWTAB];
            rng.bytes(&mut rowtab);
            let mut initial = vec![0u8; OBJ];
            rng.bytes(&mut initial);
            initial[0x40] = bank;
            initial[0x48] = slot0;
            initial[0x49] = slot1;
            initial[0xB0] = indexed_byte;
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let node_box: Box<[u8]> = nodes.clone().into_boxed_slice();
            let node_base = addr(&node_box[0]);
            rows[bank as usize] = node_base;
            plant_rows(&mut rowtab, &rows);
            let table_box: Box<[u8]> = rowtab.into_boxed_slice();
            let table = addr(&table_box[0]);
            rt::set_global(TABLE_VA, table);
            rt::set_global(STRIDE_VA, stride);
            rt::set_script(&[
                (1, StubKind::Thiscall2, vec![0]),
                (2, StubKind::Stdcall1, vec![0]),
                (3, StubKind::Thiscall2, vec![offer0, offer1]),
            ]);
            let arg0 = rng.u32();
            let ret = unsafe { fn_008A0DC0::rw_008A0DC0(this as *mut u8, arg0) };
            let calls = rt::take_calls();
            let lift = lift_obj(&initial);
            let file = lift_file(stride, table, rows.clone());
            struct Fake<'a> {
                nodes: &'a [u8],
                stride: u32,
                offers: Vec<u32>,
                log: Vec<(u8, VoiceNode, u32)>,
            }
            impl lf_audio::audio_slot::banked::Probe for Fake<'_> {
                fn entry_live(&mut self, node: VoiceNode) -> bool {
                    self.nodes[node.slot as usize * self.stride as usize + 0x72] != 0
                }
                fn setup_this(&mut self, node: VoiceNode) {
                    self.log.push((1, node, 0));
                }
                fn setup_entry(&mut self, node: VoiceNode) {
                    self.log.push((2, node, 0));
                }
                fn entry_tag(&mut self, node: VoiceNode) -> u16 {
                    let at = node.slot as usize * self.stride as usize + 6;
                    u16::from_le_bytes(self.nodes[at..at + 2].try_into().unwrap())
                }
                fn offer(&mut self, node: VoiceNode, v: u32) -> u32 {
                    self.log.push((3, node, v));
                    let ans = self.offers.remove(0);
                    ans
                }
            }
            let mut fake = Fake {
                nodes: &nodes,
                stride,
                offers: vec![offer0, offer1],
                log: Vec::new(),
            };
            let out = lift.probe(&file, arg0, &mut fake);
            assert_eq!(ret, u32::from(out), "trial {trial}: answer matches");
            // Expected call log, rebuilt from the planted values.
            let iaddr = node_base.wrapping_add(u32::from(indexed_byte).wrapping_mul(stride));
            assert_ne!(iaddr, 0, "trial {trial}: indexed address stays mapped");
            let mut expect: Vec<(u32, Vec<u32>)> = Vec::new();
            if live {
                expect.push((1, vec![this, iaddr]));
                expect.push((2, vec![iaddr]));
            }
            let mut offer_idx = 0;
            let mut expect_latch = false;
            for (k, s) in [slot0, slot1].iter().enumerate() {
                if *s == NO_SLOT {
                    continue;
                }
                let eaddr = node_base.wrapping_add(u32::from(*s).wrapping_mul(stride));
                if eaddr == 0 {
                    continue;
                }
                let t = tag(*s);
                if t == 2 {
                    let ans = [offer0, offer1][offer_idx];
                    offer_idx += 1;
                    expect.push((3, vec![eaddr, arg0]));
                    if ans & 0xFF != 0 {
                        expect_latch = true;
                    }
                } else if t == 1 {
                    expect_latch = true;
                }
                let _ = k;
            }
            assert_eq!(calls, expect, "trial {trial}: call log matches");
            assert_eq!(out, expect_latch, "trial {trial}: latch matches");
            // The lift's log maps onto the same addresses.
            let mut li = 0;
            for (id, node, v) in &fake.log {
                let want = file.node_addr(*node);
                match *id {
                    1 => {
                        assert_eq!(expect[li], (1, vec![this, want]));
                        li += 1;
                    }
                    2 => {
                        assert_eq!(expect[li], (2, vec![want]));
                        li += 1;
                    }
                    _ => {
                        assert_eq!(expect[li], (3, vec![want, *v]));
                        li += 1;
                    }
                }
            }
            assert_eq!(li, expect.len(), "trial {trial}: lift log length matches");
            let after = unsafe { image(this, OBJ) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the probe writes nothing");
            let nodes_after = unsafe { image(node_base, node_len) }.to_vec();
            assert_eq!(nodes_after, nodes, "trial {trial}: node bytes unchanged");
            // Wrong version: the offer latches on the full word, not the
            // low byte (0x100 must not latch).
            let full_word_latch = (tag(slot0) == 2
                && slot0 != NO_SLOT
                && node_base.wrapping_add(u32::from(slot0).wrapping_mul(stride)) != 0
                && offer0 != 0)
                || (tag(slot1) == 2
                    && slot1 != NO_SLOT
                    && node_base.wrapping_add(u32::from(slot1).wrapping_mul(stride)) != 0
                    && offer1 != 0)
                || (tag(slot0) == 1 && slot0 != NO_SLOT)
                || (tag(slot1) == 1 && slot1 != NO_SLOT);
            // Count only trials where the offer order lines up: both
            // tag-2 slots consume answers in slot order.
            let tag2_count =
                u32::from(tag(slot0) == 2 && slot0 != NO_SLOT) + u32::from(tag(slot1) == 2 && slot1 != NO_SLOT);
            if tag2_count <= 1 && full_word_latch != expect_latch {
                caught += 1;
            }
            let _ = (node_box, table_box);
        }
        assert!(caught > 0, "full-word-offer mutant was never caught");
    }
}
