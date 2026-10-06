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
unsafe fn rd_i16(addr: u32) -> i32 {
    unsafe { (addr as *const i16).read() as i32 }
}

#[inline(always)]
unsafe fn rd_u16(addr: u32) -> u16 {
    unsafe { (addr as *const u16).read() }
}

#[inline(always)]
unsafe fn wr_u32(addr: u32, val: u32) {
    unsafe { (addr as *mut u32).write(val) }
}

#[inline(always)]
unsafe fn wr_u16(addr: u32, val: u16) {
    unsafe { (addr as *mut u16).write(val) }
}

#[inline(always)]
unsafe fn image_f32(file_va: u32) -> f32 {
    unsafe { lf_checker_rt::global::<f32>(file_va).read() }
}

const SLOT_BASE: u32 = 0x804;
const ROW_BASE: u32 = 0x904;
const SCAN_COUNT: u32 = 0x200;
const LIVE_OFF: u32 = 0xE04;
const GATE_OFF: u32 = 0x1C04;
const INVALID: u32 = 0xFFFF;
const SWEEP_CAP: u32 = 0x1388;
const FRESH_WORD: u16 = 0x7FFE;
const NOT_FOUND_BITS: u32 = 0x47C35000;
const CHAIN_BREAK: i32 = 0x7166;
const GLOBAL_ARR: u32 = 0x01179890;

/// Arguments the entry block pushes for callee 1 (both sites share the
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
            wr_u32(a20, NOT_FOUND_BITS);
        }
        a20
    }
}

/// Resolve an entry pointer from a packed id (low word = slot, high
/// word = index into 32-byte entries).
#[inline(always)]
unsafe fn resolve(handle: u32, id: u32) -> u32 {
    unsafe {
        let base = rd_u32(handle.wrapping_add((id & 0xFFFF).wrapping_mul(4)).wrapping_add(SLOT_BASE));
        base.wrapping_add((id >> 16).wrapping_mul(32))
    }
}

