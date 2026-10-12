// original: 0x00DB1DC0 UILayoutFrame::sync_cached_child_metrics

/// Recompute the cached child rectangle when the receiver's virtual metric
/// matches the child's key. The receiver holds its virtual table at +0 and
/// the child pointer at +0xEC. The child key is at +8 and its low-byte flags
/// at +4. The receiver stores width, height, center-x and center-y at +4,
/// +8, +0xC and +0x10. Flag bits 0x02/0x08 choose the left/right x edge;
/// bits 0x04/0x10 choose the upper/lower y edge, with the first bit in each
/// pair taking precedence. The half-size multiplier is read from the image.
/// The method's incidental EAX value is not a stable return value.
lf_checker_rt::export!(thiscall, rw_00db1dc0(this: u32) -> u32 {
    const CHILD_POINTER: u32 = 0xEC;
    const CHILD_KEY: u32 = 0x08;
    const CHILD_FLAGS: u32 = 0x04;
    const CHILD_X: u32 = 0x0C;
    const CHILD_Y: u32 = 0x10;
    const VTABLE_METRIC_SLOT: u32 = 0x4C;
    const HALF_SIZE_FILE_VA: u32 = 0x00FE8830;
    const EDGE_X_MINUS: u8 = 0x02;
    const EDGE_X_PLUS: u8 = 0x08;
    const EDGE_Y_MINUS: u8 = 0x04;
    const EDGE_Y_PLUS: u8 = 0x10;

    #[inline(always)]
    fn multiply_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn subtract_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    unsafe {
        let child = ((this.wrapping_add(CHILD_POINTER)) as *const u32).read_unaligned();
        if child == 0 {
            return 0;
        }

        let vtable = (this as *const u32).read_unaligned();
        let metric_address = (vtable.wrapping_add(VTABLE_METRIC_SLOT) as *const u32).read_unaligned();
        let get_metric: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(metric_address as usize);
        let metric = get_metric(this);
        let child_key = (child.wrapping_add(CHILD_KEY) as *const u32).read_unaligned();
        if child_key != metric {
            return 0;
        }

        let center_x = (this.wrapping_add(0x0C) as *const f32).read_unaligned();
        let center_y = (this.wrapping_add(0x10) as *const f32).read_unaligned();
        let width = (this.wrapping_add(0x04) as *const f32).read_unaligned();
        let height = (this.wrapping_add(0x08) as *const f32).read_unaligned();
        let half_size = lf_checker_rt::global::<f32>(HALF_SIZE_FILE_VA).read_unaligned();
        let half_width = multiply_f32(width, half_size);
        let half_height = multiply_f32(height, half_size);

        (child.wrapping_add(CHILD_X) as *mut f32).write_unaligned(center_x);
        (child.wrapping_add(CHILD_Y) as *mut f32).write_unaligned(center_y);

        let flags = (child.wrapping_add(CHILD_FLAGS) as *const u8).read_unaligned();
        if flags & EDGE_X_MINUS != 0 {
            (child.wrapping_add(CHILD_X) as *mut f32)
                .write_unaligned(subtract_f32(center_x, half_width));
        } else if flags & EDGE_X_PLUS != 0 {
            (child.wrapping_add(CHILD_X) as *mut f32)
                .write_unaligned(add_f32(center_x, half_width));
        }

        if flags & EDGE_Y_MINUS != 0 {
            (child.wrapping_add(CHILD_Y) as *mut f32)
                .write_unaligned(subtract_f32(center_y, half_height));
        } else if flags & EDGE_Y_PLUS != 0 {
            (child.wrapping_add(CHILD_Y) as *mut f32)
                .write_unaligned(add_f32(center_y, half_height));
        }
    }
    0
});
