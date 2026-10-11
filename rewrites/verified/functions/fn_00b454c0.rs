// original: 0x00b454c0 CEventObjectCollision::CEventObjectCollision
/// Rebuilds this event object in place with the original MSVC thiscall layout.
/// ECX is the receiver; each named parameter is one incoming 32-bit stack word.
/// calls the event base constructor, then installs its own vtable; impact_kind is stored at +0x10 (2 byte(s); impact_bits is stored at +0x0C (4 byte(s); collision_reference is stored at +0x14 (4 byte(s); first_record supplies 3 dwords at +0x20; second_record supplies 3 dwords at +0x30; flags is stored at +0x12 (2 byte(s); +0x40 receives constant 0x0; reference-link calls use the object field address as their stack argument and the stored reference as ECX.
/// The returned EAX value is the receiver. Raw floating inputs are stored without conversion.
lf_checker_rt::export!(thiscall, rw_00b454c0(this: u32, impact_kind: u32, impact_bits: u32, collision_reference: u32, first_record: u32, second_record: u32, flags: u32) -> u32 {
        const VTABLE_FIELD: u32 = 0x00000000;
        const VTABLE_FILE_VA: u32 = 0x00eadaf4;
        const FIELD_20: u32 = 0x00000020;
        const FIELD_40: u32 = 0x00000040;
        const FIELD_0C: u32 = 0x0000000c;
        const FIELD_10: u32 = 0x00000010;
        const FIELD_30: u32 = 0x00000030;
        const FIELD_12: u32 = 0x00000012;
        const FIELD_14: u32 = 0x00000014;
        const GLOBAL_EVENT_TOKEN: u32 = 0x011735b4;
        const INVALID_SENTINEL: u32 = 0xffffffff;
        const VALUE_00000000: u32 = 0x00000000;
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
        write_u16(this.wrapping_add(FIELD_10), impact_kind as u16);
        write_u32(this.wrapping_add(FIELD_0C), impact_bits);
        write_u32(this.wrapping_add(FIELD_14), collision_reference);
        copy_words(this.wrapping_add(FIELD_20), first_record, 3);
        copy_words(this.wrapping_add(FIELD_30), second_record, 3);
        write_u16(this.wrapping_add(FIELD_12), flags as u16);
        write_u8(this.wrapping_add(FIELD_40), VALUE_00000000 as u8);
        if read_u32(this.wrapping_add(FIELD_14)) != 0 {
            let _link_result = lf_checker_rt::callee_thiscall!(2, u32, read_u32(this.wrapping_add(FIELD_14)), this.wrapping_add(FIELD_14));
        }
        this
    }
});
