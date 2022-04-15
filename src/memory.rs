use bootloader::bootinfo::MemoryMap;
use bootloader::bootinfo::MemoryRegionType;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PhysFrame, Size4KiB};
use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

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

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator { memory_map, next: 0 }
    }
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // メモリマップからusableな領域を得る
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // それぞれの領域をアドレス範囲にmapで変換する
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // フレームの開始アドレスのイテレータへと変換する
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // 開始アドレスから`PhysFrame`型を作る
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        crate::println!("Allocate Frame: {:?}", frame);
        self.next += 1;
        frame
    }
}
