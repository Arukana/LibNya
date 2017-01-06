#include "neko.h"
#include <stdio.h>

void start(t_lbstat *state, void ** _) {
	state->sheet = Bust;
	state->emotion[0][0].part = Heart;
	state->emotion[0][0].emotion = Shocked;
	state->position.cardinal = MiddleCentral;
}
