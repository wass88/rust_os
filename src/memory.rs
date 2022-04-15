use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

use x86_64::structures::paging::OffsetPageTable;

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/*
    CR3にLevel4テーブルの物理アドレスが入っている
    仮想アドレスは
    Lv4 lv3 Lv2 Lv1 Offsetで構成されている。
*/

// level4 tableを得る。物理メモリへのオフセットを与える。
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;
    let (level_4_page_frame, _) = Cr3::read();
    let phys = level_4_page_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *page_table_ptr
}
