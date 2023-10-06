use std::env;
use vm_info::{self, ProcessId};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <pid>", args[0]);
        return;
    }
    let pid: u32 = args[1].parse::<u32>().unwrap();
    let page_size = 4096;
    
    let infos = vm_info::mapped_region::iter_mappings(ProcessId::Num(pid)).unwrap();
    for info in infos {
        let vpage = info.unwrap().start_address/page_size;
        let phys = vm_info::page_map::read_page_map(ProcessId::Num(pid), vpage).unwrap();
        let phys = phys.page_frame();
        println!("Virtual page: \t{:x}\tPhysical page: \t{:x}", vpage, phys.unwrap_or(0));
    }
}
