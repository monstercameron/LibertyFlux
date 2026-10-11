// original: 0x00b45320 CEventGunShot::CEventGunShot
/// Rebuilds this event object in place with the original MSVC thiscall layout.
/// ECX is the receiver; each named parameter is one incoming 32-bit stack word.
/// calls the editable base constructor, then installs its own vtable; shooter_reference is stored at +0x18 (4 byte(s); first_record supplies 3 dwords at +0x20; second_record supplies 3 dwords at +0x30; bullet_kind is stored at +0x40 (1 byte(s); +0x41 receives constant 0x1; +0x43 receives constant 0x0; +0x44 receives constant 0x0; sequence is stored at +0x48 (4 byte(s); reference-link calls use the object field address as their stack argument and the stored reference as ECX.
/// The returned EAX value is the receiver.
lf_checker_rt::export!(thiscall, rw_00b45320(this: u32, shooter_reference: u32, first_record: u32, second_record: u32, bullet_kind: u32, sequence: u32) -> u32 {
        const VTABLE_FIELD: u32 = 0x00000000;
        const VTABLE_FILE_VA: u32 = 0x00ead934;
        const FIELD_20: u32 = 0x00000020;
        const FIELD_40: u32 = 0x00000040;
        const FIELD_41: u32 = 0x00000041;
        const FIELD_43: u32 = 0x00000043;
        const FIELD_44: u32 = 0x00000044;
        const FIELD_48: u32 = 0x00000048;
        const FIELD_30: u32 = 0x00000030;
        const FIELD_18: u32 = 0x00000018;
        const GLOBAL_EVENT_TOKEN: u32 = 0x011735b4;
        const INVALID_SENTINEL: u32 = 0xffffffff;
        const GLOBAL_011735B4: u32 = 0x011735b4;
        const GLOBAL_016660F0: u32 = 0x016660f0;
        const VALUE_00000001: u32 = 0x00000001;
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
        write_u32(this.wrapping_add(FIELD_18), shooter_reference);
        copy_words(this.wrapping_add(FIELD_20), first_record, 3);
        copy_words(this.wrapping_add(FIELD_30), second_record, 3);
        write_u8(this.wrapping_add(FIELD_40), bullet_kind as u8);
        write_u16(this.wrapping_add(FIELD_41), VALUE_00000001 as u16);
        write_u8(this.wrapping_add(FIELD_43), VALUE_00000000 as u8);
        write_u32(this.wrapping_add(FIELD_44), VALUE_00000000 as u32);
        write_u32(this.wrapping_add(FIELD_48), sequence);
        lf_checker_rt::global::<u32>(GLOBAL_016660F0).write_unaligned(lf_checker_rt::global::<u32>(GLOBAL_011735B4).read_unaligned());
        if read_u32(this.wrapping_add(FIELD_18)) != 0 {
            let _link_result = lf_checker_rt::callee_thiscall!(2, u32, read_u32(this.wrapping_add(FIELD_18)), this.wrapping_add(FIELD_18));
        }
        this
    }
});
