
// use cty::*;

// This is where you should define the types shared by the kernel and user
// space, eg:
//
#[repr(C)]
#[derive(Debug)]
pub struct PacketInfo {
    pub host: u32,      // IP address as 32bit int
    pub len: u32,     // Size of packet
    pub port: u32
}
