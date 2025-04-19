obj-m += $(addsuffix .o, get_system_mem_info)
#ccflags-y := -DDEBUG -g

KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) -C $(KDIR) M=$$PWD

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD modules_install
