use sysinfo::System;


pub fn get_hardware_info() -> (f64, f64) {
    let (mem, swap) = get_memory_info();
    (mem, swap)
}


fn get_memory_info() -> (f64, f64) {
    // Create a system object.
    let sys = System::new_all();

    // get total memory and swap space
    let memory: u64 = sys.total_memory();
    let swap: u64 = sys.total_swap();

    // TODO add param for what to return as: (bytes, mb, gb, etc)
    // Convert kilobytes to gigabytes for display.
    let total_memory_gb = memory as f64 / (1024.0 * 1024.0 * 1024.0);
    let total_swap_gb = swap as f64 / (1024.0 * 1024.0 * 1024.0);

    (total_memory_gb, total_swap_gb)
}