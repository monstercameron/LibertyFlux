// original: 0x00B00110 marker_list_rebuild
/// Visible-marker rebuild: when the focus point has moved far enough since
/// the last rebuild (or the cache is empty or stale), rescans the 64 grid
/// slots, keeps the markers within the given radius of the focus, and stores
/// the survivors as packed pairs plus a flag byte each.
///
/// The function is void; no return channel is compared. Every `jbe` after a
/// `comiss` is "not strictly greater" (true for NaN); every `jae` used as a
/// skip is "strictly greater" (false for NaN).
///
/// Original: 0x00B00110 (cdecl/2: focus point, radius).
const GRID_SINGLETON: u32 = 0x0117_7A80;
const GRID_SLOTS: u32 = 0x40;
const GRID_PRIMARY: u32 = 0x0117_8284;
const GRID_PAIRS: u32 = 0x0117_8384;
const MARK_COUNT: u32 = 0x0160_0180;
const MARK_DIRTY: u32 = 0x0103_FFF1;
const MARK_LAST: u32 = 0x0160_1040;
const MARK_PAIRS: u32 = 0x0160_0320;
const MARK_FLAGS: u32 = 0x0160_0188;
const MARK_CAP: u32 = 0x190;
const MOVE_LIMIT: u32 = 0x00FE_8B08;
const COORD_SCALE: u32 = 0x00FE_87A4;
const GRID_BIAS: u32 = 0x00E8_3174;

export!(cdecl, rw_00b00110(focus: u32, radius: f32) -> u32 {
    unsafe {
        let fx = (focus as *const f32).read();
        let fy = ((focus + 4) as *const f32).read();
        let count = global::<u32>(MARK_COUNT).read();
        if count != 0 {
            if global::<u8>(MARK_DIRTY).read() == 0 {
                let lx = (global::<f32>(MARK_LAST) as *const f32).read();
                let ly = (global::<f32>(MARK_LAST + 4) as *const f32).read();
                let dy = fy - ly;
                let dx = fx - lx;
                let moved = dy * dy + dx * dx;
                let limit = (global::<f32>(MOVE_LIMIT) as *const f32).read();
                if limit > moved.sqrt() {
                    return 0;
                }
            }
        }
        global::<u8>(MARK_DIRTY).write(0);
        global::<f32>(MARK_LAST).write(fx);
        global::<f32>(MARK_LAST + 4).write(fy);
        global::<f32>(MARK_LAST + 8).write(((focus + 8) as *const f32).read());
        global::<f32>(MARK_LAST + 12).write(((focus + 12) as *const f32).read());
        let r2 = radius * radius;
        let xr_plus = fx + radius;
        let xr_minus = fx - radius;
        let yr_plus = fy + radius;
        let yr_minus = fy - radius;
        global::<u32>(MARK_COUNT).write(0);
        let sing = relocated(GRID_SINGLETON);
        let scale = (global::<f32>(COORD_SCALE) as *const f32).read();
        let bias = (global::<f32>(GRID_BIAS) as *const f32).read();
        let tab1 = relocated(GRID_PRIMARY);
        let tab2 = relocated(GRID_PAIRS);
        let pairs = relocated(MARK_PAIRS);
        let flags = relocated(MARK_FLAGS);
        let mut kept = global::<u32>(MARK_COUNT).read();
        for slot in 0..GRID_SLOTS {
            let block = ((tab1 + slot * 4) as *const u32).read();
            if block == 0 {
                continue;
            }
            let gx = f32::from_bits(callee_thiscall!(1, u32, sing, slot & 7));
            let gy = f32::from_bits(callee_thiscall!(1, u32, sing, slot >> 3));
            let xd = if gx > xr_minus { gx - xr_plus } else { xr_minus - (gx + bias) };
            if xd > 0.0 {
                continue;
            }
            let lean = gy + bias;
            let yd = if gy > yr_minus { gy - yr_plus } else { yr_minus - lean };
            // Both sides of the branch reload the edge slot with yr_plus.
            let edge = yr_plus;
            if yd > 0.0 {
                continue;
            }
            let mut out_a = 0u32;
            let mut out_b = 0u32;
            callee_thiscall!(2, u32, sing, slot, yr_minus.to_bits(), edge.to_bits(),
                (&mut out_b as *mut u32) as u32, (&mut out_a as *mut u32) as u32);
            let rec_b = block.wrapping_add(out_a << 5);
            let mut rec = block.wrapping_add(out_b << 5);
            if rec == rec_b {
                continue;
            }
            while rec != rec_b {
                let with_mark = |rx: u32| {
                    let qx = (((rx + 0x14) as *const i16).read() as i32) as f32 * scale;
                    let qy = (((rx + 0x16) as *const i16).read() as i32) as f32 * scale;
                    let ddx = qx - fx;
                    let ddy = qy - fy;
                    ddx * ddx + ddy * ddy
                };
                if !(with_mark(rec) > r2) {
                    let tag = (((rec + 0x1C) as *const u8).read() >> 4) & 0xF;
                    if tag == 0 || tag == 9 {
                        let mates = ((rec + 0x1E) as *const u8).read() & 0xF;
                        if mates > 0 {
                            let arr = ((tab2 + slot * 4) as *const u32).read();
                            let pbase = ((rec + 0x12) as *const i16).read() as i32;
                            let mut j = 0u32;
                            while j < mates as u32 {
                                let pidx = pbase.wrapping_add(j as i32) as u32;
                                let pair = ((arr + pidx * 8) as *const u32).read();
                                let tab = pair & 0xFFFF;
                                let peer = ((tab1 + tab * 4) as *const u32).read();
                                if peer != 0 {
                                    let ridx = (pair >> 16) & 0xFFFF;
                                    let rec2 = peer.wrapping_add(ridx << 5);
                                    if rec < rec2 {
                                        let tag2 = (((rec2 + 0x1C) as *const u8).read() >> 4) & 0xF;
                                        if tag2 == 0 || tag2 == 9 {
                                            if !(with_mark(rec2) > r2) {
                                                if kept >= MARK_CAP {
                                                    return 0;
                                                }
                                                let v0 = rec.wrapping_add(
                                                    0xFFFF_FFECu32.wrapping_sub(block)).wrapping_add(0x14);
                                                let w0 = (v0 >> 5) << 16 | slot;
                                                let w1 = ridx << 16 | tab;
                                                ((pairs + kept * 8) as *mut u32).write(w0);
                                                ((pairs + kept * 8 + 4) as *mut u32).write(w1);
                                                let cb = ((arr + pidx * 8 + 5) as *const u8).read();
                                                ((flags + kept) as *mut u8)
                                                    .write(((cb >> 3) & 7) + (cb & 7));
                                                kept += 1;
                                                global::<u32>(MARK_COUNT).write(kept);
                                            }
                                        }
                                    }
                                }
                                j += 1;
                            }
                        }
                    }
                }
                rec = rec.wrapping_add(0x20);
            }
        }
        0
    }
});
