#obj-m += $(addsuffix .o, get_system_mem_info)
#obj-m += $(addsuffix .o, mem_info)
#ccflags-y := -DDEBUG -g

obj-m += $(addsuffix .o, get_mm_info)

KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) -C $(KDIR) M=$$PWD

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD modules_install
