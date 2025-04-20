MM_VERSION = 1.0
MM_SITE = $(BR2_EXTERNAL_RUST_PATH)/package/kernel_module/c/mm
# 如果你的源代码放在外部树的 package 目录内，可以使用 local 方法。
MM_SITE_METHOD = local
# 如果在 git 上，可以设置为 git 地址，并设置 VDEV_SITE_METHOD = git

include $(sort $(MM_SITE)/*/*.mk)
# MM 只是起到一个总开关的作用, 本身并没有任何代码，就不需要下面的代码了
#MM_LICENSE = GPL-2.0
#MM_LICENSE_FILES = COPYING
#MM_DEPENDENCIES = linux

#include $(sort $(wildcard package/mm/*/*.mk))
#include $(BR2_EXTERNAL_RUST_PATH)/package/kernel_module/rs/mm/get_system_mem_info/get_system_mem_info.mk

#$(eval $(kernel-module))
#$(eval $(generic-package))
