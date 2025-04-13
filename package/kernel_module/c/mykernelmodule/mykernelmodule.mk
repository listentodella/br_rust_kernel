MYKERNELMODULE_VERSION = 1.0
MYKERNELMODULE_SITE = $(BR2_EXTERNAL_RUST_PATH)/package/kernel_module/c/mykernelmodule
# 如果你的源代码放在外部树的 package 目录内，可以使用 local 方法。
MYKERNELMODULE_SITE_METHOD = local
# 如果在 git 上，可以设置为 git 地址，并设置 MY_KERNEL_MODULE_SITE_METHOD = git

MYKERNELMODULE_LICENSE = GPL-2.0
MYKERNELMODULE_LICENSE_FILES = COPYING
#MY_KERNEL_MODULE_DEPENDENCIES = linux

# 设定模块源代码所在的目录（相对于包目录）
# MY_KERNEL_MODULE_SOURCE_DIR = .

# 启用内核模块包的构建规则（Buildroot 提供了一套通用规则），
# kernel-module.mk 定义了一系列变量和通用规则。
$(eval $(kernel-module))
$(eval $(generic-package))
