GET_MEM_INFO_VERSION = 1.0
GET_MEM_INFO_SITE = $(BR2_EXTERNAL_RUST_PATH)/package/kernel_module/c/mm/get_mem_info
#GET_SYSTEM_MEM_INFO_SITE = $(PWD)
# 如果你的源代码放在外部树的 package 目录内，可以使用 local 方法。
GET_MEM_INFO_SITE_METHOD = local
# 如果在 git 上，可以设置为 git 地址，并设置 VDEV_SITE_METHOD = git

GET_MEM_INFO_LICENSE = GPL-2.0
GET_MEM_INFO_LICENSE_FILES = COPYING
GET_MEM_INFO_DEPENDENCIES = linux

$(eval $(kernel-module))
$(eval $(generic-package))
