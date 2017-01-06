#include "neko.h"
#include <Python.h>
#include <stdio.h>

struct API {
	void (*start)(t_lbstat *x, void **y);
	void (*end)(t_lbstat *x, void **y);
	char* (*dump_buffer)(char *buffer, int buffer_size);
	int (*release_object)(char *obj);
};

void start(t_lbstat *lib, void **data);
void idle(t_lbstat *lib, void **data);

#define PYVERIFY(exp) if ((exp) == 0) { fprintf(stderr, "%s[%d]: ", __FILE__, __LINE__); PyErr_Print(); exit(1); }

struct API the_ffi;

void start(t_lbstat *lib, void **data)
{ PyObject *pName, *pModule, *py_results;
  PyObject *fill_api;
  Py_Initialize();
  PyRun_SimpleString
  ( "import sys;"
    "sys.path.insert(0, '.')" );
  PYVERIFY(pName = PyString_FromString("lib"))
  PYVERIFY(pModule = PyImport_Import(pName))
  Py_DECREF(pName);
  PYVERIFY(fill_api = PyObject_GetAttrString(pModule, "fill_api"))
  py_results = PyObject_CallFunction(fill_api, "k", &the_ffi);
  printf("  C_LIB::%lu\n", lib);
  the_ffi.start(lib, (void**)data); }

void end(t_lbstat *lib, void **data)
{ PyObject *pName, *pModule, *py_results;
  PyObject *fill_api;
  PYVERIFY(pName = PyString_FromString("lib"))
  PYVERIFY(pModule = PyImport_Import(pName))
  Py_DECREF(pName);
  PYVERIFY(fill_api = PyObject_GetAttrString(pModule, "fill_api"))
  py_results = PyObject_CallFunction(fill_api, "k", &the_ffi);
  the_ffi.end(lib, (void**)data);
  Py_Finalize(); }
