#include <stdlib.h>
#include "HsFFI.h"

void HsStart(void) {
  hs_init(NULL, NULL);
}

void HsEnd(void) {
  hs_exit();
}
