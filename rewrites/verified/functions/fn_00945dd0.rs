// original: 0x00945dd0 radio_rebuild_station_tables
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Rebuild the radio station tables for one station group (original 0x00945DD0).
///
/// Walks the group's entries: each entry pointer is refreshed through the
/// station manager, its item table is relocated and each item pointer is
/// refreshed (copying the weight byte into the item's weight word when the
/// item is active). The entry name is copied to a frame buffer and resolved
/// to a station object; when resolution succeeds every active item is linked
/// into the station (merging weights, appending to the item chain, or queuing
/// the entry on the station's pending list). When resolution fails a fresh
/// station object is allocated and published into the global station table.
/// Returns the cookie-check answer, like the original (callers ignore it).
export!(cdecl, rw_00945dd0(group: u32) -> u32 {
    const MANAGER: u32 = 0x0115D9A0;
    const COUNT_OFF: u32 = 0x0A;
    const SLOTS_OFF: u32 = 0x0B;
    unsafe {
        let manager = relocated(MANAGER);
        let mut outer: u8 = 0;
        while outer < ((group + COUNT_OFF) as *const u8).read() {
            let slot = group.wrapping_add(outer as u32 * 4);
            let entry = (slot + SLOTS_OFF) as *mut u32;
            let raw = entry.read();
            if raw != 0 {
                let station = callee_thiscall!(1, u32, manager, raw);
                entry.write(station);
                if station != 0 {
                    rebuild_entry(station, manager);
                }
            }
            outer = outer.wrapping_add(1);
        }
        // Epilogue cookie check; its answer is the (ignored) return value.
        callee_cdecl!(7, u32,)
    }
});

/// Refresh one entry's item table and link it into its station object.
#[inline(never)]
unsafe fn rebuild_entry(station: u32, manager: u32) {
    const ALLOC_SIZE: u32 = 0x1934;
    const NAME_LIMIT: u32 = 0xFF;
    const NAME_LEN_OFF: u32 = 0x1D;
    const NAME_OFF: u32 = 0x1E;
    const ITEM_COUNT_OFF: u32 = 0x13;
    const ITEMS_OFF: u32 = 0x14;
    const ITEM_BASE_OFF: u32 = 0x11D;
    const ITEMS_BASE_OFF: u32 = 0x11E;
    const ACTIVE_OFF: u32 = 0x0A;
    const WEIGHT_WORD_OFF: u32 = 0x39;
    const WEIGHT_BYTE_OFF: u32 = 0x3F;
    const CHAIN_OFF: u32 = 0x3B;
    const PENDING_HEAD_OFF: u32 = 0x17D0;
    const PENDING_LINK_OFF: u32 = 0x18;
    const STATION_COUNT: u32 = 0x011D74F1;
    const STATION_TABLE: u32 = 0x011D76AC;
    unsafe {
        let name_len = ((station + NAME_LEN_OFF) as *const u8).read() as u32;
        let base = station.wrapping_sub(0xFFu32.wrapping_sub(name_len));
        let item_count = ((base + ITEM_BASE_OFF) as *const u8).read();
        let items = base.wrapping_add(ITEMS_BASE_OFF);
        ((station + ITEM_COUNT_OFF) as *mut u8).write(item_count);
        ((station + ITEMS_OFF) as *mut u32).write(items);
        // Refresh every item pointer through the manager.
        let mut inner: u8 = 0;
        while inner < ((station + ITEM_COUNT_OFF) as *const u8).read() {
            let slot = (items + inner as u32 * 4) as *mut u32;
            let raw = slot.read();
            if raw != 0 {
                let item = callee_thiscall!(1, u32, manager, raw);
                slot.write(item);
                if ((item + ACTIVE_OFF) as *const u8).read() != 0 {
                    let weight = ((item + WEIGHT_BYTE_OFF) as *const u8).read();
                    ((item + WEIGHT_WORD_OFF) as *mut u16).write(weight as u16);
                }
            }
            inner = inner.wrapping_add(1);
        }
        // Resolve the entry name to a station object.
        let mut name = [0u8; 256];
        callee_cdecl!(
            2,
            u32,
            name.as_mut_ptr() as u32,
            station.wrapping_add(NAME_OFF),
            name_len
        );
        let name_len = ((station + NAME_LEN_OFF) as *const u8).read() as usize;
        if name_len >= NAME_LIMIT as usize {
            // Unreachable in the contract (names are short); the original
            // calls its fatal-error routine here.
            core::hint::unreachable_unchecked()
        }
        name[name_len] = 0;
        let resolved = callee_cdecl!(3, u32, name.as_mut_ptr() as u32);
        if resolved == 0 {
            // Resolution failed: allocate and publish a fresh object.
            let fresh = callee_cdecl!(5, u32, ALLOC_SIZE);
            let published = if fresh == 0 {
                0
            } else {
                callee_thiscall!(6, u32, fresh, station)
            };
            let count = global::<u8>(STATION_COUNT);
            let table = global::<u32>(STATION_TABLE).read() as *mut u32;
            table.add(count.read() as usize).write(published);
            count.write(count.read().wrapping_add(1));
            return;
        }
        // Link every active item into the resolved station.
        let mut link: u8 = 0;
        while link < ((station + ITEM_COUNT_OFF) as *const u8).read() {
            let item = ((items + link as u32 * 4) as *const u32).read();
            let active = ((item + ACTIVE_OFF) as *const u8).read();
            if active != 0 {
                let target = callee_thiscall!(4, u32, resolved, active as u32);
                if target == 0 {
                    // Queue the entry on the station's pending list.
                    let mut node = (resolved + PENDING_HEAD_OFF) as *const u32;
                    node = node.read() as *const u32;
                    while ((node as u32 + PENDING_LINK_OFF) as *const u32).read() != 0 {
                        node = ((node as u32 + PENDING_LINK_OFF) as *const u32).read()
                            as *const u32;
                    }
                    if node as u32 != station {
                        ((node as u32 + PENDING_LINK_OFF) as *mut u32).write(station);
                    }
                } else if target != item {
                    let extra =
                        ((item + WEIGHT_BYTE_OFF) as *const u8).read() as u16;
                    let slot = (target + WEIGHT_WORD_OFF) as *mut u16;
                    slot.write(slot.read().wrapping_add(extra));
                    // Append the item at the tail of the target's chain.
                    let mut node = target;
                    while ((node + CHAIN_OFF) as *const u32).read() != 0 {
                        node = ((node + CHAIN_OFF) as *const u32).read();
                    }
                    ((node + CHAIN_OFF) as *mut u32).write(item);
                }
            }
            link = link.wrapping_add(1);
        }
    }
}
