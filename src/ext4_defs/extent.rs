use super::Ext4Inode;
use crate::constants::*;
use core::mem::size_of;

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Ext4ExtentHeader {
    /// Magic number, 0xF30A.
    pub magic: u16,

    /// Number of valid entries following the header.
    pub entries_count: u16,

    /// Maximum number of entries that could follow the header.
    pub max_entries_count: u16,

    /// Depth of this extent node in the extent tree.
    /// 0 = this extent node points to data blocks;
    /// otherwise, this extent node points to other extent nodes.
    /// The extent tree can be at most 5 levels deep:
    /// a logical block number can be at most 2^32,
    /// and the smallest n that satisfies 4*(((blocksize - 12)/12)^n) >= 2^32 is 5.
    pub depth: u16,

    /// Generation of the tree. (Used by Lustre, but not standard ext4).
    pub generation: u32,
}

impl<T> TryFrom<&[T]> for Ext4ExtentHeader {
    type Error = u64;
    fn try_from(data: &[T]) -> core::result::Result<Self, u64> {
        let data = data;
        Ok(unsafe { core::ptr::read(data.as_ptr() as *const _) })
    }
}

impl Ext4ExtentHeader {
    // 获取extent header的魔数
    pub fn get_magic(&self) -> u16 {
        self.magic
    }

    // 设置extent header的魔数
    pub fn set_magic(&mut self, magic: u16) {
        self.magic = magic;
    }

    // 获取extent header的条目数
    pub fn get_entries_count(&self) -> u16 {
        self.entries_count
    }

    // 设置extent header的条目数
    pub fn set_entries_count(&mut self, count: u16) {
        self.entries_count = count;
    }

    // 获取extent header的最大条目数
    pub fn get_max_entries_count(&self) -> u16 {
        self.max_entries_count
    }

    // 设置extent header的最大条目数
    pub fn set_max_entries_count(&mut self, max_count: u16) {
        self.max_entries_count = max_count;
    }

    // 获取extent header的深度
    pub fn get_depth(&self) -> u16 {
        self.depth
    }

    // 设置extent header的深度
    pub fn set_depth(&mut self, depth: u16) {
        self.depth = depth;
    }

    // 获取extent header的生成号
    pub fn get_generation(&self) -> u32 {
        self.generation
    }

    // 设置extent header的生成号
    pub fn set_generation(&mut self, generation: u32) {
        self.generation = generation;
    }

    pub fn ext4_extent_header_depth(&self) -> u16 {
        self.depth
    }

    pub fn ext4_extent_header_set_depth(&mut self, depth: u16) {
        self.depth = depth;
    }
    pub fn ext4_extent_header_set_entries_count(&mut self, entries_count: u16) {
        self.entries_count = entries_count;
    }
    pub fn ext4_extent_header_set_generation(&mut self, generation: u32) {
        self.generation = generation;
    }
    pub fn ext4_extent_header_set_magic(&mut self) {
        self.magic = EXT4_EXTENT_MAGIC;
    }

