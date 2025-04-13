VDEV_VERSION = 1.0
VDEV_SITE = $(BR2_EXTERNAL_RUST_PATH)/package/kernel_module/rs/vdev
# 如果你的源代码放在外部树的 package 目录内，可以使用 local 方法。
VDEV_SITE_METHOD = local
# 如果在 git 上，可以设置为 git 地址，并设置 VDEV_SITE_METHOD = git

VDEV_LICENSE = GPL-2.0
VDEV_LICENSE_FILES = COPYING
VDEV_DEPENDENCIES = linux

$(eval $(kernel-module))
$(eval $(generic-package))
