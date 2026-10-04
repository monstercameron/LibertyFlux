// original: 0x00AFF3F0 pool_distant_spawn (proposed)
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

fn vcall_thiscall0(stub: u32, this: u32) -> u32 {
    let f: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(stub as usize) };
    f(this)
}

/// Scan the pool for a distant live row and run the spawn round on it.
///
/// Walks the global pool from the top down while the mode global selects
/// this pass. A row qualifies when its slot is live, its check call passes,
/// its state is fresh, its model flag is clear, its marker byte is set, both
/// reject calls stay silent, and the clock is past both of its timestamps.
/// Qualifying rows closer than the far radius are skipped; a row past it
/// runs the range call and, when that stays silent, the spawn call.
/// Returns the last value left in EAX along the taken path.
export!(cdecl, rw_aff3f0() -> u32 {
    unsafe {
        const MODE: u32 = 0x01173604;
        const POOL: u32 = 0x012E22A4;
        const CLOCK: u32 = 0x011735B4;
        const MODEL_TABLE: u32 = 0x01295CD8;
        const FAR_DIST: u32 = 0x00FE8B90;
        const FIXED_OBJ: u32 = 0x01908EF0;

        let gate: u32 = callee_cdecl!(1, u32,);
        let mut eax: u32 = gate;
        if (gate as u8) == 0 {
            return eax;
        }
        let mode: u32 = global::<u32>(MODE).read() & 0x1F;
        eax = mode;
        if (mode as u8) != 0x17 {
            return eax;
        }
        let pool: u32 = global::<u32>(POOL).read();
        let mut idx: i32 = ((pool + 8) as *const i32).read_unaligned();
        if idx == 0 {
            return eax;
        }
        loop {
            let used: u32 = ((pool + 4) as *const u32).read_unaligned();
            eax = used;
            idx = idx.wrapping_sub(1);
            let slot_live = (used.wrapping_add(idx as u32) as *const u8).read() & 0x80 == 0;
            'row: {
                if !slot_live {
                    break 'row;
                }
                let stride: i32 = ((pool + 12) as *const i32).read_unaligned();
                let base: i32 = (pool as *const i32).read_unaligned();
                let row = stride.wrapping_mul(idx).wrapping_add(base) as u32;
                if row == 0 {
                    break 'row;
                }
                let vt: u32 = (row as *const u32).read_unaligned();
                let stub: u32 = ((vt + 0x34) as *const u32).read_unaligned();
                let live: u32 = vcall_thiscall0(stub, row);
                eax = live;
                if (live as u8) == 0 {
                    break 'row;
                }
                let state: u32 = ((row + 0x1304) as *const u32).read_unaligned();
                eax = state;
                if state != 1 && state != 0 {
                    break 'row;
                }
                let mid = (((row + 0x2E) as *const u16).read_unaligned() as i16) as i32;
                eax = mid as u32;
                let mtbl = global::<u8>(MODEL_TABLE) as u32;
                let ment: u32 =
                    (mtbl.wrapping_add((mid as u32).wrapping_mul(4)) as *const u32).read_unaligned();
                eax = ment;
                let mflags: u32 = ((ment + 0x94) as *const u32).read_unaligned();
                eax = mflags;
                let shrunk = mflags >> 1;
                eax = shrunk;
                if (shrunk as u8) & 1 == 1 {
                    break 'row;
                }
                if ((row + 0x10B8) as *const u8).read() != 1 {
                    break 'row;
                }
                let r1: u32 = callee_thiscall!(3, u32, row);
                eax = r1;
                if (r1 as u8) != 0 {
                    break 'row;
                }
                let r2: u32 = callee_thiscall!(4, u32, row);
                eax = r2;
                if (r2 as u8) != 0 {
                    break 'row;
                }
                let now: u32 = global::<u32>(CLOCK).read();
                let t0: u32 = ((row + 0x10BC) as *const u32).read_unaligned();
                if now <= t0 {
                    break 'row;
                }
                let span: u32 = ((row + 0x0E38) as *const u32).read_unaligned();
                eax = span;
                let limit = span.wrapping_add(0x2710);
                eax = limit;
                if now <= limit {
                    break 'row;
                }
                let mat: u32 = ((row + 0x20) as *const u32).read_unaligned();
                eax = mat;
                let fx = ((mat + 0x30) as *const f32).read_unaligned();
                let fy = ((mat + 0x34) as *const f32).read_unaligned();
                let fz = ((mat + 0x38) as *const f32).read_unaligned();
                let frame = [fx.to_bits(), fy.to_bits(), fz.to_bits()];
                let near: u32 = callee_cdecl!(5, u32, frame.as_ptr() as u32, 0x41200000);
                eax = near;
                if (near as i32) < 4 {
                    break 'row;
                }
                let p1: u32 = callee_cdecl!(6, u32,);
                eax = p1;
                if p1 == 0 {
                    break 'row;
                }
                let p2: u32 = callee_cdecl!(6, u32,);
                eax = p2;
                let pmat: u32 = ((p2 + 0x20) as *const u32).read_unaligned();
                eax = pmat;
                let dx = ((pmat + 0x30) as *const f32).read_unaligned() - fx;
                let dy = ((pmat + 0x34) as *const f32).read_unaligned() - fy;
                let dz = ((pmat + 0x38) as *const f32).read_unaligned() - fz;
                let d2 = dx * dx + dy * dy + dz * dz;
                let dist = d2.sqrt();
                let far: f32 = global::<f32>(FAR_DIST).read();
                if !(dist > far) {
                    break 'row;
                }
                let mut out_slot = 0u32;
                let ranged: u32 = callee_thiscall!(
                    7,
                    u32,
                    relocated(FIXED_OBJ),
                    frame.as_ptr() as u32,
                    0x428C0000,
                    &mut out_slot as *mut u32 as u32
                );
                eax = ranged;
                if (ranged as u8) != 0 {
                    break 'row;
                }
                let spawned: u32 = callee_cdecl!(8, u32, row);
                eax = spawned;
            }
            if idx == 0 {
                break;
            }
        }
        eax
    }
});
