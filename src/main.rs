use libzfs_rs::zfs::{LibZfs};

fn main() {
    println!("Hello, world!");

    let mut libzfs_handle = LibZfs::new().unwrap();
//    let pool = libzfs_handle.pool_by_name("test");
//    let pool = pool.unwrap();
//    let state = pool.state();
//    dbg!(pool);

    let result = libzfs_handle.create_filesystem("test/abcd").unwrap();
}
