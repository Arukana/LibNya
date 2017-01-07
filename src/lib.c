#include "neko.h"
#include <Python.h>
#include <stdio.h>

struct API {
	void (*start)(t_lbstat *x, void **y);
	void (*end)(t_lbstat *x, void **y);
	char *(*dump_buffer)(char *buffer, int buffer_size);
	int (*release_object)(char *obj);
};

void start(t_lbstat *lib, void **data);
void idle(t_lbstat *lib, void **data);

struct API the_ffi;

void start(t_lbstat *lib, void **data) {
	PyObject *pName = NULL, *pModule = NULL, *py_results = NULL;
	PyObject *fill_api = NULL;
	Py_Initialize();
	PyRun_SimpleString("import sys;"
					 "sys.path.insert(0, '.')");
	pName = PyString_FromString("lib");
	pModule = PyImport_Import(pName);
	Py_DECREF(pName);
	fill_api = PyObject_GetAttrString(pModule, "fill_api");
	py_results = PyObject_CallFunction(fill_api, "k", &the_ffi);
	the_ffi.start(lib, (void **)data);
}

void end(t_lbstat *lib, void **data) {
	PyObject *pName = NULL, *pModule = NULL, *py_results = NULL;
	PyObject *fill_api = NULL;
	pName = PyString_FromString("lib");
	pModule = PyImport_Import(pName);
	Py_DECREF(pName);
	fill_api = PyObject_GetAttrString(pModule, "fill_api");
	py_results = PyObject_CallFunction(fill_api, "k", &the_ffi);
	the_ffi.end(lib, (void **)data);
	Py_Finalize();
}
