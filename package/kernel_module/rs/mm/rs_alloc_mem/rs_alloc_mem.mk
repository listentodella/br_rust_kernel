# 永远要记住，重要的不光是site后的路径名称
# 这些变量名也非常重要
RS_ALLOC_MEM_VERSION = 1.0
RS_ALLOC_MEM_SITE = $(BR2_EXTERNAL_RUST_PATH)/package/kernel_module/rs/mm/rs_alloc_mem
#GET_SYSTEM_MEM_INFO_SITE = $(PWD)
# 如果你的源代码放在外部树的 package 目录内，可以使用 local 方法。
RS_ALLOC_MEM_SITE_METHOD = local
# 如果在 git 上，可以设置为 git 地址，并设置 VDEV_SITE_METHOD = git

RS_ALLOC_MEM_LICENSE = GPL-2.0
RS_ALLOC_MEM_LICENSE_FILES = COPYING
RS_ALLOC_MEM_DEPENDENCIES = linux

$(eval $(kernel-module))
$(eval $(generic-package))
