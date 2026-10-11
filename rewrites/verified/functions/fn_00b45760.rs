// original: 0x00b45760 CEventPotentialWalkIntoFire::CEventPotentialWalkIntoFire
/// Rebuilds this event object in place with the original MSVC thiscall layout.
/// ECX is the receiver; each named parameter is one incoming 32-bit stack word.
/// calls the editable base constructor, then installs its own vtable; fire_source supplies 3 dwords at +0x20; component_bits is stored at +0x30 (4 byte(s); flags is stored at +0x38 (4 byte(s); selects 0.7 below 1.0, 1.0 below 2.0, otherwise 1.5 (including unordered NaN), then adds 0.25 in that operand order.
/// The returned EAX value is the receiver. Raw floating inputs are stored without conversion.
lf_checker_rt::export!(thiscall, rw_00b45760(this: u32, fire_source: u32, component_bits: u32, flags: u32) -> u32 {
        const VTABLE_FIELD: u32 = 0x00000000;
        const VTABLE_FILE_VA: u32 = 0x00eae38c;
        const FIELD_20: u32 = 0x00000020;
        const FIELD_30: u32 = 0x00000030;
        const FIELD_34: u32 = 0x00000034;
        const FIELD_38: u32 = 0x00000038;
        const GLOBAL_EVENT_TOKEN: u32 = 0x011735b4;
        const INVALID_SENTINEL: u32 = 0xffffffff;
        let _ = GLOBAL_EVENT_TOKEN;
        const BELOW_ONE: f32 = 1.0;
        const BELOW_TWO: f32 = 2.0;
        const LOW_COMPONENT: f32 = 0.7;
        const MID_COMPONENT: f32 = 1.0;
        const HIGH_COMPONENT: f32 = 1.5;
        const COMPONENT_BIAS: f32 = 0.25;
        #[inline(always)] fn ordered_add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)] unsafe fn read_u32(address: u32) -> u32 { core::ptr::read_unaligned(address as *const u32) }
        #[inline(always)] unsafe fn write_u32(address: u32, value: u32) { core::ptr::write_unaligned(address as *mut u32, value); }
        #[inline(always)] unsafe fn write_u16(address: u32, value: u16) { core::ptr::write_unaligned(address as *mut u16, value); }
        #[inline(always)] unsafe fn write_u8(address: u32, value: u8) { core::ptr::write(address as *mut u8, value); }
        #[inline(always)] unsafe fn copy_words(destination: u32, source: u32, count: usize) {
            for index in 0..count {
                let value = read_u32(source.wrapping_add((index as u32).wrapping_mul(4)));
                write_u32(destination.wrapping_add((index as u32).wrapping_mul(4)), value);
            }
        }
    unsafe {
        let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
        write_u32(this.wrapping_add(VTABLE_FIELD), lf_checker_rt::relocated(VTABLE_FILE_VA));
        copy_words(this.wrapping_add(FIELD_20), fire_source, 3);
        write_u32(this.wrapping_add(FIELD_30), component_bits);
        write_u32(this.wrapping_add(FIELD_38), flags);
        let component = f32::from_bits(component_bits);
        let selected = if component < BELOW_ONE { LOW_COMPONENT } else if component < BELOW_TWO { MID_COMPONENT } else { HIGH_COMPONENT };
        let adjusted = ordered_add(selected, COMPONENT_BIAS);
        write_u32(this.wrapping_add(FIELD_34), adjusted.to_bits());
        this
    }
});
