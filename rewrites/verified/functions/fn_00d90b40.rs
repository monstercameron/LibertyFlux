// original: 0x00d90b40 audio_search_cell_planes
/// Search a cell node for a plane whose corner midpoint hits the target.
///
/// `node`+0x2c heads the child list: when null the node is a leaf and each
/// of its four children is bound-tested against the shared float box and
/// searched recursively. Otherwise every listed cell whose packed bound
/// box overlaps the query bounds is scanned plane by plane: a plane whose
/// flag word (written by the flag callee) has either of bits 0x6000 set
/// has two corners resolved, and when their midpoint is within 0.1 in x
/// and y and within 2.0 in z of the shared target point, the plane's
/// address is stored to `out` and 1 returned. Only the low byte of the
/// result is meaningful; the upper bits are whatever the last call left.
export!(thiscall, rw_00d90b40(this: u32, node: u32, bounds: *const i16, out: *mut u32) -> u32 {
    let head = unsafe { *((node + 0x2c) as *const u32) };
    if head == 0 {
        for j in 0..4u32 {
            let child = unsafe { *((node + 0x30 + j * 4) as *const u32) };
            // comiss+ja passes unordered NaN through, so `!(a > b)`, not `<=`.
            let c0 = unsafe { *(child as *const f32) };
            let c1 = unsafe { *((child + 4) as *const f32) };
            let c10 = unsafe { *((child + 0x10) as *const f32) };
            let c14 = unsafe { *((child + 0x14) as *const f32) };
            let g = unsafe {
                [
                    *global::<f32>(0x179faf0),
                    *global::<f32>(0x179faf4),
                    *global::<f32>(0x179fb00),
                    *global::<f32>(0x179fb04),
                ]
            };
            if !(g[0] > c10) && !(g[1] > c14) && !(c0 > g[2]) && !(c1 > g[3]) {
                let r = callee_thiscall!(4, u32, this, child, bounds as u32, out as u32);
                if r & 0xFF != 0 {
                    return (r & 0xFFFFFF00) | 1;
                }
            }
        }
        return 0;
    }
    let count = unsafe { *((head + 0xc) as *const u16) } as u32;
    if count == 0 {
        return 0;
    }
    let idx = unsafe { *((head + 0x4) as *const *const u16) };
    let entries = unsafe { *((this + 0x6c) as *const u32) };
    // Masked uninitialized stack slots in the original; the contract
    // defines the fill as zero, leaving exactly these constants.
    let mut fw = [0xFFFF0FFFu32, 0x0FFFFFFFu32];
    let mut k = 0u32;
    while k < count {
        let ei = unsafe { *idx.add(k as usize) } as u32;
        let e = entries.wrapping_add(ei.wrapping_mul(40));
        // Signed 16-bit box compares (jg/jl), like the sibling query.
        let b = |i: usize| unsafe { *bounds.add(i) };
        let m0 = unsafe { *((e + 0x10) as *const i16) };
        let m1 = unsafe { *((e + 0x14) as *const i16) };
        let m2 = unsafe { *((e + 0x18) as *const i16) };
        let x0 = unsafe { *((e + 0x12) as *const i16) };
        let x1 = unsafe { *((e + 0x16) as *const i16) };
        let x2 = unsafe { *((e + 0x1a) as *const i16) };
        if m0 <= b(1) && m1 <= b(3) && m2 <= b(5) && x0 >= b(0) && x1 >= b(2) && x2 >= b(4) {
            let e0 = unsafe { *(e as *const u32) };
            let count2 = (e0 >> 0x15) & 0xf;
            if count2 != 0 {
                let base = unsafe { *((e + 4) as *const u32) } & 0x1ffff;
                let planes = unsafe { *((this + 0x64) as *const u32) };
                let words = unsafe { *((this + 0x60) as *const *const u16) };
                let gx = unsafe { *global::<f32>(0x179fb10) };
                let gy = unsafe { *global::<f32>(0x179fb14) };
                let gz = unsafe { *global::<f32>(0x179fb18) };
                let mut inner = 0u32;
                // First iteration tests the last plane, then plane 0, 1, ...
                let mut pidx = base + count2 - 1;
                while inner < count2 {
                    let plane = planes.wrapping_add(pidx.wrapping_mul(8));
                    callee_thiscall!(1, u32, plane, fw.as_mut_ptr() as u32);
                    if fw[0] & 0x6000 != 0 {
                        let mut fa = [0.0f32; 3];
                        let mut fb = [0.0f32; 3];
                        let wa = unsafe { *words.add(pidx as usize) } as u32;
                        callee_thiscall!(2, u32, this, wa, fa.as_mut_ptr() as u32);
                        let wb = unsafe { *words.add((base + inner) as usize) } as u32;
                        callee_thiscall!(3, u32, this, wb, fb.as_mut_ptr() as u32);
                        let dx = (fadd(fa[0], fb[0]) * 0.5 - gx).abs();
                        if 0.1f32 > dx {
                            let dy = (fadd(fa[1], fb[1]) * 0.5 - gy).abs();
                            if 0.1f32 > dy {
                                let dz = (fadd(fa[2], fb[2]) * 0.5 - gz).abs();
                                if 2.0f32 > dz {
                                    unsafe {
                                        *out = plane;
                                    }
                                    return 1;
                                }
                            }
                        }
                    }
                    pidx = base + inner;
                    inner += 1;
                }
            }
        }
        k += 1;
    }
    0
});
