// original: 0x00879F50 rage::crmtComposerOptimized::route_float_value

/// Append the current track to the composition route and dispatch its value.
///
/// `this` is the composer object. Its route-list pointer is at byte offset
/// `0x0c`, and the 16-bit pending-item count is at `0x72`. The list has a
/// four-byte pointer to its track-pointer array at `+0x04` and a 32-bit item
/// count at `+0x14`. The routine increments the pending-item count, decrements
/// the list count with 32-bit wraparound, then reads the new last track and
/// the former last track from the array. It passes a pointer to the new-last
/// track through the callee's `this` register, forwards the former-last
/// pointer and the incoming stack word, and forwards the low single-precision
/// value from XMM1 in XMM3. The intercepted callee's 32-bit return is returned
/// unchanged. The array is intentionally addressed with wrapping 32-bit
/// arithmetic, matching the original's pointer-width behavior.
///
/// The internal dispatch uses two integer words on the stack and a low-f32
/// value in XMM3; the remaining XMM lanes are not read by that target.
lf_checker_rt::export!(thiscall, rw_00879f50(this: u32, forwarded_value: u32) -> u32 {
    unsafe {
        const ROUTE_LIST: u32 = 0x0c;
        const PENDING_COUNT: u32 = 0x72;
        const TRACK_ARRAY: u32 = 0x04;
        const TRACK_COUNT: u32 = 0x14;
        const WORD_SIZE: u32 = 4;
        const ROUTE_FLOAT_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        // Update the short pending count before touching the route list.
        let pending_address = this.wrapping_add(PENDING_COUNT);
        write_u16(pending_address, read_u16(pending_address).wrapping_add(1));

        let route_list = read_u32(this.wrapping_add(ROUTE_LIST));
        let count_address = route_list.wrapping_add(TRACK_COUNT);
        let previous_count = read_u32(count_address);
        write_u32(count_address, previous_count.wrapping_sub(1));
        let new_count = read_u32(count_address);

        let tracks = read_u32(route_list.wrapping_add(TRACK_ARRAY));
        let new_last = read_u32(tracks.wrapping_add(new_count.wrapping_mul(WORD_SIZE)));
        let former_last = read_u32(tracks.wrapping_add(previous_count.wrapping_mul(WORD_SIZE)));

        let mut selected_track = new_last;
        type RouteFloatRequest = unsafe extern "vectorcall" fn(
            u32,
            u32,
            u32,
            u32,
            f32,
            f32,
            f32,
            f32,
        ) -> u32;
        let route_float: RouteFloatRequest = core::mem::transmute(
            lf_checker_rt::callee_addr(ROUTE_FLOAT_CALLEE) as usize,
        );
        let value = f32::from_bits(lf_checker_rt::xmm_word(1, 0));
        route_float(
            (&mut selected_track as *mut u32) as u32,
            0,
            former_last,
            forwarded_value,
            0.0,
            0.0,
            0.0,
            value,
        )
    }
});
