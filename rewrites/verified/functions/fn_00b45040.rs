// original: 0x00b45040 CEventClimbLadderOnRoute::CEventClimbLadderOnRoute
/// Rebuilds this event object in place with the original MSVC thiscall layout.
/// ECX is the receiver; each named parameter is one incoming 32-bit stack word.
/// calls the event base constructor, then installs its own vtable; first_record supplies 3 dwords at +0x10; second_record supplies 3 dwords at +0x20; kind is stored at +0x30 (4 byte(s); weight_bits is stored at +0x34 (4 byte(s); flags is stored at +0x38 (4 byte(s).
/// The returned EAX value is the receiver. Raw floating inputs are stored without conversion.
lf_checker_rt::export!(thiscall, rw_00b45040(this: u32, first_record: u32, second_record: u32, kind: u32, weight_bits: u32, flags: u32) -> u32 {
        const VTABLE_FIELD: u32 = 0x00000000;
        const VTABLE_FILE_VA: u32 = 0x00eae724;
        const FIELD_20: u32 = 0x00000020;
        const FIELD_10: u32 = 0x00000010;
        const FIELD_30: u32 = 0x00000030;
        const FIELD_34: u32 = 0x00000034;
        const FIELD_38: u32 = 0x00000038;
        const GLOBAL_EVENT_TOKEN: u32 = 0x011735b4;
        const INVALID_SENTINEL: u32 = 0xffffffff;
        let _ = GLOBAL_EVENT_TOKEN;
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
        copy_words(this.wrapping_add(FIELD_10), first_record, 3);
        copy_words(this.wrapping_add(FIELD_20), second_record, 3);
        write_u32(this.wrapping_add(FIELD_30), kind);
        write_u32(this.wrapping_add(FIELD_34), weight_bits);
        write_u32(this.wrapping_add(FIELD_38), flags);
        this
    }
});
