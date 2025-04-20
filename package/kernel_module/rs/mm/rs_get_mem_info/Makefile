#ccflags-y := -DDEBUG -g
obj-m += $(addsuffix .o, rs_get_mem_info)
rs_get_mem_info-y := get_system_mem_info.o mem_ffi.o

KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) -C $(KDIR) M=$$PWD

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD modules_install
