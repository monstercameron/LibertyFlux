// original: 0x008e7b90 NodeGraph_PathSearch
use lf_checker_rt::{callee_thiscall, export, relocated};

#[inline(always)]
unsafe fn rd_u32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read() }
}

#[inline(always)]
unsafe fn rd_u8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd_u16(addr: u32) -> u16 {
    unsafe { (addr as *const u16).read() }
}

#[inline(always)]
unsafe fn rd_i16(addr: u32) -> i32 {
    unsafe { (addr as *const i16).read() as i32 }
}

#[inline(always)]
unsafe fn wr_u32(addr: u32, val: u32) {
    unsafe { (addr as *mut u32).write(val) }
}

#[inline(always)]
unsafe fn wr_u16(addr: u32, val: u16) {
    unsafe { (addr as *mut u16).write(val) }
}

/// Arguments the entry block pushes for callee 1 (all sites share the
/// shape; only the caller word differs). `frame_word` stands in for the
/// caller's stack slot the original passes; its address is skipped by the
/// contract because each side's frame sits elsewhere.
unsafe fn search_call(handle: u32, caller_word: u32, f24: u32, dval: u32) -> u32 {
    unsafe {
        let slot = [0u32, 0u32, 0u32];
        let ans: u32 = callee_thiscall!(
            1,
            u32,
            handle,
            slot.as_ptr() as u32,
            caller_word,
            f24,
            0,
            0,
            dval,
            0,
            0,
            0,
            0,
            0,
            0x40400000,
            0xFFFFFFFF,
            0
        );
        rd_u32(ans)
    }
}

