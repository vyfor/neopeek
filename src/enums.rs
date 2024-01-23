#[derive(Eq, PartialEq, Hash, Debug)]
pub enum StatType {
    Os,
    Kernel,
    Cpu,
    Gpu,
    Resolution,
    Ram,
    Uptime,
    Disks,
}
