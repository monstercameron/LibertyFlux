// original: 0x00C5E390 CTaskComplexDestroyCarArmed::vf19

/// Creates the armed destroy-car task's next child. With no current child it
/// returns null. Otherwise it clears the mode byte and performs the ped task
/// setup call. If the stored speed threshold is strictly below the current
/// speed, the byte at +0x68 selects factory type 0x387 or 0x38b. For all
/// other ordered and unordered comparisons it computes a vehicle quadrant
/// blend from four stored floats, writes that exact f32 result to the ped at
/// +0xaa4, then requests factory type 0x3fc. The final factory receives the
/// ped as its second stack argument and its return value is forwarded.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e390(this: u32, ped: u32) -> u32 {
    unsafe {
        const CURRENT_CHILD_OFFSET: u32 = 0x18;
        const MODE_OFFSET: u32 = 0x14;
        const SPEED_OFFSET: u32 = 0x60;
        const SPEED_LIMIT_OFFSET: u32 = 0x58;
        const STYLE_OFFSET: u32 = 0x68;
        const BLEND_OUTPUT_OFFSET: u32 = 0x0aa4;
        const CHILD_SPEED_TYPE: u32 = 0x387;
        const CHILD_IDLE_TYPE: u32 = 0x38b;
        const CHILD_BLEND_TYPE: u32 = 0x3fc;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let current_child = read_u32(this.wrapping_add(CURRENT_CHILD_OFFSET));
        if current_child == 0 {
            return 0;
        }

        (this.wrapping_add(MODE_OFFSET) as *mut u8).write(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, ped);

        let current_speed = f32::from_bits(read_u32(this.wrapping_add(SPEED_OFFSET)));
        let speed_limit = f32::from_bits(read_u32(this.wrapping_add(SPEED_LIMIT_OFFSET)));
        if core::hint::black_box(current_speed) >
            core::hint::black_box(speed_limit) {
            let style = (this.wrapping_add(STYLE_OFFSET) as *const u8).read();
            let child_type = if style != 0 { CHILD_SPEED_TYPE } else { CHILD_IDLE_TYPE };
            return lf_checker_rt::callee_thiscall!(3, u32, this, child_type, ped);
        }

        let blend_a = f32::from_bits(read_u32(this.wrapping_add(0x30)));
        let blend_b = f32::from_bits(read_u32(this.wrapping_add(0x34)));
        let blend_c = f32::from_bits(read_u32(this.wrapping_add(0x20)));
        let blend_d = f32::from_bits(read_u32(this.wrapping_add(0x24)));
        let blend: f32 = lf_checker_rt::callee_cdecl!(2, f32,
            blend_a.to_bits(), blend_b.to_bits(), blend_c.to_bits(), blend_d.to_bits());
        (ped.wrapping_add(BLEND_OUTPUT_OFFSET) as *mut f32).write_unaligned(blend);
        lf_checker_rt::callee_thiscall!(3, u32, this, CHILD_BLEND_TYPE, ped)
    }
}
