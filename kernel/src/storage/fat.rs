use crate::storage::block::read_sector;
use alloc::vec::Vec;
use crate::print;

extern crate alloc;

struct Bpb {
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    num_fats: u8,
    fat_size_32: u32,
    root_cluster: u32,
}

unsafe fn read_bpb() -> Result<Bpb, ()> {
    let mut buf = [0u8; 512];
    read_sector(0, &mut buf).map_err(|_| ())?;

    if buf[510] != 0x55 || buf[511] != 0xAA {
        return Err(());
    }

    Ok(Bpb {
        bytes_per_sector:    u16::from_le_bytes([buf[11], buf[12]]),
        sectors_per_cluster: buf[13],
        reserved_sectors:    u16::from_le_bytes([buf[14], buf[15]]),
        num_fats:            buf[16],
        fat_size_32:         u32::from_le_bytes([buf[36], buf[37], buf[38], buf[39]]),
        root_cluster:        u32::from_le_bytes([buf[44], buf[45], buf[46], buf[47]]),
    })
}

fn cluster_to_sector(bpb: &Bpb, cluster: u32) -> u64 {
    let data_start = bpb.reserved_sectors as u64
        + bpb.num_fats as u64 * bpb.fat_size_32 as u64;
    data_start + (cluster as u64 - 2) * bpb.sectors_per_cluster as u64
}

unsafe fn next_cluster(bpb: &Bpb, cluster: u32) -> Result<Option<u32>, ()> {
    let fat_start = bpb.reserved_sectors as u64;
    let byte_offset = cluster as u64 * 4;
    let sector = fat_start + byte_offset / 512;
    let offset = (byte_offset % 512) as usize;

    let mut buf = [0u8; 512];
    read_sector(sector, &mut buf).map_err(|_| ())?;

    let val = u32::from_le_bytes([
        buf[offset],
        buf[offset + 1],
        buf[offset + 2],
        buf[offset + 3],
    ]) & 0x0FFFFFFF;

    if val >= 0x0FFFFFF8 {
        Ok(None)
    } else {
        Ok(Some(val))
    }
}

unsafe fn read_chain(bpb: &Bpb, start_cluster: u32, size: u32) -> Result<Vec<u8>, ()> {
    let mut data: Vec<u8> = Vec::new();
    let mut cluster = start_cluster;

    loop {
        let sector = cluster_to_sector(bpb, cluster);
        for i in 0..bpb.sectors_per_cluster as u64 {
            let mut buf = [0u8; 512];
            read_sector(sector + i, &mut buf).map_err(|_| ())?;
            data.extend_from_slice(&buf);
        }

        match next_cluster(bpb, cluster)? {
            Some(next) => cluster = next,
            None => break,
        }
    }

    if size > 0 {
        data.truncate(size as usize);
    }

    Ok(data)
}

pub unsafe fn ls() {
    let bpb = match read_bpb() {
        Ok(b) => b,
        Err(_) => { print!("\nls: disk not formatted as FAT32\n"); return; }
    };

    let data = match read_chain(&bpb, bpb.root_cluster, 0) {
        Ok(d) => d,
        Err(_) => { print!("\nls: failed to read root dir\n"); return; }
    };

    print!("\n");
    for entry in data.chunks(32) {
        if entry[0] == 0x00 { break; }
        if entry[0] == 0xE5 { continue; }

        let attr = entry[11];
        if attr == 0x0F || attr == 0x08 { continue; }

        let name = &entry[0..8];
        let ext  = &entry[8..11];

        let name = name.iter().rposition(|&b| b != b' ')
            .map(|i| &name[..=i]).unwrap_or(name);
        let ext  = ext.iter().rposition(|&b| b != b' ')
            .map(|i| &ext[..=i]).unwrap_or(ext);

        if let (Ok(n), Ok(e)) = (core::str::from_utf8(name), core::str::from_utf8(ext)) {
            if attr & 0x10 != 0 {
                print!("{}/\n", n);
            } else if e.is_empty() {
                print!("{}\n", n);
            } else {
                print!("{}.{}\n", n, e);
            }
        }
    }
}

pub unsafe fn cat(name: &str) {
    let bpb = match read_bpb() {
        Ok(b) => b,
        Err(_) => { print!("\ncat: disk not formatted as FAT32\n"); return; }
    };

    let dir = match read_chain(&bpb, bpb.root_cluster, 0) {
        Ok(d) => d,
        Err(_) => { print!("\ncat: failed to read root dir\n"); return; }
    };

    let mut name83 = [b' '; 11];
    let (base, ext) = match name.rfind('.') {
        Some(i) => (&name[..i], &name[i+1..]),
        None    => (name, ""),
    };

    for (i, b) in base.bytes().take(8).enumerate() {
        name83[i] = b.to_ascii_uppercase();
    }
    for (i, b) in ext.bytes().take(3).enumerate() {
        name83[8 + i] = b.to_ascii_uppercase();
    }

    for entry in dir.chunks(32) {
        if entry[0] == 0x00 { break; }
        if entry[0] == 0xE5 { continue; }
        let attr = entry[11];
        if attr == 0x0F || attr == 0x08 { continue; }

        if &entry[0..11] == &name83 {
            let cluster = (u16::from_le_bytes([entry[20], entry[21]]) as u32) << 16
                        |  u16::from_le_bytes([entry[26], entry[27]]) as u32;
            let size = u32::from_le_bytes([entry[28], entry[29], entry[30], entry[31]]);

            match read_chain(&bpb, cluster, size) {
                Ok(data) => {
                    print!("\n");
                    if let Ok(s) = core::str::from_utf8(&data) {
                        print!("{}", s);
                    } else {
                        print!("cat: binary file\n");
                    }
                }
                Err(_) => print!("\ncat: read failed\n"),
            }
            return;
        }
    }

    print!("\ncat: file not found: {}\n", name);
}
