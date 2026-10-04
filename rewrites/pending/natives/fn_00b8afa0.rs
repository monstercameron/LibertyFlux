// original: 0x00b8afa0 ADD_WIDGET_FLOAT_SLIDER
// rw_add_widget_float_slider: native ADD_WIDGET_FLOAT_SLIDER (handler 0x00B8AFA0).
//
// Widget float slider: forwards a tag, an int and three float args to the widget dispatcher, stores the result in the return slot.
export!(cdecl, rw_add_widget_float_slider(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4));
        *(*ctx as *mut u32) = ans;
        ans
    }
});
