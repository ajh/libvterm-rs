use libc::{c_int, uint32_t, size_t, c_char, c_uchar};

pub enum VTerm {}

#[repr(C)]
pub enum VTermModifier {
  VTERM_MOD_NONE  = 0x00,
  VTERM_MOD_SHIFT = 0x01,
  VTERM_MOD_ALT   = 0x02,
  VTERM_MOD_CTRL  = 0x04,
}

#[repr(C)]
pub enum VTermKey {
  VTERM_KEY_NONE,

  VTERM_KEY_ENTER,
  VTERM_KEY_TAB,
  VTERM_KEY_BACKSPACE,
  VTERM_KEY_ESCAPE,

  VTERM_KEY_UP,
  VTERM_KEY_DOWN,
  VTERM_KEY_LEFT,
  VTERM_KEY_RIGHT,

  VTERM_KEY_INS,
  VTERM_KEY_DEL,
  VTERM_KEY_HOME,
  VTERM_KEY_END,
  VTERM_KEY_PAGEUP,
  VTERM_KEY_PAGEDOWN,

  VTERM_KEY_FUNCTION_0   = 256,
  VTERM_KEY_FUNCTION_MAX = 256 + 255,

  VTERM_KEY_KP_0,
  VTERM_KEY_KP_1,
  VTERM_KEY_KP_2,
  VTERM_KEY_KP_3,
  VTERM_KEY_KP_4,
  VTERM_KEY_KP_5,
  VTERM_KEY_KP_6,
  VTERM_KEY_KP_7,
  VTERM_KEY_KP_8,
  VTERM_KEY_KP_9,
  VTERM_KEY_KP_MULT,
  VTERM_KEY_KP_PLUS,
  VTERM_KEY_KP_COMMA,
  VTERM_KEY_KP_MINUS,
  VTERM_KEY_KP_PERIOD,
  VTERM_KEY_KP_DIVIDE,
  VTERM_KEY_KP_ENTER,
  VTERM_KEY_KP_EQUAL,

  VTERM_KEY_MAX, // Must be last
}

extern {
    pub fn vterm_new(rows: c_int, cols: c_int) -> *mut VTerm;
    pub fn vterm_free(vt: *mut VTerm);

    pub fn vterm_get_size(vt: *const VTerm, rowsp: *mut c_int, colsp: *mut c_int);
    pub fn vterm_set_size(vt: *mut VTerm, rows: c_int, cols: c_int);

    pub fn vterm_get_utf8(vt: *const VTerm) -> c_int;
    pub fn vterm_set_utf8(vt: *mut VTerm, is_utf8: c_int);

    pub fn vterm_input_write(vt: *mut VTerm, bytes: *const c_uchar, len: size_t) -> size_t;

    pub fn vterm_output_get_buffer_size(vt: *const VTerm) -> size_t;
    pub fn vterm_output_get_buffer_current(vt: *const VTerm) -> size_t;
    pub fn vterm_output_get_buffer_remaining(vt: *const VTerm) -> size_t;

    pub fn vterm_output_read(vt: *mut VTerm, buffer: *mut c_char, len: size_t) -> size_t;

    pub fn vterm_keyboard_unichar(vt: *mut VTerm, c: uint32_t, modifier: VTermModifier);
    pub fn vterm_keyboard_key(vt: *mut VTerm, key: VTermKey, modifier: VTermModifier);

    pub fn vterm_keyboard_start_paste(vt: *mut VTerm);
    pub fn vterm_keyboard_end_paste(vt: *mut VTerm);

    pub fn vterm_mouse_move(vt: *mut VTerm, row: c_int, col: c_int, modifier: VTermModifier);
    pub fn vterm_mouse_button(vt: *mut VTerm, button: c_int, pressed: bool, modifier: VTermModifier);
}

mod tests {
    use libc::{c_int, c_uchar};
    use super::*;

    #[test]
    fn ffi_vterm_can_create_and_destroy() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn ffi_vterm_can_get_size() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let mut cols: c_int = 0;
            let mut rows: c_int = 0;
            vterm_get_size(vterm_ptr, &mut cols, &mut rows);
            assert_eq!((2, 2), (cols, rows));

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn ffi_vterm_can_set_size() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_set_size(vterm_ptr, 1, 1);

            let mut cols: c_int = 0;
            let mut rows: c_int = 0;
            vterm_get_size(vterm_ptr, &mut cols, &mut rows);
            assert_eq!((1, 1), (cols, rows));

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn ffi_vterm_can_get_and_set_utf8() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);

            vterm_set_utf8(vterm_ptr, 0);
            assert_eq!(0, vterm_get_utf8(vterm_ptr));

            // vterm represents this as "int utf8: 1" which means it is either 0 or -1.
            vterm_set_utf8(vterm_ptr, -1);
            assert_eq!(-1, vterm_get_utf8(vterm_ptr));

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn ffi_vterm_can_write_input() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);

            // there probably a nicer way to do this
            let input = [
                b'a' as c_uchar,
                b'b' as c_uchar,
                b'c' as c_uchar,
            ];
            let bytes_read = vterm_input_write(vterm_ptr, input.as_ptr(), 3);
            assert_eq!(3, bytes_read);
            vterm_free(vterm_ptr);
        }
    }
}
