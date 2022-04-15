use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

/*
    CR3にLevel4テーブルの物理アドレスが入っている
    仮想アドレスは
    Lv4 lv3 Lv2 Lv1 Offsetで構成されている。
*/

// level4 tableを得る。物理メモリへのオフセットを与える。
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;
    let (level_4_page_frame, _) = Cr3::read();
    let phys = level_4_page_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *page_table_ptr
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    let (level_4_page_frame, _) = Cr3::read();
    let table_indexes = [addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()];
    crate::println!("  Indexes: {:?}", table_indexes);
    let mut frame = level_4_page_frame;
    for &index in &table_indexes {
        crate::print!(" - {:?}", frame.start_address());
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        }
    }
    crate::println!();

    Some(frame.start_address() + u64::from(addr.page_offset()))
}
