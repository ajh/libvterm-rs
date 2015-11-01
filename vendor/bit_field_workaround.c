#include "libvterm/src/vterm_internal.h"

// Work around rust ffi compatibility issues with c bit fields
//
// typedef struct {
// #define VTERM_MAX_CHARS_PER_CELL 6
//   uint32_t chars[VTERM_MAX_CHARS_PER_CELL];
//   char     width;
//   struct {
//     unsigned int bold      : 1;
//     unsigned int underline : 2;
//     unsigned int italic    : 1;
//     unsigned int blink     : 1;
//     unsigned int reverse   : 1;
//     unsigned int strike    : 1;
//     unsigned int font      : 4; /* 0 to 9 */
//     unsigned int dwl       : 1; /* On a DECDWL or DECDHL line */
//     unsigned int dhl       : 2; /* On a DECDHL line (1=top 2=bottom) */
//   } attrs;
//   VTermColor fg, bg;
// } VTermScreenCell;

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

void vterm_cell_set_bold(VTermScreenCell *cell, unsigned int is_bold)
{
  cell->attrs.bold = is_bold;
};

unsigned int vterm_cell_get_underline(VTermScreenCell *cell)
{
  return cell->attrs.underline;
};

void vterm_cell_set_underline(VTermScreenCell *cell, unsigned int underline_value)
{
  cell->attrs.underline = underline_value;
};

unsigned int vterm_cell_get_italic(VTermScreenCell *cell)
{
  return cell->attrs.italic;
};

void vterm_cell_set_italic(VTermScreenCell *cell, unsigned int is_italic)
{
  cell->attrs.italic = is_italic;
};

unsigned int vterm_cell_get_blink(VTermScreenCell *cell)
{
  return cell->attrs.blink;
};

void vterm_cell_set_blink(VTermScreenCell *cell, unsigned int is_blink)
{
  cell->attrs.blink = is_blink;
};

unsigned int vterm_cell_get_reverse(VTermScreenCell *cell)
{
  return cell->attrs.reverse;
};

void vterm_cell_set_reverse(VTermScreenCell *cell, unsigned int is_reverse)
{
  cell->attrs.reverse = is_reverse;
};

unsigned int vterm_cell_get_strike(VTermScreenCell *cell)
{
  return cell->attrs.strike;
};

void vterm_cell_set_strike(VTermScreenCell *cell, unsigned int is_strike)
{
  cell->attrs.strike = is_strike;
};

unsigned int vterm_cell_get_font(VTermScreenCell *cell)
{
  return cell->attrs.font;
};

void vterm_cell_set_font(VTermScreenCell *cell, unsigned int font_value)
{
  cell->attrs.font = font_value;
};

unsigned int vterm_cell_get_dwl(VTermScreenCell *cell)
{
  return cell->attrs.dwl;
};

void vterm_cell_set_dwl(VTermScreenCell *cell, unsigned int dwl)
{
  cell->attrs.dwl = dwl;
};

unsigned int vterm_cell_get_dhl(VTermScreenCell *cell)
{
  return cell->attrs.dhl;
};

void vterm_cell_set_dhl(VTermScreenCell *cell, unsigned int dhl)
{
  cell->attrs.dhl = dhl;
};

VTermColor vterm_cell_get_fg(VTermScreenCell *cell)
{
  return cell->fg;
};

void vterm_cell_set_fg(VTermScreenCell *cell, VTermColor color)
{
  cell->fg = color;
};

VTermColor vterm_cell_get_bg(VTermScreenCell *cell)
{
  return cell->bg;
};

void vterm_cell_set_bg(VTermScreenCell *cell, VTermColor color)
{
  cell->bg = color;
};
