// original: 0x0089A480 rage::audLoopingSound::vf7
/// Attach a resolved voice to a looping sound node.
///
/// `this` is the 32-bit sound-node object. The third stack argument points
/// to a 24-byte voice description copied, in three ordered 64-bit pieces,
/// into the node's voice fields. The description pointer stored at `+0x94`
/// supplies a signed mode, an unsigned mode, and a resource pointer.
///
/// The method first asks the base sound code whether the request is valid.
/// Rejection returns false without writing this object. On acceptance it
/// copies the voice data, records the two modes and resource pointer, then
/// asks the voice manager for a resolved voice. A missing result stores the
/// `0xff` slot sentinel. Otherwise it subtracts the selected channel's slot
/// base, divides by the runtime slot stride, stores the low byte as the slot,
/// and reports success only when rebuilding that address is nonzero. The
/// division is unsigned and faults when the runtime stride is zero.
///
/// Calling convention: thiscall, `this` in ECX, three stack words, callee
/// cleanup of 12 bytes. The two outgoing calls are intercepted by contract.
lf_checker_rt::export!(thiscall, rw_0089a480(
    this: u32,
    request_mode: u32,
    request_flags: u32,
    voice_data: u32,
) -> u32 {
    unsafe {
        const VOICE_INFO: u32 = 0x94;
        const CHANNEL: u32 = 0x40;
        const VOICE_SLOT: u32 = 0x48;
        const VOICE_DATA: u32 = 0xb0;
        const SIGNED_MODE: u32 = 0xc8;
        const UNSIGNED_MODE: u32 = 0xcc;
        const RESOURCE: u32 = 0xd4;
        const VOICE_RECORD_STRIDE: u32 = 0x6f40;
        const VOICE_ARRAY_OFFSET: u32 = 0x6f10;
        const VOICE_TABLE_GLOBAL: u32 = 0x0115_d988;
        const VOICE_STRIDE_GLOBAL: u32 = 0x0115_d964;
        const VOICE_MANAGER: u32 = 0x0115_dc18;
        const NO_SLOT: u8 = 0xff;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u64(address: u32) -> u64 {
            unsafe { (address as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_u64(address: u32, value: u64) {
            unsafe { (address as *mut u64).write_unaligned(value) }
        }

        let accepted = lf_checker_rt::callee_thiscall!(
            1,
            u8,
            this,
            request_mode,
            request_flags,
            voice_data,
        );
        if accepted == 0 {
            return 0;
        }

        let info = read_u32(this.wrapping_add(VOICE_INFO));

        // The copy is deliberately sequenced so overlapping input and output
        // ranges retain the byte-for-byte ordering of the original routine.
        let mut piece = 0;
        while piece < 3 {
            let offset = piece * 8;
            let source = core::hint::black_box(voice_data.wrapping_add(offset));
            let value = read_u64(source);
            let destination = core::hint::black_box(VOICE_DATA.wrapping_add(this).wrapping_add(offset));
            write_u64(destination, value);
            if piece == 1 {
                let flags = read_u8(this.wrapping_add(0x3a)) | 4;
                write_u8(this.wrapping_add(0x3a), flags);
            }
            piece += 1;
        }

        let signed_mode = read_u16(info) as i16 as i32 as u32;
        let unsigned_mode = read_u16(info.wrapping_add(2)) as u32;
        let resource = read_u32(info.wrapping_add(4));
        write_u32(this.wrapping_add(SIGNED_MODE), signed_mode);
        write_u32(this.wrapping_add(UNSIGNED_MODE), unsigned_mode);
        write_u32(this.wrapping_add(RESOURCE), resource);

        let manager = lf_checker_rt::relocated(VOICE_MANAGER);
        let created = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            manager,
            resource,
            this,
            request_flags,
            voice_data,
        );
        let slot = if created == 0 {
            NO_SLOT
        } else {
            let channel = read_u8(this.wrapping_add(CHANNEL)) as u32;
            let table = read_u32(lf_checker_rt::relocated(VOICE_TABLE_GLOBAL));
            let slot_record = table
                .wrapping_add(channel.wrapping_mul(VOICE_RECORD_STRIDE))
                .wrapping_add(VOICE_ARRAY_OFFSET);
            let slot_stride = read_u32(lf_checker_rt::relocated(VOICE_STRIDE_GLOBAL));
            let slot_base = read_u32(slot_record);
            created.wrapping_sub(slot_base).wrapping_div(slot_stride) as u8
        };
        write_u8(this.wrapping_add(VOICE_SLOT), slot);
        if slot == NO_SLOT {
            return 0;
        }

        let channel = read_u8(this.wrapping_add(CHANNEL)) as u32;
        let table = read_u32(lf_checker_rt::relocated(VOICE_TABLE_GLOBAL));
        let slot_record = table
            .wrapping_add(channel.wrapping_mul(VOICE_RECORD_STRIDE))
            .wrapping_add(VOICE_ARRAY_OFFSET);
        let slot_base = read_u32(slot_record);
        let slot_stride = read_u32(lf_checker_rt::relocated(VOICE_STRIDE_GLOBAL));
        let resolved_slot = slot_base.wrapping_add(u32::from(slot).wrapping_mul(slot_stride));
        u32::from(resolved_slot != 0)
    }
});
