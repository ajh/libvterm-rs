#include "libvterm/src/vterm_internal.h"

// Work around rust ffi compatibility issues with c bit fields
VTermScreenCell *vterm_cell_new(const VTerm *vt)
{
    return vterm_allocator_malloc(vt, sizeof(VTermScreenCell));
}

void vterm_cell_free(const VTerm *vt, VTermScreenCell *cell)
{
  vterm_allocator_free(vt, cell);
}

/*void vterm_cell_get_chars(const VTermScreenCell *cell) {};*/

char vterm_cell_get_width(const VTermScreenCell *cell)
{
  return cell->width;
};

char vterm_cell_set_width(VTermScreenCell *cell, char width)
{
  cell->width = width;
};

unsigned int vterm_cell_get_bold(VTermScreenCell *cell)
{
  return cell->attrs.bold;
};
/*void vterm_cell_get_underline(VTermScreenCell *cell) {};*/
/*void vterm_cell_get_italic(VTermScreenCell *cell) {};*/
/*void vterm_cell_get_blink(VTermScreenCell *cell) {};*/
/*void vterm_cell_get_reverse(VTermScreenCell *cell) {};*/
/*void vterm_cell_get_strike(VTermScreenCell *cell) {};*/
/*void vterm_cell_get_font(VTermScreenCell *cell) {};*/
/*void vterm_cell_get_dwl(VTermScreenCell *cell) {};*/
/*void vterm_cell_get_dhl(VTermScreenCell *cell) {};*/
/*void vterm_cell_get_fg(VTermScreenCell *cell) {};*/
/*void vterm_cell_get_bg(VTermScreenCell *cell) {};*/
