obj-m += $(addsuffix .o, vdev)
#ccflags-y := -DDEBUG -g

KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) -C $(KDIR) M=$$PWD

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD modules_install
