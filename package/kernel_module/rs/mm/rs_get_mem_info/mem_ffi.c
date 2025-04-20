#include <linux/version.h>
#include <linux/module.h>
#include <linux/init.h>
#include <linux/mm.h>

unsigned long c_get_num_physpages(void)
{
    return get_num_physpages();
}
