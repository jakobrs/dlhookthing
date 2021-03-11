#include <iostream>

extern "C" void *dlsym(void *, const char *);

const char *render(const char *file) {
  return file == nullptr ? "NULL" : file;
}

extern "C" void prehook(const char *file, int mode) {
  std::cout << "Attempting to open " << render(file) << " with mode " << mode << std::endl;
}
extern "C" void posthook(const char *file, int mode, void *rax) {
  file = render(file);

  if (rax == nullptr) {
    std::cout << "Failed to open " << file << " with mode " << mode << std::endl;
  } else {
    std::cout << "Successfully opened " << file << " with mode " << mode << ", returning " << rax << std::endl;
  }
}

using Dlopentype = void *(*)(const char *, int);

extern "C" void *dlopen(const char *file, int mode) {
  void *rax;
  asm("mov %%rax, %0":"=r" (rax));

  static int PHASE = 0;
  static Dlopentype ORIGDLOPEN = (Dlopentype)nullptr;
  static const char *OLDFILE;
  static int OLDMODE;

  switch (PHASE) {
    case 0:
      if (ORIGDLOPEN == nullptr) {
        ORIGDLOPEN = (Dlopentype)dlsym((void *)-1, "dlopen");
      }

      PHASE = 1;

      const void **aora;
      asm volatile("leaq 8(%%rbp), %0":"=r" (aora));
      *aora -= 5;

      prehook(file, mode);

      OLDFILE = file;
      OLDMODE = mode;

      return ORIGDLOPEN(file, mode);
    default:
      PHASE = 0;

      posthook(OLDFILE, OLDMODE, rax);
      
      return rax;
  }
}