    pub fn ext4_extent_header_set_max_entries_count(&mut self, max_entries_count: u16) {
        self.max_entries_count = max_entries_count;
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Ext4ExtentIndex {
    /// This index node covers file blocks from ‘block’ onward.
    pub first_block: u32,

    /// Lower 32-bits of the block number of the extent node that is
    /// the next level lower in the tree. The tree node pointed to
    /// can be either another internal node or a leaf node, described below.
    pub leaf_lo: u32,

    /// Upper 16-bits of the previous field.
    pub leaf_hi: u16,

    pub padding: u16,
}

impl<T> TryFrom<&[T]> for Ext4ExtentIndex {
    type Error = u64;
    fn try_from(data: &[T]) -> core::result::Result<Self, u64> {
        let data = &data[..size_of::<Ext4ExtentIndex>()];
        Ok(unsafe { core::ptr::read(data.as_ptr() as *const _) })
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Ext4Extent {
    /// First file block number that this extent covers.
    pub first_block: u32,

    /// Number of blocks covered by extent.
    /// If the value of this field is <= 32768, the extent is initialized.
    /// If the value of the field is > 32768, the extent is uninitialized
    /// and the actual extent length is ee_len - 32768.
    /// Therefore, the maximum length of a initialized extent is 32768 blocks,
    /// and the maximum length of an uninitialized extent is 32767.
    pub block_count: u16,

    /// Upper 16-bits of the block number to which this extent points.
    pub start_hi: u16,

    /// Lower 32-bits of the block number to which this extent points.
    pub start_lo: u32,
}

impl<T> TryFrom<&[T]> for Ext4Extent {
    type Error = u64;
    fn try_from(data: &[T]) -> core::result::Result<Self, u64> {
        let data = &data[..size_of::<Ext4Extent>()];
        Ok(unsafe { core::ptr::read(data.as_ptr() as *const _) })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ext4ExtentPath {
    // Physical block number
    pub p_block: u64,
    // Single block descriptor
    // pub block: Ext4Block,
    // Depth of this extent node
    pub depth: u16,
    // Max depth of the extent tree
    pub maxdepth: u16,
    // Pointer to the extent header
    pub header: *mut Ext4ExtentHeader,
    // Pointer to the index in the current node
    pub index: *mut Ext4ExtentIndex,
    // Pointer to the extent in the current node
    pub extent: *mut Ext4Extent,
}

impl Default for Ext4ExtentPath {
    fn default() -> Self {
        Self {
            p_block: 0,
            // block: Ext4Block::default(),
            depth: 0,
            maxdepth: 0,
            header: core::ptr::null_mut(),
            index: core::ptr::null_mut(),
            extent: core::ptr::null_mut(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ext4ExtentPathOld {
    // Physical block number
    pub p_block: u32,
    // Single block descriptor
    // pub block: Ext4Block,
    // Depth of this extent node
    pub depth: u16,
    // Max depth of the extent tree
    pub maxdepth: u16,
    // Pointer to the extent header
    pub header: *const Ext4ExtentHeader,
    // Pointer to the index in the current node
    pub index: *const Ext4ExtentIndex,
    // Pointer to the extent in the current node
    pub extent: *const Ext4Extent,
}

impl Default for Ext4ExtentPathOld {
    fn default() -> Self {
        Self {
            p_block: 0,
            // block: Ext4Block::default(),
            depth: 0,
            maxdepth: 0,
            header: core::ptr::null_mut(),
            index: core::ptr::null_mut(),
            extent: core::ptr::null_mut(),
        }
    }
}

#[allow(unused)]
pub fn ext4_first_extent(hdr: *const Ext4ExtentHeader) -> *const Ext4Extent {
    unsafe {
        let offset = core::mem::size_of::<Ext4ExtentHeader>();

        (hdr as *const u8).add(offset) as *const Ext4Extent
    }
}

pub fn ext4_first_extent_mut(hdr: *mut Ext4ExtentHeader) -> *mut Ext4Extent {
    unsafe {
        let offset = core::mem::size_of::<Ext4ExtentHeader>();

        (hdr as *mut u8).add(offset) as *mut Ext4Extent
    }
}

#[allow(unused)]
pub fn ext4_last_extent(hdr: *const Ext4ExtentHeader) -> *const Ext4Extent {
    unsafe {
        let hdr_size = core::mem::size_of::<Ext4ExtentHeader>();
        let ext_size = core::mem::size_of::<Ext4Extent>();
        let hdr_ref = core::mem::transmute::<*const Ext4ExtentHeader, &Ext4ExtentHeader>(hdr);
        let ext_count = hdr_ref.entries_count as usize;
        (hdr as *const u8).add(hdr_size + (ext_count - 1) * ext_size) as *const Ext4Extent
    }
}

pub fn ext4_last_extent_mut(hdr: *mut Ext4ExtentHeader) -> *mut Ext4Extent {
    unsafe {
        let hdr_size = core::mem::size_of::<Ext4ExtentHeader>();
        let ext_size = core::mem::size_of::<Ext4Extent>();
        let hdr_ref = core::mem::transmute::<*mut Ext4ExtentHeader, &Ext4ExtentHeader>(hdr);
        let ext_count = hdr_ref.entries_count as usize;

        (hdr as *mut u8).add(hdr_size + (ext_count - 1) * ext_size) as *mut Ext4Extent
    }
}

#[allow(unused)]
pub fn ext4_first_extent_index(hdr: *const Ext4ExtentHeader) -> *const Ext4ExtentIndex {
    unsafe {
        let offset = core::mem::size_of::<Ext4ExtentHeader>();

        (hdr as *const u8).add(offset) as *const Ext4ExtentIndex
    }
}

pub fn ext4_first_extent_index_mut(hdr: *mut Ext4ExtentHeader) -> *mut Ext4ExtentIndex {
    unsafe {
        let offset = core::mem::size_of::<Ext4ExtentHeader>();

        (hdr as *mut u8).add(offset) as *mut Ext4ExtentIndex
    }
}

#[allow(unused)]
pub fn ext4_last_extent_index(hdr: *const Ext4ExtentHeader) -> *const Ext4ExtentIndex {
    unsafe {
        let hdr_size = core::mem::size_of::<Ext4ExtentHeader>();
        let ext_size = core::mem::size_of::<Ext4ExtentIndex>();
        let hdr_ref = core::mem::transmute::<*const Ext4ExtentHeader, &Ext4ExtentHeader>(hdr);
        let ext_count = hdr_ref.entries_count as usize;
        (hdr as *const u8).add(hdr_size + (ext_count - 1) * ext_size) as *const Ext4ExtentIndex
    }
}

pub fn ext4_last_extent_index_mut(hdr: *mut Ext4ExtentHeader) -> *mut Ext4ExtentIndex {
    unsafe {
        let hdr_size = core::mem::size_of::<Ext4ExtentHeader>();
        let ext_size = core::mem::size_of::<Ext4ExtentIndex>();
        let hdr_ref = core::mem::transmute::<*mut Ext4ExtentHeader, &Ext4ExtentHeader>(hdr);
        let ext_count = hdr_ref.entries_count as usize;
        (hdr as *mut u8).add(hdr_size + (ext_count - 1) * ext_size) as *mut Ext4ExtentIndex
    }
}

pub fn ext4_inode_hdr(inode: &Ext4Inode) -> *const Ext4ExtentHeader {
    let eh = &inode.block as *const [u32; 15] as *const Ext4ExtentHeader;
    eh
}

#[allow(unused)]
pub fn ext4_inode_hdr_mut(inode: &mut Ext4Inode) -> *mut Ext4ExtentHeader {
    let eh = &mut inode.block as *mut [u32; 15] as *mut Ext4ExtentHeader;
    eh
}

/// 定义ext4_ext_binsearch函数，接受一个指向ext4_extent_path的可变引用和一个逻辑块号
///
/// 返回一个布尔值，表示是否找到了对应的extent
pub fn ext4_ext_binsearch(path: &mut Ext4ExtentPath, block: u32) -> bool {
    // 获取extent header的引用
    // let eh = unsafe { &*path.header };
    let eh = path.header;

    unsafe {
        if (*eh).entries_count == 0 {
            return false;
        }
    }

    // 定义左右两个指针，分别指向第一个和最后一个extent
    let mut l = unsafe { ext4_first_extent_mut(eh).add(1) };
    let mut r = ext4_last_extent_mut(eh);

    // 如果extent header中没有有效的entry，直接返回false
    unsafe {
        if (*eh).entries_count == 0 {
            return false;
        }
    }
    // 使用while循环进行二分查找
    while l <= r {
        // 计算中间指针
        let m = unsafe { l.add((r as usize - l as usize) / 2) };
        // 获取中间指针所指向的extent的引用
        let ext = unsafe { &*m };
        // 比较逻辑块号和extent的第一个块号
        if block < ext.first_block {
            // 如果逻辑块号小于extent的第一个块号，说明目标在左半边，将右指针移动到中间指针的左边
            r = unsafe { m.sub(1) };
        } else {
            // 如果逻辑块号大于或等于extent的第一个块号，说明目标在右半边，将左指针移动到中间指针的右边
            l = unsafe { m.add(1) };
        }
    }
    // 循环结束后，将path的extent字段设置为左指针的前一个位置
    path.extent = unsafe { l.sub(1) };
    // 返回true，表示找到了对应的extent
    true
}

pub fn ext4_ext_binsearch_idx(path: &mut Ext4ExtentPath, block: ext4_lblk_t) -> bool {
    // 获取extent header的引用
    let eh = path.header;

    // 定义左右两个指针，分别指向第一个和最后一个extent
    let mut l = unsafe { ext4_first_extent_index_mut(eh).add(1) };
    let mut r = ext4_last_extent_index_mut(eh);

    // 如果extent header中没有有效的entry，直接返回false
    unsafe {
        if (*eh).entries_count == 0 {
            return false;
        }
    }
    // 使用while循环进行二分查找
    while l <= r {
        // 计算中间指针
        let m = unsafe { l.add((r as usize - l as usize) / 2) };
        // 获取中间指针所指向的extent的引用
        let ext = unsafe { &*m };
        // 比较逻辑块号和extent的第一个块号
        if block < ext.first_block {
            // 如果逻辑块号小于extent的第一个块号，说明目标在左半边，将右指针移动到中间指针的左边
            r = unsafe { m.sub(1) };
        } else {
            // 如果逻辑块号大于或等于extent的第一个块号，说明目标在右半边，将左指针移动到中间指针的右边
            l = unsafe { m.add(1) };
        }
    }
    // 循环结束后，将path的extent字段设置为左指针的前一个位置
    path.index = unsafe { l.sub(1) };
    // 返回true，表示找到了对应的extent
    true
}

#[allow(unused)]
pub fn ext4_ext_find_extent(eh: *mut Ext4ExtentHeader, block: ext4_lblk_t) -> *mut Ext4Extent {
    // 初始化一些变量
    let mut low: i32;
    let mut high: i32;
    let mut mid: i32;
    let mut ex: *mut Ext4Extent;

    // 如果头部的extent数为0，返回空指针
    if eh.is_null() || unsafe { (*eh).entries_count } == 0 {
        return core::ptr::null_mut();
    }

    // 从头部获取第一个extent的指针
    ex = ext4_first_extent_mut(eh);

    // 如果头部的深度不为0，返回空指针
    if unsafe { (*eh).depth } != 0 {
        return core::ptr::null_mut();
    }

    // 使用二分查找法在extent数组中查找逻辑块号
    low = 0;
    high = unsafe { (*eh).entries_count - 1 } as i32;
    while low <= high {
        // 计算中间位置
        mid = (low + high) / 2;

        // 获取中间位置的extent的指针
        ex = unsafe { ex.add(mid as usize) };

        // 比较extent的逻辑块号和目标逻辑块号
        if block >= unsafe { (*ex).first_block } {
            // 如果目标逻辑块号大于等于extent的逻辑块号，说明目标在右半部分
            low = mid + 1;
        } else {
            // 如果目标逻辑块号小于extent的逻辑块号，说明目标在左半部分
            high = mid - 1;
        }
    }

    // 如果没有找到目标，返回最后一个小于目标的extent的指针
    if high < 0 {
        return core::ptr::null_mut();
    } else {
        return unsafe { ex.add(high as usize) };
    }
}
