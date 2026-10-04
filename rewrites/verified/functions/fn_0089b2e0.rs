// original: 0x0089b2e0 aud_sound_read_params
/// Copies an object's tuning fields into five caller buffers.
///
/// First copies the stored words, then, for each live float source linked
/// from the object, overwrites the matching buffer with the scaled value
/// truncated to an integer (truncation matches `cvttss2si`, including NaN
/// and overflow mapping to `i32::MIN`). Null links keep the stored value.
/// Returns the last linked conversion, or zero when that link is null.
export!(thiscall, rw_0089b2e0(
    this: *const u8,
    o1: *mut u32,
    o2: *mut u32,
    o3: *mut u32,
    o4: *mut u32,
    o5: *mut u32,
) -> u32 {
    /// Truncating float-to-integer conversion with x86 `cvttss2si`
    /// semantics: NaN, infinities and out-of-range values convert to
    /// `i32::MIN`; in-range values truncate toward zero. The negative
    /// bound is written folded since -2^31 itself converts to `i32::MIN`.
    fn cvtt_ss2si(value: f32) -> i32 {
        let truncated = value.trunc();
        if truncated.is_nan() {
            return i32::MIN;
        }
        if truncated >= 2147483648.0f32 {
            return i32::MIN;
        }
        if truncated <= -2147483648.0f32 {
            return i32::MIN;
        }
        truncated as i32
    }
    unsafe {
        let scale = *global::<f32>(0x00FE8C58);
        *o1 = core::ptr::read_unaligned(this.add(0xF0) as *const u16) as u32;
        *o2 = core::ptr::read_unaligned(this.add(0xF2) as *const u16) as u32;
        *o3 = *this.add(0xF4) as u32;
        *o4 = *(this.add(0xE8) as *const u32);
        *o5 = *(this.add(0xEC) as *const u32);
        let link = *(this.add(0xD4) as *const u32);
        if link != 0 {
            let scaled = *(link as *const f32) * scale;
            *o1 = cvtt_ss2si(scaled) as u16 as u32;
        }
        let link = *(this.add(0xD8) as *const u32);
        if link != 0 {
            let scaled = *(link as *const f32) * scale;
            *o2 = cvtt_ss2si(scaled) as u16 as u32;
        }
        let link = *(this.add(0xE0) as *const u32);
        if link != 0 {
            *o3 = cvtt_ss2si(*(link as *const f32)) as u8 as u32;
        }
        let link = *(this.add(0xDC) as *const u32);
        if link != 0 {
            let scaled = *(link as *const f32) * scale;
            *o4 = cvtt_ss2si(scaled) as u32;
        }
        let link = *(this.add(0xE4) as *const u32);
        if link != 0 {
            let scaled = *(link as *const f32) * scale;
            let converted = cvtt_ss2si(scaled) as u32;
            *o5 = converted;
            converted
        } else {
            0
        }
    }
});

