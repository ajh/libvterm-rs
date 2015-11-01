#include "libvterm/src/vterm_internal.h"

#include <stdlib.h>
#include <string.h>

// Work around rust ffi compatibility issues with structs of function pointers
//
// typedef struct {
//   int (*text)(const char *bytes, size_t len, void *user);
//   int (*control)(unsigned char control, void *user);
//   int (*escape)(const char *bytes, size_t len, void *user);
//   int (*csi)(const char *leader, const long args[], int argcount, const char *intermed, char command, void *user);
//   int (*osc)(const char *command, size_t cmdlen, void *user);
//   int (*dcs)(const char *command, size_t cmdlen, void *user);
//   int (*resize)(int rows, int cols, void *user);
// } VTermParserCallbacks;
//
// typedef struct {
//   int (*putglyph)(VTermGlyphInfo *info, VTermPos pos, void *user);
//   int (*movecursor)(VTermPos pos, VTermPos oldpos, int visible, void *user);
//   int (*scrollrect)(VTermRect rect, int downward, int rightward, void *user);
//   int (*moverect)(VTermRect dest, VTermRect src, void *user);
//   int (*erase)(VTermRect rect, int selective, void *user);
//   int (*initpen)(void *user);
//   int (*setpenattr)(VTermAttr attr, VTermValue *val, void *user);
//   int (*settermprop)(VTermProp prop, VTermValue *val, void *user);
//   int (*bell)(void *user);
//   int (*resize)(int rows, int cols, VTermPos *delta, void *user);
//   int (*setlineinfo)(int row, const VTermLineInfo *newinfo, const VTermLineInfo *oldinfo, void *user);
// } VTermStateCallbacks;
//
// typedef struct {
//   int (*damage)(VTermRect rect, void *user);
//   int (*moverect)(VTermRect dest, VTermRect src, void *user);
//   int (*movecursor)(VTermPos pos, VTermPos oldpos, int visible, void *user);
//   int (*settermprop)(VTermProp prop, VTermValue *val, void *user);
//   int (*bell)(void *user);
//   int (*resize)(int rows, int cols, void *user);
//   int (*sb_pushline)(int cols, const VTermScreenCell *cells, void *user);
//   int (*sb_popline)(int cols, VTermScreenCell *cells, void *user);
// } VTermScreenCallbacks;

VTermScreenCallbacks *vterm_screen_callbacks_new()
{
  return VTermScreenCallbacks {};
}

void vterm_screen_callbacks_set_sb_pushline(VTermScreenCallbacks *self, int (*cb)(int cols, const VTermScreenCell *cells, void *user)) {
  self->sb_pushline = cb;
}
