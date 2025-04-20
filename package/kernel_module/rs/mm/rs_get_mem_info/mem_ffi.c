#include <linux/version.h>
#include <linux/module.h>
#include <linux/init.h>
#include <linux/mm.h>
#include <linux/types.h>

struct page *c_pfn_to_page(u64 pfn)
{
    return pfn_to_page(pfn);
}

//u64 C_ARCH_PFN_OFFSET = ARCH_PFN_OFFSET;
u64 c_get_pfn_offset(void)
{
    //return ARCH_PFN_OFFSET;
    return 0;
}

unsigned long c_get_num_physpages(void)
{
    return get_num_physpages();
}

bool c_pfn_valid(u64 pfn)
{
    return pfn_valid(pfn);
}

u64 c_page_count(u64 pfn)
{
    return page_count(pfn_to_page(pfn));
}

// u64 c_page_count(u8 *page)
// {
//     return page_count((struct page *)page);
// }

bool c_page_locked(u64 pfn)
{
    return PageLocked(pfn_to_page(pfn));
}

bool c_page_reserved(u64 pfn)
{
    return PageReserved(pfn_to_page(pfn));
}

bool c_page_swapcache(u64 pfn)
{
    struct folio *folio = page_folio(pfn_to_page(pfn));
    return folio_test_swapcache(folio);
}

bool c_page_referenced(u64 pfn)
{
    struct folio *folio = page_folio(pfn_to_page(pfn));
    return folio_test_referenced(folio);
}

bool c_page_slab(u64 pfn)
{
    struct folio *folio = page_folio(pfn_to_page(pfn));
    return folio_test_slab(folio);
}

bool c_page_private(u64 pfn)
{
    return PagePrivate(pfn_to_page(pfn));
}

bool c_page_uptodate(u64 pfn)
{
    struct folio *folio = page_folio(pfn_to_page(pfn));
    return folio_test_uptodate(folio);
}

bool c_page_dirty(u64 pfn)
{
    return PageDirty(pfn_to_page(pfn));
}

bool c_page_active(u64 pfn)
{
    struct folio *folio = page_folio(pfn_to_page(pfn));
    return folio_test_active(folio);
}

bool c_page_writeback(u64 pfn)
{
    struct folio *folio = page_folio(pfn_to_page(pfn));
    return folio_test_writeback(folio);
}

bool c_page_mappedtodisk(u64 pfn)
{
    struct folio *folio = page_folio(pfn_to_page(pfn));
    return folio_test_mappedtodisk(folio);
}
