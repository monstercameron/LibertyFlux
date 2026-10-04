// original: 0x0091bad0 key_table_lookup
use lf_checker_rt::{export, global, relocated};

/// Look up a key in per-index tables with case folding.
///
/// Searches two index ranges from the record selected by `idx`, then retries
/// with the key upper-cased (low byte minus 0x20) when the mode byte allows,
/// then scans the record's leading table. Returns the matching slot index,
/// or 0xFFFF/0xFD sentinels when nothing matches (chosen by mode bytes).
export!(cdecl, rw_0091bad0(a0: u32, a1: u32, a2: u32) -> u32 {
    const MODE: u32 = 0x116C250;
    const FLAG: u32 = 0x116C253;
    const TABLE: u32 = 0x1195577;
    const B0A: u32 = 0x1195698;
    const B0B: u32 = 0x119569C;
    const B1A: u32 = 0x11956A0;
    const B1B: u32 = 0x11956A4;
    const B2A: u32 = 0x11956A8;
    const B2B: u32 = 0x11956AC;
    const B3A: u32 = 0x11956B0;
    const B3B: u32 = 0x11956B4;
    unsafe {
        let mut edx = a0.wrapping_add(0x20);
        let dx0 = (edx & 0xFFFF) as u16;
        if dx0 == 0x20 {
            if global::<u8>(MODE).read() == 0x6A {
                return 0xFFFF;
            }
            if global::<u8>(FLAG).read() != 0 {
                return 0xFFFF;
            }
            return 0xFD;
        }
        let esi = a1.wrapping_mul(0x258);
        let (mut ecx, ebx) = if a2.wrapping_sub(1) == 0 {
            let c = (relocated(B1A.wrapping_add(esi)) as *const u32).read_unaligned();
            let b = (relocated(B1B.wrapping_add(esi)) as *const u32).read_unaligned();
            (c, b)
        } else if a2.wrapping_sub(2) == 0 {
            let c = (relocated(B2A.wrapping_add(esi)) as *const u32).read_unaligned();
            let b = (relocated(B2B.wrapping_add(esi)) as *const u32).read_unaligned();
            (c, b)
        } else {
            let c = (relocated(B0A.wrapping_add(esi)) as *const u32).read_unaligned();
            let b = (relocated(B0B.wrapping_add(esi)) as *const u32).read_unaligned();
            (c, b)
        };
        if (ecx as i32) < (ebx as i32) {
            let mut edi = ecx;
            loop {
                let va = TABLE.wrapping_add(esi).wrapping_add(edi);
                let b = (relocated(va) as *const u8).read();
                if (b as u16) == ((edx & 0xFFFF) as u16) {
                    return edi & 0xFFFF;
                }
                edi = edi.wrapping_add(1);
                if !((edi as i32) < (ebx as i32)) {
                    break;
                }
            }
        }
        let mut edi = (relocated(B3A.wrapping_add(esi)) as *const u32).read_unaligned();
        let ebp = (relocated(B3B.wrapping_add(esi)) as *const u32).read_unaligned();
        if (edi as i32) < (ebp as i32) {
            loop {
                let va = TABLE.wrapping_add(esi).wrapping_add(edi);
                let b = (relocated(va) as *const u8).read();
                if (b as u16) == ((edx & 0xFFFF) as u16) {
                    return edi & 0xFFFF;
                }
                edi = edi.wrapping_add(1);
                if !((edi as i32) < (ebp as i32)) {
                    break;
                }
            }
        }
        let mode = global::<u8>(MODE).read();
        if mode == 0x72 {
            let dx = (edx & 0xFFFF) as u16;
            if !(0x61u16..=0x7Au16).contains(&dx) {
                let ax = ((edx.wrapping_sub(0xAE)) & 0xFFFF) as u16;
                if ax > 0x1F {
                    return fallback_0091bad0(esi, edx);
                }
            }
            edx = edx.wrapping_add(0xFFE0);
            if (ecx as i32) < (ebx as i32) {
                loop {
                    let va = TABLE.wrapping_add(esi).wrapping_add(ecx);
                    let b = (relocated(va) as *const u8).read();
                    if (b as u16) == ((edx & 0xFFFF) as u16) {
                        return ecx & 0xFFFF;
                    }
                    ecx = ecx.wrapping_add(1);
                    if !((ecx as i32) < (ebx as i32)) {
                        break;
                    }
                }
            }
            return fallback_0091bad0(esi, edx);
        } else {
            let dx = (edx & 0xFFFF) as u16;
            if !(0x61u16..=0x7Au16).contains(&dx) && dx < 0xE0 {
                return fallback_0091bad0(esi, edx);
            }
            edx = edx.wrapping_add(0xFFE0);
            if (ecx as i32) < (ebx as i32) {
                loop {
                    let va = TABLE.wrapping_add(esi).wrapping_add(ecx);
                    let b = (relocated(va) as *const u8).read();
                    if (b as u16) == ((edx & 0xFFFF) as u16) {
                        return ecx & 0xFFFF;
                    }
                    ecx = ecx.wrapping_add(1);
                    if !((ecx as i32) < (ebx as i32)) {
                        break;
                    }
                }
            }
            return fallback_0091bad0(esi, edx);
        }
    }
});

/// Scan the record's leading 254 bytes for the key, else sentinel.
fn fallback_0091bad0(esi: u32, edx: u32) -> u32 {
    const MODE: u32 = 0x116C250;
    const FLAG: u32 = 0x116C253;
    const TABLE: u32 = 0x1195577;
    unsafe {
        let base = TABLE.wrapping_add(esi);
        let mut ecx: u32 = 0;
        while ecx < 0xFE {
            let b = (relocated(base.wrapping_add(ecx)) as *const u8).read();
            if (b as u16) == ((edx & 0xFFFF) as u16) {
                return ecx & 0xFFFF;
            }
            ecx = ecx.wrapping_add(1);
        }
        if global::<u8>(MODE).read() == 0x6A {
            return 0xFFFF;
        }
        if global::<u8>(FLAG).read() != 0 {
            return 0xFFFF;
        }
        0xFD
    }
}
