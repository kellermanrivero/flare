use std::slice::Iter;

use serde::{Deserialize, Serialize};

const MAX_DATA_LENGTH: usize = 1000;

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PacketData {
    order: u32,
    size: u32,
    __unused0__: u32,
    __unused1__: u32,
    data: Vec<u8>,
}

impl PacketData {
    fn begin(width: u32, height: u32, bpp: u32) -> Self {
        let mut data = Vec::new();

        data.append(&mut width.to_be_bytes().to_vec());
        data.append(&mut height.to_be_bytes().to_vec());
        data.append(&mut bpp.to_be_bytes().to_vec());

        Self {
            order: 0xFEEDFEED,
            size: data.len() as u32,
            __unused0__: 0,
            __unused1__: 0,
            data,
        }
    }

    fn chunk(order: u32, chunk: &[u8]) -> Self {
        Self {
            order,
            size: chunk.len() as u32,
            __unused0__: 0,
            __unused1__: 0,
            data: Vec::from(chunk),
        }
    }

    fn end() -> Self {
        Self {
            order: 0xCAFECAFE,
            size: 0,
            __unused0__: 0,
            __unused1__: 0,
            data: Vec::new(),
        }
    }

    pub(crate) fn get_data(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

pub struct Frame {
    packets: Vec<PacketData>,
}

impl Frame {
    pub fn from(width: u32, height: u32, bpp: u32, data: Vec<u8>) -> Self {
        let mut packets: Vec<PacketData> = Vec::new();

        packets.push(PacketData::begin(width, height, bpp));

        let chunks = data.chunks(MAX_DATA_LENGTH);
        let mut counter = 1;
        for chunk in chunks {
            packets.push(PacketData::chunk(counter, chunk));
            counter = counter + 1;
        }

        packets.push(PacketData::end());

        Self {
            packets
        }
    }

    pub fn packets(&self) -> Iter<'_, PacketData> {
        self.packets.iter()
    }
}