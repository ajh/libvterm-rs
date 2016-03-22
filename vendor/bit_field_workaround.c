#include "libvterm/src/vterm_internal.h"

#include <stdlib.h>
#include <string.h>

// This file contains c code to workaround rust ffi compatibility issues with c
// bit fields.

// ------------
// Screen cell stuff
// ------------

VTermScreenCell *vterm_cell_new()
{
  size_t size = sizeof(VTermScreenCell);
  void *ptr = malloc(size);
  if(ptr)
    memset(ptr, 0, size);
  return ptr;
}

void vterm_cell_free(VTermScreenCell *cell)
{
  free(cell);
}

int vterm_cell_get_chars(const VTermScreenCell *cell, uint32_t *chars, size_t len) {
  if (len < VTERM_MAX_CHARS_PER_CELL) {
    return -1;
  }

  int i;
  for(i = 0; i < VTERM_MAX_CHARS_PER_CELL && cell->chars[i]; i++) {
    chars[i] = cell->chars[i];
  }
  return i;
}

void vterm_cell_set_chars(VTermScreenCell *cell, const uint32_t *chars, size_t len) {
  int i;
  for(i = 0; i < len && i < VTERM_MAX_CHARS_PER_CELL; i++) {
    cell->chars[i] = chars[i];
  }
}

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

// Need this since rust doesn't know the size of a VTermScreenCell
const VTermScreenCell *vterm_cell_pointer_arithmetic(VTermScreenCell *const cell, int amount)
{
  return cell + amount;
}

// ------------
// Glyph Info stuff
// ------------

int vterm_glyph_info_get_chars(const VTermGlyphInfo *glyph_info, uint32_t *chars, size_t len) {
  if (len < VTERM_MAX_CHARS_PER_CELL) {
    return -1;
  }

  int i;
  for(i = 0; i < VTERM_MAX_CHARS_PER_CELL && glyph_info->chars[i]; i++) {
    chars[i] = glyph_info->chars[i];
  }
  return i;
}

int vterm_glyph_info_width(const VTermGlyphInfo *glyph_info) {
  return glyph_info->width;
}

unsigned int vterm_glyph_info_protected_cell(const VTermGlyphInfo *glyph_info) {
  return glyph_info->protected_cell;
}

int vterm_glyph_info_dwl(const VTermGlyphInfo *glyph_info) {
  return glyph_info->dwl;
}

int vterm_glyph_info_dhl(const VTermGlyphInfo *glyph_info) {
  return glyph_info->dhl;
}
