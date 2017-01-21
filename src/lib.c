#include <unistd.h>

#include "neko.h"

void messagecpy(t_character *start, const unsigned char *source) {
	t_character *end = start + SPEC_CHARACTER_MAX;

	while (32 <= *source && *source < 127 && start < end) {
		start->glyph = (unsigned int)*source++;
		start++;
	}
	while (start < end) {
		start->glyph = (unsigned int)'\0';
		start++;
	}
}

void start(t_lbstat *lib, void ** _) {
	lib->persona.sheet = Bust;
	(*lib).tooltip.relative = Left;
	messagecpy(lib->tooltip.message, (const unsigned char *)"start\0");
}

void key_unicode_down (t_lbstat *lib, void **data, unsigned long long key) {
	messagecpy(lib->tooltip.message, (const unsigned char []){(const unsigned char)key, (const unsigned char)'\0'});
}

void key_string_down (t_lbstat *lib, void **data, unsigned char *copy) {
	messagecpy(lib->tooltip.message, (const unsigned char *)copy);
}

void mouse_pressed (t_lbstat *lib, void **data, t_mouse code, unsigned short cartesian[2]) {
	messagecpy(lib->tooltip.message, (const unsigned char []){(const unsigned char)code+' ', (const unsigned char)'\0'});
}

void mouse_released (t_lbstat *lib, void **data, t_mouse code, unsigned short cartesian[2]) {
	messagecpy(lib->tooltip.message, (const unsigned char []){(const unsigned char)code+'!', (const unsigned char)'\0'});
}

void resized (t_lbstat *lib, void **data, t_winszed *win) {
	messagecpy(lib->tooltip.message, (const unsigned char *)"resized\0");
}

void process (t_lbstat *lib, void **data, char *name, pid_t pid) {
	messagecpy(lib->tooltip.message, (const unsigned char *)name);
}
