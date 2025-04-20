#ccflags-y := -DDEBUG -g
obj-m += $(addsuffix .o, rs_alloc_mem)
rs_alloc_mem-y := alloc_mem.o mem_ffi.o

KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) -C $(KDIR) M=$$PWD

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD modules_install
