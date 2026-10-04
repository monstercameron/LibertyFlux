// original: 0x008eea20 tagged_record_scanner
/// Scan the record list reachable from `rec` and publish hits into out-slots.
///
/// `this` is the owning table object, `rec` the record to scan, `out_first`
/// and `out_rest` two caller slots (the second may be null when no second
/// hit can occur), `mode` a flag byte and `reseed` a reseed selector.
///
/// First the record's tag byte (+0x1E) is refreshed: when `reseed` is
/// nonzero it is folded with the tag byte's neighbour (+0x1F), otherwise
/// its low seven bits are kept and the top bit comes from `mode`. Both
/// out-slots are then zeroed and the first callee classifies `rec`; a
/// signed result of 3 or more ends the scan immediately.
///
/// Otherwise the tag's low nibble bounds a loop over the record's entries.
/// Each entry resolves through the +0x904 row table and the +0x804 node
/// table to a node record; empty nodes are skipped. The second callee
/// validates the node, a tag check selected by `reseed` (two tag bits, or
/// the tag's top bit against `mode`) filters it, and the first callee
/// re-classifies it. Accepted nodes land in `out_first` while
/// it is still zero and in `out_rest` afterwards.
///
/// Returns the first callee's verdict when the scan never runs, otherwise
/// the tag's low nibble.
export!(thiscall, rw_008eea20(this: u32, rec: u32, out_first: u32, out_rest: u32, mode: u32, reseed: u32) -> u32 {
    unsafe {
        let tag = rec as *mut u8;
        if (reseed as u8) != 0 {
            let neighbour = *tag.add(0x1f);
            let folded = neighbour.wrapping_shl(5);
            let old = *tag.add(0x1e);
            *tag.add(0x1e) = ((folded ^ old) & 0x7f) ^ folded;
        } else {
            let old = *tag.add(0x1e);
            let top = (mode as u8).wrapping_shl(7);
            *tag.add(0x1e) = (old & 0x7f) | top;
        }
        core::ptr::write(out_first as *mut u32, 0);
        if out_rest != 0 {
            core::ptr::write(out_rest as *mut u32, 0);
        }
        let verdict: u32 = callee_thiscall!(0, u32, this, rec);
        if (verdict as i32) >= 3 {
            return verdict;
        }
        if (*tag.add(0x1e) & 0x0f) == 0 {
            return verdict;
        }
        let row_index = ((rec as *const u16).add(4).read()) as u32;
        let entry_base = ((rec as *const i16).add(9).read()) as i32;
        let mut entry = 0u32;
        loop {
            let rows = ((this
                .wrapping_add(row_index.wrapping_mul(4))
                .wrapping_add(0x904)) as *const u32)
                .read();
            let row = ((rows.wrapping_add(
                (entry_base.wrapping_add(entry as i32) as u32).wrapping_mul(8),
            )) as *const u32)
                .read();
            let node_base = ((this
                .wrapping_add((row as u16 as u32).wrapping_mul(4))
                .wrapping_add(0x804)) as *const u32)
                .read();
            if node_base != 0 {
                let node = node_base.wrapping_add((row >> 16).wrapping_shl(5));
                let accepted: u32 = callee_thiscall!(1, u32, this, node);
                if (accepted as u8) != 0 {
                    let keep = if (reseed as u8) != 0 {
                        let left = (*((node as *const u8).add(0x1e)) >> 5) & 4;
                        let right = *((node as *const u8).add(0x1f)) & 4;
                        left != right
                    } else {
                        let flag = *((node as *const u8).add(0x1e)) >> 7;
                        flag != (mode as u8)
                    };
                    if keep {
                        let second: u32 = callee_thiscall!(0, u32, this, node);
                        if (second as i32) < 3 {
                            if (out_first as *const u32).read() == 0 {
                                core::ptr::write(out_first as *mut u32, node);
                            } else {
                                core::ptr::write(out_rest as *mut u32, node);
                            }
                        }
                    }
                }
            }
            entry = entry.wrapping_add(1);
            if entry >= (u32::from(*tag.add(0x1e) & 0x0f)) {
                break;
            }
        }
        u32::from(*tag.add(0x1e) & 0x0f)
    }
});
