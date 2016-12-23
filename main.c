#include <dlfcn.h>

int main(void) {
	void* bundle = dlopen("libss.so", RTLD_LAZY);

	void (*start)(void);
	void (*end)(void);

	start = dlsym(bundle, "start");
	end = dlsym(bundle, "end");

	(*start)();
	(*end)();
	dlclose(bundle);
}