/// Tail sweep shared by both exits: stamp the fresh word over every
/// collected id's cost slot, or run the reset call past the cap.
unsafe fn sweep(handle: u32, counter: u32) {
    unsafe {
        if counter >= SWEEP_CAP {
            callee_thiscall!(4, u32, handle);
        } else if counter > 0 {
            let arr = relocated(GLOBAL_ARR);
            let mut i = 0u32;
            while i < counter {
                let w = rd_u32(arr.wrapping_add(i.wrapping_mul(4)));
                let base =
                    rd_u32(handle.wrapping_add((w & 0xFFFF).wrapping_mul(4)).wrapping_add(SLOT_BASE));
                wr_u16(
                    base.wrapping_add((w >> 16).wrapping_mul(32)).wrapping_add(0x10),
                    FRESH_WORD,
                );
                i = i.wrapping_add(1);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn run(
    handle: u32,
    a08: u32,
    a0c: u32,
    a10: u32,
    a14: u32,
    a18: u32,
    a1c: u32,
    a20: u32,
    a24: u32,
    a28: u32,
    a2c: u32,
    a30: u32,
    a34: u32,
    a38: u32,
    a3c: u32,
    a40: u32,
    a44: u32,
    a48: u32,
    a4c: u32,
) -> u32 {
    unsafe {
        let gate = if rd_u8(handle.wrapping_add(GATE_OFF)) != 0 {
            0u32
        } else {
            (a44 as u8) as u32
        };
        wr_u32(handle.wrapping_add(LIVE_OFF), 0);
        let id1: u32;
        if a28 == 0 {
            id1 = search_call(handle, a10, a24, a3c);
        } else {
            let w = rd_u32(a28);
            let slot = w & INVALID;
            if slot == INVALID {
                id1 = search_call(handle, a10, a24, a3c);
            } else if rd_u32(handle.wrapping_add(slot.wrapping_mul(4)).wrapping_add(SLOT_BASE)) == 0
            {
                id1 = search_call(handle, a10, a24, a3c);
            } else {
                id1 = w;
            }
        }
        if (id1 & INVALID) == INVALID {
            wr_u32(a18, 0);
            return epilogue(a20);
        }
        let id2: u32;
        let slot2 = a0c & INVALID;
        if slot2 == INVALID {
            id2 = search_call(handle, a08, a24, a3c);
        } else if rd_u32(handle.wrapping_add(slot2.wrapping_mul(4)).wrapping_add(SLOT_BASE)) == 0 {
            id2 = search_call(handle, a08, a24, a3c);
        } else {
            id2 = a0c;
        }
        if (id2 & INVALID) == INVALID {
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
        let entry_c = resolve(handle, id2);
        let entry_d = resolve(handle, id1);
        if (rd_u8(entry_c.wrapping_add(0x1B)) ^ rd_u8(entry_d.wrapping_add(0x1B))) & 0x1F != 0 {
            wr_u32(a18, 0);
            return epilogue(a20);
        }
        // Main scan: clear the slot table, seed through callee 2, then
        // walk slots trip by trip.
        for k in 0..SCAN_COUNT {
            wr_u32(handle.wrapping_add(4).wrapping_add(k.wrapping_mul(4)), 0);
        }
        callee_thiscall!(2, u32, handle, entry_d, 0);
        let gw = rd_u32(entry_d.wrapping_add(8));
        wr_u32(relocated(GLOBAL_ARR), gw);
        let mut counter = 1u32;
        let mut trip = 0u32;
        let mut latch = 0u32;
        loop {
            let slot_idx = trip & 0x1FF;
            let mut link = rd_u32(handle.wrapping_add(slot_idx.wrapping_mul(4)).wrapping_add(4));
            if link != 0 {
                loop {
                    if link == entry_c {
                        latch = 1;
                    }
                    let rows = rd_u8(link.wrapping_add(0x1E)) & 0xF;
                    if rows != 0 {
                        let w16 = rd_u16(link.wrapping_add(8)) as u32;
                        let s16 = rd_i16(link.wrapping_add(0x12));
                        let rowbase =
                            rd_u32(handle.wrapping_add(w16.wrapping_mul(4)).wrapping_add(ROW_BASE));
                        let mut r = 0u32;
                        while r < rows as u32 {
                            let rowabs = (s16.wrapping_add(r as i32)) as u32;
                            let row_at = rowbase.wrapping_add(rowabs.wrapping_mul(8));
                            let id = rd_u32(row_at);
                            let base = rd_u32(
                                handle
                                    .wrapping_add((id & INVALID).wrapping_mul(4))
                                    .wrapping_add(SLOT_BASE),
                            );
                            if base != 0 {
                                let entry = base.wrapping_add((id >> 16).wrapping_mul(32));
                                let flagv =
                                    (rd_u8(row_at.wrapping_add(5)) >> 3) as u32 & 7;
                                let mut lat: u32 = 0;
                                if (a30 as u8) != 0 {
                                    let bit_set =
                                        rd_u8(row_at.wrapping_add(7)) & 0x10 != 0;
                                    if (a4c as u8) == 0 || !bit_set {
                                        if flagv == 0 {
                                            lat = 1;
                                        }
                                    }
                                }
                                if a34 == id {
                                    lat = 1;
                                }
                                let dh = rd_u8(link.wrapping_add(0x1F));
                                let dl = rd_u8(entry.wrapping_add(0x1F));
                                if (dh ^ dl) & 0x2 != 0 {
                                    if (a38 as u8) == 0 {
                                        lat = 1;
                                    }
                                }
                                if gate != 0 && dl & 0x80 != 0 {
                                    lat = 1;
                                }
                                if (a48 as u8) != 0 && dl & 0x20 != 0 && dh & 0x20 == 0 {
                                    lat = 1;
                                }
                                let ek = rd_u8(entry.wrapping_add(0x1C)) & 0xF0 == 0xB0;
                                let lk = rd_u8(link.wrapping_add(0x1C)) & 0xF0 == 0xB0;
                                if ek == lk && lat == 0 {
                                    let rb = rd_u8(row_at.wrapping_add(4)) as u32;
                                    let mut cost =
                                        rd_i16(link.wrapping_add(0x10)).wrapping_add(rb as i32);
                                    if flagv == 0 {
                                        cost = cost.wrapping_add((rb >> 1) as i32);
                                    }
                                    let lim = rd_u16(entry.wrapping_add(0x10)) as i32;
                                    if cost < lim {
                                        if lim as u16 != FRESH_WORD {
                                            callee_thiscall!(3, u32, handle, entry);
                                        }
                                        if rd_u16(entry.wrapping_add(0x10)) == FRESH_WORD
                                        {
                                            if counter < SWEEP_CAP {
                                                let ew = rd_u32(entry.wrapping_add(8));
                                                wr_u32(
                                                    relocated(GLOBAL_ARR)
                                                        .wrapping_add(counter.wrapping_mul(4)),
                                                    ew,
                                                );
                                                counter = counter.wrapping_add(1);
                                            }
                                        }
                                        callee_thiscall!(
                                            2,
                                            u32,
                                            handle,
                                            entry,
                                            cost as u32
                                        );
                                    }
                                }
                            }
                            r = r.wrapping_add(1);
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
            if rd_u32(handle.wrapping_add(LIVE_OFF)) == 0 {
                wr_u32(a18, 0);
                sweep(handle, counter);
                return epilogue(a20);
            }
            let bound = f32::from_bits(a2c);
            let ftrip = trip as f32;
            if ftrip > bound {
                wr_u32(a18, 0);
                sweep(handle, counter);
                return epilogue(a20);
            }
            if latch != 0 {
                break;
            }
        }
        // Latch-driven collection over the winning entry's chain.
        collect(
            handle, a14, a18, a1c, a20, a40, entry_c, entry_d, counter,
        )
    }
}

/// Collection walk: append the winning entry, then chase its row chain
/// while the count stays below the bound. Returns the leftover global
/// word (or the bound when nothing was swept); unlike the counter exit
/// there is no epilogue publish here.
unsafe fn collect(
    handle: u32,
    a14: u32,
    a18: u32,
    a1c: u32,
    a20: u32,
    a40: u32,
    entry_c: u32,
    entry_d: u32,
    counter: u32,
) -> u32 {
    unsafe {
        use core::hint::black_box;
        wr_u32(a18, 0);
        if a20 != 0 {
            let w = rd_i16(entry_c.wrapping_add(0x10)) as f32;
            wr_u32(a20, black_box(w).to_bits());
        }
        if a14 != 0 {
            wr_u32(a14, rd_u32(entry_c.wrapping_add(8)));
            wr_u32(a18, 1);
        }
        let mut cur = entry_c;
        if (rd_u32(a18) as i32) < a1c as i32 {
            let xy = image_f32(0x00FE87A4);
            let zed = image_f32(0x00FE8720);
            let one = image_f32(0x00FE88E8);
            let thresh = image_f32(0x00E833CC);
            loop {
                if cur == entry_d {
                    break;
                }
                let rows = rd_u8(cur.wrapping_add(0x1E)) & 0xF;
                if rows != 0 {
                    let w16 = rd_u16(cur.wrapping_add(8)) as u32;
                    let s16 = rd_i16(cur.wrapping_add(0x12));
                    let rowbase = rd_u32(
                        handle.wrapping_add(w16.wrapping_mul(4)).wrapping_add(ROW_BASE),
                    );
                    let mut j = 0u32;
                    while j < rows as u32 {
                        let rowabs = (s16.wrapping_add(j as i32)) as u32;
                        let row_at = rowbase.wrapping_add(rowabs.wrapping_mul(8));
                        let id = rd_u32(row_at);
                        let base = rd_u32(
                            handle
                                .wrapping_add((id & INVALID).wrapping_mul(4))
                                .wrapping_add(SLOT_BASE),
                        );
                        if base != 0 {
                            let entry =
                                base.wrapping_add((id >> 16).wrapping_mul(32));
                            let mut c8 = rd_u8(row_at.wrapping_add(4)) as u32;
                            if rd_u8(row_at.wrapping_add(5)) & 7 == 0 {
                                c8 = c8.wrapping_add(c8 >> 1);
                            }
                            let diff = rd_i16(cur.wrapping_add(0x10))
                                .wrapping_sub(c8 as i32);
                            if diff == rd_i16(entry.wrapping_add(0x10)) {
                                cur = entry;
                                if (a40 as u8) != 0 && (rd_u32(a18) as i32) >= 2 {
                                    let n = rd_u32(a18) as i32;
                                    let ida = rd_u32(
                                        a14.wrapping_add((n as u32).wrapping_mul(4)).wrapping_sub(8),
                                    );
                                    let idb = rd_u32(
                                        a14.wrapping_add((n as u32).wrapping_mul(4)).wrapping_sub(4),
                                    );
                                    let enta = resolve(handle, ida);
                                    let entb = resolve(handle, idb);
                                    let ax = black_box(
                                        black_box(rd_i16(enta.wrapping_add(0x14)) as f32)
                                            * black_box(xy),
                                    );
                                    let ay = black_box(
                                        black_box(rd_i16(enta.wrapping_add(0x16)) as f32)
                                            * black_box(xy),
                                    );
                                    let az = black_box(
                                        black_box(rd_i16(enta.wrapping_add(0x18)) as f32)
                                            * black_box(zed),
                                    );
                                    let bx = black_box(
                                        black_box(rd_i16(entb.wrapping_add(0x14)) as f32)
                                            * black_box(xy),
                                    );
                                    let by = black_box(
                                        black_box(rd_i16(entb.wrapping_add(0x16)) as f32)
                                            * black_box(xy),
                                    );
                                    let bz = black_box(
                                        black_box(rd_i16(entb.wrapping_add(0x18)) as f32)
                                            * black_box(zed),
                                    );
                                    let cx = black_box(
                                        black_box(rd_i16(entry.wrapping_add(0x14)) as f32)
                                            * black_box(xy),
                                    );
                                    let cy = black_box(
                                        black_box(rd_i16(entry.wrapping_add(0x16)) as f32)
                                            * black_box(xy),
                                    );
                                    let cz = black_box(
                                        black_box(rd_i16(entry.wrapping_add(0x18)) as f32)
                                            * black_box(image_f32(0x00FE8720)),
                                    );
                                    let d1x = black_box(black_box(bx) - black_box(ax));
                                    let d1y = black_box(black_box(by) - black_box(ay));
                                    let d1z = black_box(black_box(bz) - black_box(az));
                                    let d2x = black_box(black_box(cx) - black_box(bx));
                                    let d2y = black_box(black_box(cy) - black_box(by));
                                    let d2z = black_box(black_box(cz) - black_box(bz));
                                    let l1 = black_box(
                                        black_box(
                                            black_box(black_box(d1y) * black_box(d1y))
                                                + black_box(black_box(d1x) * black_box(d1x)),
                                        ) + black_box(black_box(d1z) * black_box(d1z)),
                                    );
                                    let inv1 = if black_box(l1) == 0.0 {
                                        black_box(0.0f32);
                                        0.0f32
                                    } else {
                                        black_box(
                                            black_box(one)
                                                / black_box(
                                                    black_box(l1).sqrt(),
                                                ),
                                        )
                                    };
                                    let n1x = black_box(black_box(d1x) * black_box(inv1));
                                    let n1y = black_box(black_box(d1y) * black_box(inv1));
                                    let n1z = black_box(black_box(d1z) * black_box(inv1));
                                    let l2 = black_box(
                                        black_box(
                                            black_box(black_box(d2y) * black_box(d2y))
                                                + black_box(black_box(d2x) * black_box(d2x)),
                                        ) + black_box(black_box(d2z) * black_box(d2z)),
                                    );
                                    let inv2 = if black_box(l2) == 0.0 {
                                        black_box(0.0f32);
                                        0.0f32
                                    } else {
                                        black_box(
                                            black_box(one)
                                                / black_box(
                                                    black_box(l2).sqrt(),
                                                ),
                                        )
                                    };
                                    let m2y = black_box(black_box(d2y) * black_box(inv2));
                                    let m2x = black_box(black_box(d2x) * black_box(inv2));
                                    let t1 = black_box(black_box(m2y) * black_box(n1y));
                                    let t2 = black_box(black_box(m2x) * black_box(n1x));
                                    let m2z = black_box(black_box(d2z) * black_box(inv2));
                                    let t3 = black_box(black_box(m2z) * black_box(n1z));
                                    let dot = black_box(
                                        black_box(black_box(t1) + black_box(t2))
                                            + black_box(t3),
                                    );
                                    if black_box(dot) > black_box(thresh) {
                                        wr_u32(a18, rd_u32(a18).wrapping_sub(1));
                                    }
                                }
                                let n = rd_u32(a18);
                                wr_u32(
                                    a14.wrapping_add(n.wrapping_mul(4)),
                                    rd_u32(entry.wrapping_add(8)),
                                );
                                wr_u32(a18, n.wrapping_add(1));
                                j = CHAIN_BREAK as u32;
                            }
                        }
                        j = j.wrapping_add(1);
                        let cur_rows = rd_u8(cur.wrapping_add(0x1E)) & 0xF;
                        if j >= cur_rows as u32 {
                            break;
                        }
                    }
                }
                if (rd_u32(a18) as i32) >= a1c as i32 {
                    break;
                }
            }
        }
        if counter >= SWEEP_CAP {
            let ans: u32 = callee_thiscall!(4, u32, handle);
            return ans;
        }
        sweep(handle, counter);
        if counter > 0 {
            rd_u32(relocated(GLOBAL_ARR).wrapping_add(counter.wrapping_sub(1).wrapping_mul(4)))
        } else {
            a1c
        }
    }
}

/// Resolve two object ids, scan the slot table for the winner, and walk
/// the winning chain.
///
/// `handle` owns id tables (`+0x804` bases, `+0x904` row tables). Each id
/// comes from its argument word unless that word is missing, invalid
/// (`0xFFFF`) or unbacked, in which case callee 1 searches for it. Ids
/// that are still invalid, or equal, or whose entries disagree on the low
/// flag bits exit early with `*a18 = 0`. Otherwise the scan clears 512
/// slots, seeds through callee 2, and walks every slot trip by trip:
/// links latch when they equal the winner, rows filter on flag bytes and
/// id match, and cheaper rows release through callee 3, append to the
/// global array and re-seed through callee 2. Trips continue while the
/// live count is nonzero and the trip index stays within the float bound.
/// A latched trip ends the scan with the collection walk, which appends
/// the winner and chases its row chain (with an angle veto past two
/// entries) below the count bound. Returns `a20` on every early and
/// counter exit (publishing the not-found float when it is a real
/// pointer), and the leftover global word on the collect tail.
export!(
    thiscall,
    rw_008e7b90(
        handle: u32,
        a08: u32,
        a0c: u32,
        a10: u32,
        a14: u32,
        a18: u32,
        a1c: u32,
        a20: u32,
        a24: u32,
        a28: u32,
        a2c: u32,
        a30: u32,
        a34: u32,
        a38: u32,
        a3c: u32,
        a40: u32,
        a44: u32,
        a48: u32,
        a4c: u32,
    ) -> u32 {
        unsafe {
            run(
                handle, a08, a0c, a10, a14, a18, a1c, a20, a24, a28, a2c, a30, a34, a38, a3c,
                a40, a44, a48, a4c,
            )
        }
    }
);
