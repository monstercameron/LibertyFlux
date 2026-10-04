// original: 0x009623F0 walk_tagged_nodes
/// Walk a chain of tagged nodes starting from a three-word cursor.
///
/// The cursor holds the current pointer, a base added to it, and a tag byte
/// (the low byte of the third word). Each step validates the node through
/// helper 1 and its sequence number against the limit argument, then
/// dispatches on the node's tag: tag 0x12 runs the resolve chain (helper 4
/// looks the link up, helper 5 confirms it with 1.0) advancing 16 bytes per
/// node, tag 0x9C runs helper 3 advancing 32 bytes per node, any other tag
/// advances by helper 2's answer for the tag byte. A zero tag consults the
/// slot table: slot value 2 ends the walk, anything else reloads the base
/// from the next table (the quotient of the tag-plus-one by the divisor is
/// discarded; the divisor is nonzero in every trial). Always returns 1.
///
/// Note: the original homes its tag local over its own incoming argument
/// slot, clobbering the slot's low byte on entry. That write is
/// irreproducible from Rust, so the contract disables the stack check; the
/// slot is dead to every caller (cdecl cleanup) and nothing else the
/// function does touches the stack above its frame.
export!(cdecl, rw_009623F0(arg0: u32, limit: u32) -> u8 {
    unsafe {
        const SLOT_TABLE: u32 = 0x11F7018;
        const DIVISOR: u32 = 0x11F6FFD;
        const NEXT_TABLE: u32 = 0x11F7000;
        const TAG_RESOLVE: u8 = 0x12;
        const TAG_VISIT: u8 = 0x9C;
        const SLOT_DONE: u8 = 2;
        const CONFIRM: u32 = 0x3F800000;

        let mut edi = *(arg0 as *const u32);
        let mut ebp = *((arg0.wrapping_add(4)) as *const u32);
        let dl = *((arg0.wrapping_add(8)) as *const u8);

        'main: loop {
            let ebx = edi.wrapping_add(ebp);
            if *(ebx as *const u8) != 0 {
                let ok: u32 = callee_thiscall!(1, u32, ebx);
                if ok == 0 {
                    return 1;
                }
                if *((ebx.wrapping_add(4)) as *const u32) >= limit {
                    return 1;
                }
            }
            let cl = *(ebx as *const u8);
            if cl == 0 {
                let slot = *((relocated(SLOT_TABLE).wrapping_add(dl as u32)) as *const u8);
                if slot == SLOT_DONE {
                    return 1;
                }
                let div = *(relocated(DIVISOR) as *const u8);
                let _quotient = ((dl as u32).wrapping_add(1)) / (div as u32);
                edi = 0;
                ebp = *((relocated(NEXT_TABLE)
                    .wrapping_add((dl as u32).wrapping_mul(4)))
                    as *const u32);
                continue 'main;
            }
            if cl == TAG_RESOLVE {
                let mut esi = ebx;
                'resolve: loop {
                    if *(esi as *const u8) != TAG_RESOLVE {
                        break 'resolve;
                    }
                    if *((esi.wrapping_add(4)) as *const u32) >= limit {
                        break 'resolve;
                    }
                    let link = *((edi.wrapping_add(ebp).wrapping_add(8)) as *const u32);
                    if link != 0xFFFF_FFFF {
                        let found: u32 = callee_cdecl!(4, u32, 1, link);
                        if found != 0 {
                            let outer = *((found.wrapping_add(0x34)) as *const u32);
                            if outer != 0 {
                                let inner = *((outer.wrapping_add(4)) as *const u32);
                                if inner != 0
                                    && *((inner.wrapping_add(0xCD)) as *const u8) != 0
                                {
                                    callee_thiscall!(5, u32, found.wrapping_add(0x10E0), CONFIRM);
                                }
                            }
                        }
                    }
                    edi = edi.wrapping_add(0x10);
                    esi = edi.wrapping_add(ebp);
                    if *(esi as *const u8) != TAG_RESOLVE {
                        break 'resolve;
                    }
                }
                continue 'main;
            }
            if cl == TAG_VISIT {
                let mut esi = ebx;
                if *(esi as *const u8) == TAG_VISIT {
                    'visit: loop {
                        if *((esi.wrapping_add(4)) as *const u32) >= limit {
                            break 'visit;
                        }
                        callee_thiscall!(3, u32, edi.wrapping_add(ebp));
                        edi = edi.wrapping_add(0x20);
                        esi = edi.wrapping_add(ebp);
                        if *(esi as *const u8) != TAG_VISIT {
                            break 'visit;
                        }
                    }
                }
                continue 'main;
            }
            let tag = *(ebx as *const u8);
            let step: u32 = callee_cdecl!(2, u32, tag as u32);
            edi = edi.wrapping_add(step);
        }
    }
});