/// Shared epilogue: report `a20`, publishing the not-found float through
/// it when it is a real pointer.
unsafe fn epilogue(a20: u32) -> u32 {
    unsafe {
        if a20 != 0 {
            wr_u32(a20, 0x47C35000);
        }
        a20
    }
}
/// Search the node graph from a seed entry across one or more slot trips.
///
/// `handle` owns the id tables (`+0x804` entry bases, `+0x904` row-table
/// bases, a 0x200-word slot table at `+0x4`, and the live counter at
/// `+0xE04`). Each of the two ids comes from its argument word unless that
/// word is missing (`a28` null), invalid (`0xFFFF`) or unbacked (null
/// base), in which case callee 1 searches for it. Ids that are still
/// invalid, or equal, or whose entries disagree on the low flag bits, exit
/// early with `*a18 = 0` and report `a20`.
///
/// Past the entry the slot table is cleared, the setup call seeds slot 0,
/// and each trip walks one slot's link list: every link scans its rows,
/// each row passing a five-source latch (`a34` id match, the `a30`/`a4c`
/// tune gate, the `a38` flag-bit gate, the `a44` gate, the `a48` gate)
/// and a kind agreement check before its cost (link base plus row step,
/// with a halved step when untuned) is tested against the target bound.
/// An improving row releases through callee 3 unless already settled,
/// publishes settled ids to the sweep array, and re-links through callee 2.
/// Links release through callee 3 and the walk ends at the null link.
///
/// After each trip the counter exit fires when the live count reads zero;
/// otherwise the trip count is tested against the `a2c` float bound and a
/// set found-latch would divert into the collection (not covered by any
/// stage yet: the latch never sets while no link equals its seed entry).
/// The tail clears `*a18`, settles the sweep word, and reports `a20`.
export!(
    thiscall,
    rw_008e7b90(
        handle: u32,
        a08: u32,
        a0c: u32,
        a10: u32,
        _a14: u32,
        a18: u32,
        _a1c: u32,
        a20: u32,
        a24: u32,
        a28: u32,
        a2c: u32,
        a30: u32,
        a34: u32,
        a38: u32,
        a3c: u32,
        _a40: u32,
        a44: u32,
        a48: u32,
        a4c: u32,
    ) -> u32 {
        unsafe {
            wr_u32(handle.wrapping_add(0xE04), 0);
            let id1: u32;
            if a28 == 0 {
                id1 = search_call(handle, a10, a24, a3c);
            } else {
                let w = rd_u32(a28);
                let slot = w & 0xFFFF;
                if slot == 0xFFFF {
                    id1 = search_call(handle, a10, a24, a3c);
                } else if rd_u32(handle.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x804)) == 0
                {
                    id1 = search_call(handle, a10, a24, a3c);
                } else {
                    id1 = w;
                }
            }
            if (id1 & 0xFFFF) == 0xFFFF {
                wr_u32(a18, 0);
                return epilogue(a20);
            }
            let id2: u32;
            let slot2 = a0c & 0xFFFF;
            if slot2 == 0xFFFF {
                id2 = search_call(handle, a08, a24, a3c);
            } else if rd_u32(handle.wrapping_add(slot2.wrapping_mul(4)).wrapping_add(0x804)) == 0
            {
                id2 = search_call(handle, a08, a24, a3c);
            } else {
                id2 = a0c;
            }
            if (id2 & 0xFFFF) == 0xFFFF {
                wr_u32(a18, 0);
                return epilogue(a20);
            }
            if id1 == id2 {
                wr_u32(a18, 0);
                if a20 != 0 {
                    wr_u32(a20, 0);
                }
                return a20;
            }
            let base_c = rd_u32(handle.wrapping_add((id2 & 0xFFFF).wrapping_mul(4)).wrapping_add(0x804));
            let base_d = rd_u32(handle.wrapping_add((id1 & 0xFFFF).wrapping_mul(4)).wrapping_add(0x804));
            let entry_c = base_c.wrapping_add((id2 >> 16).wrapping_mul(32));
            let entry_d = base_d.wrapping_add((id1 >> 16).wrapping_mul(32));
            if (rd_u8(entry_c.wrapping_add(0x1B)) ^ rd_u8(entry_d.wrapping_add(0x1B))) & 0x1F != 0
            {
                wr_u32(a18, 0);
                return epilogue(a20);
            }
            // Clear the slot table, seed it through the setup call, and
            // run the trip loop over the slots in turn.
            let filter_40: u32 = if rd_u8(handle.wrapping_add(0x1C04)) == 0 {
                a44 & 0xFF
            } else {
                0
            };
            for k in 0..0x200u32 {
                wr_u32(handle.wrapping_add(4).wrapping_add(k.wrapping_mul(4)), 0);
            }
            callee_thiscall!(2, u32, handle, entry_d, 0);
            let gw = rd_u32(entry_d.wrapping_add(8));
            wr_u32(relocated(0x01179890), gw);
            let mut sweep_count: u32 = 1;
            let mut latch_all: u32 = 0;
            let mut trip: u32 = 0;
            let mut slot_idx: u32 = 0;
            loop {
                let slot = rd_u32(handle.wrapping_add(4).wrapping_add(slot_idx.wrapping_mul(4)));
                if slot != 0 {
                    let mut link = slot;
                    loop {
                        if link == entry_c {
                            latch_all = 1;
                        }
                        let row_count = rd_u8(link.wrapping_add(0x1E)) & 0xF;
                        if row_count != 0 {
                            let row_base = rd_u32(
                                handle
                                    .wrapping_add((rd_u16(link.wrapping_add(8)) as u32).wrapping_mul(4))
                                    .wrapping_add(0x904),
                            );
                            let link_off = rd_i16(link.wrapping_add(0x12));
                            let link_base_cost = rd_i16(link.wrapping_add(0x10));
                            let link_flag = rd_u8(link.wrapping_add(0x1F));
                            let link_kind_b = rd_u8(link.wrapping_add(0x1C)) & 0xF0 == 0xB0;
                            let mut row_i: u32 = 0;
                            while row_i < row_count as u32 {
                                let row_at = row_base.wrapping_add(
                                    (link_off.wrapping_add(row_i as i32).wrapping_mul(8)) as u32,
                                );
                                let row_id = rd_u32(row_at);
                                let target_base = rd_u32(
                                    handle
                                        .wrapping_add((row_id & 0xFFFF).wrapping_mul(4))
                                        .wrapping_add(0x804),
                                );
                                if target_base != 0 {
                                    let target = target_base
                                        .wrapping_add((row_id >> 16).wrapping_mul(32));
                                    let tune = (rd_u8(row_at.wrapping_add(5)) >> 3) & 7;
                                    let mut latch: u32 = 0;
                                    if a30 != 0 {
                                        let attempt = a4c == 0
                                            || rd_u8(row_at.wrapping_add(7)) & 0x10 == 0;
                                        if attempt && tune == 0 {
                                            latch = 1;
                                        }
                                    }
                                    if row_id == a34 {
                                        latch = 1;
                                    }
                                    let target_flag = rd_u8(target.wrapping_add(0x1F));
                                    if (link_flag ^ target_flag) & 2 != 0 && a38 == 0 {
                                        latch = 1;
                                    }
                                    if filter_40 != 0 && target_flag & 0x80 != 0 {
                                        latch = 1;
                                    }
                                    if a48 != 0
                                        && target_flag & 0x20 != 0
                                        && link_flag & 0x20 == 0
                                    {
                                        latch = 1;
                                    }
                                    let target_kind_b =
                                        rd_u8(target.wrapping_add(0x1C)) & 0xF0 == 0xB0;
                                    if link_kind_b == target_kind_b && latch == 0 {
                                        let step = rd_u8(row_at.wrapping_add(4));
                                        let mut cost =
                                            link_base_cost.wrapping_add(step as i32);
                                        if tune == 0 {
                                            cost = cost.wrapping_add((step >> 1) as i32);
                                        }
                                        let target_cost =
                                            rd_u16(target.wrapping_add(0x10)) as i32;
                                        if cost < target_cost {
                                            if rd_u16(target.wrapping_add(0x10)) != 0x7FFE {
                                                callee_thiscall!(3, u32, handle, target);
                                            }
                                            if rd_u16(target.wrapping_add(0x10)) == 0x7FFE {
                                                if sweep_count < 0x1388 {
                                                    wr_u32(
                                                        relocated(0x01179890).wrapping_add(
                                                            sweep_count.wrapping_mul(4),
                                                        ),
                                                        rd_u32(target.wrapping_add(8)),
                                                    );
                                                    sweep_count += 1;
                                                }
                                            }
                                            // The call reads its object from the frame slot
                                            // that the two argument pushes re-alias
                                            // onto the handle cell: ECX is the
                                            // handle here, not the seed entry.
                                            callee_thiscall!(2, u32, handle, target, cost as u32);
                                        }
                                    }
                                }
                                row_i += 1;
                            }
                        }
                        callee_thiscall!(3, u32, handle, link);
                        link = rd_u32(link);
                        if link == 0 {
                            break;
                        }
                    }
                }
                trip = trip.wrapping_add(1);
                slot_idx = trip & 0x1FF;
                if rd_u32(handle.wrapping_add(0xE04)) == 0 {
                    break;
                }
                // The trip bound rides in a2c; the original converts
                // the count and exits on ordered greater, which the
                // float comparison below mirrors exactly.
                let bound = f32::from_bits(a2c);
                if trip as f32 > bound {
                    break;
                }
                if latch_all != 0 {
                    panic!("fn2 s4: collection");
                }
            }
            let _ = latch_all;
            wr_u32(a18, 0);
            let base_t = rd_u32(handle.wrapping_add((gw & 0xFFFF).wrapping_mul(4)).wrapping_add(0x804));
            wr_u16(
                base_t.wrapping_add((gw >> 16).wrapping_mul(32)).wrapping_add(0x10),
                0x7FFE,
            );
            return epilogue(a20);
        }
    }
);
