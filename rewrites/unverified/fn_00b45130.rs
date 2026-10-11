// original: 0x00b45130 CEventCommunicateEvent::CEventCommunicateEvent
/// Rebuilds this event object in place with the original MSVC thiscall layout.
/// ECX is the receiver; each named parameter is one incoming 32-bit stack word.
/// calls the event base constructor, then installs its own vtable; event_reference is stored at +0x0C (4 byte(s); event_type is stored at +0x10 (4 byte(s); distance_bits is stored at +0x14 (4 byte(s); +0x30 receives constant 0x0; +0x34 receives constant 0x0; when +0x18 is the sentinel and the event pointer is non-null, it reads the global token and copies four dwords from the nested record or the event record fallback; flags is stored at +0x34 (4 byte(s); context_record supplies 3 dwords at +0x50; +0x38 receives constant 0x0; reference-link calls use the object field address as their stack argument and the stored reference as ECX.
/// The returned EAX value is the receiver. Raw floating inputs are stored without conversion.
lf_checker_rt::export!(thiscall, rw_00b45130(this: u32, event_reference: u32, event_type: u32, distance_bits: u32, flags: u32, context_record: u32) -> u32 {
        const VTABLE_FIELD: u32 = 0x00000000;
        const VTABLE_FILE_VA: u32 = 0x00eae50c;
        const FIELD_20: u32 = 0x00000020;
        const FIELD_0C: u32 = 0x0000000c;
        const FIELD_10: u32 = 0x00000010;
        const FIELD_30: u32 = 0x00000030;
        const FIELD_50: u32 = 0x00000050;
        const FIELD_14: u32 = 0x00000014;
        const FIELD_34: u32 = 0x00000034;
        const FIELD_38: u32 = 0x00000038;
        const FIELD_18: u32 = 0x00000018;
        const GLOBAL_EVENT_TOKEN: u32 = 0x011735b4;
        const INVALID_SENTINEL: u32 = 0xffffffff;
        const VALUE_00000000: u32 = 0x00000000;
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
        write_u32(this.wrapping_add(FIELD_0C), event_reference);
        write_u32(this.wrapping_add(FIELD_10), event_type);
        write_u32(this.wrapping_add(FIELD_14), distance_bits);
        write_u32(this.wrapping_add(FIELD_30), VALUE_00000000 as u32);
        write_u32(this.wrapping_add(FIELD_34), VALUE_00000000 as u32);
        if read_u32(this.wrapping_add(FIELD_18)) == INVALID_SENTINEL && event_reference != 0 {
            write_u32(this.wrapping_add(FIELD_18), lf_checker_rt::global::<u32>(GLOBAL_EVENT_TOKEN).read_unaligned());
            let nested = read_u32(event_reference.wrapping_add(FIELD_20));
            let source = if nested != 0 { nested.wrapping_add(FIELD_30) } else { event_reference.wrapping_add(FIELD_10) };
            copy_words(this.wrapping_add(FIELD_20), source, 4);
        }
        write_u32(this.wrapping_add(FIELD_34), flags);
        copy_words(this.wrapping_add(FIELD_50), context_record, 3);
        write_u8(this.wrapping_add(FIELD_38), VALUE_00000000 as u8);
        if read_u32(this.wrapping_add(FIELD_0C)) != 0 {
            let _link_result = lf_checker_rt::callee_thiscall!(2, u32, read_u32(this.wrapping_add(FIELD_0C)), this.wrapping_add(FIELD_0C));
        }
        this
    }
});
