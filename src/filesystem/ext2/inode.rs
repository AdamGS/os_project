#[repr(C)]
struct Inode {
    type_and_permissions: u16,
    used_id: u16,
    lower_bits_of_size: u32,
    last_access_time: u32,
    creation_time: u32,
    last_modification_time: u32,
    deletion_time: u32,
    group_id: u16,
    hard_links_to_inode: u16,
    disk_sector_count: u32,
    flags: u32,
    os_specific_value: u32
}
