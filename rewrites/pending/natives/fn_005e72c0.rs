// original: 0x005e72c0 GET_MOBILE_PHONE_RENDER_ID
/// Script native `GET_MOBILE_PHONE_RENDER_ID` (hash 0x5E7B3816).
///
/// Makes no engine call: copies the phone render-id global into the output word addressed by the first script argument.
export!(cdecl, rw_005e72c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let out = *args as *mut u32;
        let value = *global::<u32>(0x018B6EF4);
        *out = value;
        value
    }
});
