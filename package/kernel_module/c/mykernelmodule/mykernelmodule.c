#include <linux/init.h>
#include <linux/module.h>

MODULE_LICENSE("GPL");

static int __init mykernelmodule_init(void)
{
    printk(KERN_INFO "%s: module init\n", __func__);
    return 0;
}

static void __exit mykernelmodule_exit(void)
{
    printk(KERN_INFO "%s: module exit\n", __func__);
}

module_init(mykernelmodule_init);
module_exit(mykernelmodule_exit);
