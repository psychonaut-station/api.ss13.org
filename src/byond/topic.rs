use std::{net::SocketAddr, time::Duration};
use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt as _},
    net::TcpStream,
    time::timeout,
};

use super::Result;

const BYOND_PACKET_HEADER_SIZE: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PacketHeader {
    packet_type: u16,
    data_size: usize,
}

#[derive(Debug)]
pub enum ResponseDataType {
    Null = 0x0,
    Number = 0x2A,
    String = 0x6,
}

pub async fn topic(address: &str, data: &str) -> Result<(ResponseDataType, Vec<u8>)> {
    let data_size = data.len() as u8 + 6;
    let mut packet_data = vec![0x00, 0x83, 0x00, data_size];
    packet_data.extend(vec![0x00; 5]);
    packet_data.extend(data.as_bytes());
    packet_data.push(0x00);

    let address: SocketAddr = address.parse()?;
    let mut stream = timeout(Duration::from_millis(100), TcpStream::connect(address)).await??;
    stream.write_all(&packet_data).await?;

    let mut response_header_data = [0; BYOND_PACKET_HEADER_SIZE];
    stream.read_exact(&mut response_header_data).await?;

    let response_header = PacketHeader {
        packet_type: u16::from_be_bytes([response_header_data[0], response_header_data[1]]),
        data_size: u16::from_be_bytes([response_header_data[2], response_header_data[3]]) as usize,
    };

    let mut response_data = vec![0; response_header.data_size];
    stream.read_exact(&mut response_data).await?;

    let response_data_type = if response_data.len() > 2 {
        match response_data[0] {
            0x0 => ResponseDataType::Null,
            0x2A => ResponseDataType::Number,
            0x6 => ResponseDataType::String,
            _ => unreachable!(),
        }
    } else {
        ResponseDataType::Null
    };

    if matches!(response_data_type, ResponseDataType::Null) {
        return Ok((response_data_type, vec![]));
    }

    Ok((response_data_type, response_data[1..].to_vec()))
}
