// original: 0x00d7a960 blend_probed_score_by_distance
use lf_checker_rt::{export, global};

/// Distance-attenuated blend of a probed score with a default value.
///
/// `obj_a` anchors two vectors (an axis at +0x10 and a point at +0x30,
/// reached through the link at +0x20). `obj_c` is an object with a vtable;
/// the same virtual query (slot 0xEC) is invoked twice with a scratch probe
/// buffer. When `obj_c` is null, when the probe point is farther than the
/// limit (70.0) from the anchor point, or when the score exceeds the
/// default, the default is returned unchanged. Otherwise the result blends
/// the score (at distance 0) into the default (at the limit).
///
/// Argument order (cdecl): anchor object, queried object (nullable),
/// default, score floor, score bias, all as raw words; floats travel as
/// their bit patterns. Returns the blended `f32` in ST0.
export!(cdecl, rw_d7a960(
    obj_a: u32,
    obj_c: u32,
    default_bits: u32,
    min_bits: u32,
    add_bits: u32,
) -> f32 {
    unsafe {
        let default = f32::from_bits(default_bits);
        if obj_c == 0 {
            return default;
        }
        let vtable = *(obj_c as *const u32);
        let target = *((vtable as *const u8).add(0xEC) as *const u32);
        let query: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let mut probe = [0u32; 4];
        query(obj_c, probe.as_mut_ptr() as u32);
        let inner = *((obj_c as *const u8).add(0x20) as *const u32);
        let base: *const u8 = if inner == 0 {
            (obj_c as *const u8).add(0x10)
        } else {
            (inner as *const u8).add(0x30)
        };
        let obj_b = *((obj_a as *const u8).add(0x20) as *const u32);
        let anchor = (obj_b as *const u8).add(0x30);
        let dx = f32::from_bits(*(base as *const u32))
            - f32::from_bits(*(anchor as *const u32));
        let dy = f32::from_bits(*((base as *const u8).add(4) as *const u32))
            - f32::from_bits(*((anchor as *const u8).add(4) as *const u32));
        let dz = f32::from_bits(*((base as *const u8).add(8) as *const u32))
            - f32::from_bits(*((anchor as *const u8).add(8) as *const u32));
        let dist_sq = (dy * dy + dx * dx) + dz * dz;
        let dist = dist_sq.sqrt();
        let limit = *global::<f32>(0xFE8B90);
        if dist > limit {
            return default;
        }
        let dir = query(obj_c, probe.as_mut_ptr() as u32);
        let axis = (obj_b as *const u8).add(0x10);
        let vx = f32::from_bits(*(axis as *const u32));
        let vy = f32::from_bits(*((axis as *const u8).add(4) as *const u32));
        let vz = f32::from_bits(*((axis as *const u8).add(8) as *const u32));
        let rx = f32::from_bits(*(dir as *const u32));
        let ry = f32::from_bits(*((dir as *const u8).add(4) as *const u32));
        let rz = f32::from_bits(*((dir as *const u8).add(8) as *const u32));
        let mut score = (ry * vy + vx * rx) + rz * vz;
        score += f32::from_bits(add_bits);
        let floor = f32::from_bits(min_bits);
        if !(score > floor) {
            score = floor;
        }
        if score > default {
            return default;
        }
        let step = *global::<f32>(0xE7CB80);
        let t = dist * step;
        let rest = *global::<f32>(0xFE88E8) - t;
        rest * score + t * default
    }
});
