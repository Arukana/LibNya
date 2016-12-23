#include "Adder_stub.h"
#include <stdio.h>

void HsStart(void);
void HsEnd(void);

void start(void) {
	  HsStart();
	  printf("12 + 5 = %i\n", adder(12,5));
}

void end(void) {
	HsEnd();
}
