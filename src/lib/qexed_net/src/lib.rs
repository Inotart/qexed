use std::io::ErrorKind;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::{io::AsyncReadExt, net::TcpListener};
use std::io::{Error, Result};
use bytes::{BytesMut, Buf};
use crate::net_types::packet::{Packet, PacketState};
use crate::net_types::var_int::VarInt;
use crate::packet::decode::PacketReader;
use crate::packet::encode::PacketWriter;
use std::io::{Cursor};
use tokio::io::{ AsyncWriteExt};
use bytes::{BufMut};
use flate2::Compression;
use flate2::bufread::{ZlibDecoder, ZlibEncoder};
use std::io::Read;
pub mod mojang_online;
pub mod net_types;
pub mod packet;
pub mod player;
// 创建新的tcp服务器
pub async fn new_tcp_server(ip:&str,port:u16)->Result<TcpListener>{
    let addr = format!("{}:{}", ip, port);
    let conn = TcpListener::bind(addr).await?;
    // Server listening on 0.0.0.0:25565
    log::info!("服务器监听在 {}:{}", ip, port);
    Ok(conn)
}
// 压缩阈值：当数据包长度超过此值时启用压缩
pub struct PacketListener {
    pub socket: TcpStream,
    pub socketaddr: SocketAddr,
    pub player:Option<player::Player>,
    compression_threshold:usize,
    buffer: BytesMut,
    compression_enabled: bool, // 是否启用压缩
}

impl PacketListener {
    pub fn new(socket: TcpStream, socketaddr: SocketAddr) -> Self {
        Self {
            socket,
            socketaddr,
            buffer: BytesMut::with_capacity(4096),
            compression_enabled: false, // 默认不启用压缩
            compression_threshold:0,
            player: None,
        }
    }

    // 启用或禁用压缩
    pub fn set_compression(&mut self, enabled: bool,compression_threshold:usize) {
        self.compression_enabled = enabled;
        self.compression_threshold =compression_threshold;
    }

    pub async fn send<T: Packet>(&mut self, packet: &T) -> Result<()> {
        let mut buf = BytesMut::new();
        let mut writer = PacketWriter::new(&mut buf);
        writer.varint(&VarInt(packet.id().try_into().unwrap()));
        packet.serialize(&mut writer); // 序列化数据包
        self.send_raw(buf.freeze()).await
    }

    async fn send_raw(&mut self, data: bytes::Bytes) -> Result<()> {
        if self.compression_enabled {
            self.send_compressed(data).await
        } else {
            self.send_uncompressed(data).await
        }
    }

    async fn send_uncompressed(&mut self, data: bytes::Bytes) -> Result<()> {
        let mut buf = BytesMut::new();
        write_varint(data.len() as i32, &mut buf); // 长度字段
        buf.put(data);
        self.socket.write_all(&buf).await?;
        Ok(())
    }

    async fn send_compressed(&mut self, data: bytes::Bytes) -> Result<()> {
        let mut buf = BytesMut::new();
        
        if data.len() >= self.compression_threshold {
            // 压缩数据
            let mut encoder = ZlibEncoder::new(&data[..], Compression::default());
            let mut compressed = Vec::new();
            encoder.read_to_end(&mut compressed)?;
            
            // 写入数据长度 (VarInt)
            write_varint(data.len() as i32, &mut buf); // 未压缩长度
            buf.put_slice(&compressed);
        } else {
            // 小数据包不压缩
            write_varint(0, &mut buf); // 0 表示未压缩
            buf.put(data);
        }
        
        // 写入总长度 (VarInt)
        let mut final_buf = BytesMut::new();
        write_varint(buf.len() as i32, &mut final_buf);
        final_buf.put(buf);
        
        self.socket.write_all(&final_buf).await?;
        Ok(())
    }

    pub async fn read(&mut self) -> Result<Vec<u8>> {
        loop {
            if let Some(packet) = self.try_parse_packet()? {
                return Ok(packet);
            }

            // 从套接字读取更多数据
            let mut temp_buf = [0u8; 1024];
            match self.socket.read(&mut temp_buf).await {
                Ok(0) => return Err(Error::new(ErrorKind::ConnectionAborted, "Connection closed")),
                Ok(n) => {
                    self.buffer.extend_from_slice(&temp_buf[..n]);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e),
            }
        }
    }

    /// 尝试从缓冲区解析完整数据包（处理压缩）
    fn try_parse_packet(&mut self) -> Result<Option<Vec<u8>>> {
        // 创建缓冲区视图（不消耗数据）
        let mut buf_view = self.buffer.clone().freeze();
        
        // 1. 读取数据包长度 (VarInt)
        let packet_len = match read_varint(&mut buf_view) {
            Ok(len) => len as usize,
            Err(_) => return Ok(None), // 长度不完整
        };
        
        // 检查整个数据包是否可用
        let varint_len = self.buffer.len() - buf_view.len();
        if self.buffer.len() < varint_len + packet_len {
            return Ok(None);
        }
        
        // 消耗缓冲区中的长度字段
        self.buffer.advance(varint_len);
        
        // 提取数据包部分
        let packet_data = self.buffer.split_to(packet_len);
        
        // 2. 处理压缩
        let raw_data = if self.compression_enabled {
            self.decompress_packet(packet_data)?
        } else {
            packet_data.to_vec()
        };
        
        Ok(Some(raw_data))
    }

    /// 解压缩数据包
    fn decompress_packet(&self, data: BytesMut) -> Result<Vec<u8>> {
        let mut cursor = Cursor::new(&data);
        
        // 读取未压缩数据长度
        let uncompressed_size = read_varint(&mut cursor)? as usize;
        let header_len = cursor.position() as usize;
        
        if uncompressed_size == 0 {
            // 未压缩的数据包
            Ok(data[header_len..].to_vec())
        } else {
            // 解压缩数据
            let compressed_data = &data[header_len..];
            let mut decoder = ZlibDecoder::new(compressed_data);
            let mut decompressed = Vec::with_capacity(uncompressed_size);
            decoder.read_to_end(&mut decompressed)?;
            
            if decompressed.len() != uncompressed_size {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "Decompressed size mismatch: expected {}, got {}",
                        uncompressed_size,
                        decompressed.len()
                    ),
                ));
            }
            
            Ok(decompressed)
        }
    }
    pub async fn shutdown(&mut self)->anyhow::Result<(),std::io::Error>{
        self.socket.shutdown().await
    }
}

/// 读取 Minecraft 协议的变长整数 (VarInt)
fn read_varint<B: Buf>(buf: &mut B) -> Result<i32> {
    let mut value = 0;
    let mut position = 0;
    let mut current_byte;
    
    while position < 5 {
        if buf.remaining() == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "VarInt incomplete"));
        }
        
        current_byte = buf.get_u8();
        value |= (current_byte as i32 & 0x7F) << (7 * position);
        
        if (current_byte & 0x80) == 0 {
            return Ok(value);
        }
        
        position += 1;
    }
    
    Err(Error::new(ErrorKind::InvalidData, "VarInt too big"))
}
/// 写入 Minecraft 协议的变长整数 (VarInt)
fn write_varint(value: i32, buf: &mut BytesMut) {
    let mut val = value as u32;
    loop {
        let mut temp = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            temp |= 0x80;
        }
        buf.put_u8(temp);
        if val == 0 {
            break;
        }
    }
}
pub fn read_packet(data: Vec<u8>,status:PacketState)->anyhow::Result<Box<dyn Packet>>{
    let mut buf = BytesMut::new();
    buf.extend_from_slice(&data);
    let mut reader = PacketReader::new(Box::new(&mut buf));
    let id = reader.varint().0 as u32;
    let mut decoded = match status {
        PacketState::Configuration => crate::packet::status_pool::configuration::id_to_packet(id),
        PacketState::Handshake => crate::packet::status_pool::handshake::id_to_packet(id),
        PacketState::Login => crate::packet::status_pool::login::id_to_packet(id),
        PacketState::Play => crate::packet::status_pool::play::id_to_packet(id),
        PacketState::Status => crate::packet::status_pool::status::id_to_packet(id),
    };
    decoded.deserialize(&mut reader);
    Ok(decoded)
}
        //     // 创建 Protobuf 解码器
        //     // let mut decoder = ProtobufDecoder::new();
        //     loop {
        //         let mut buf: [u8; 1024] = [0; 1024];
        //         let n = socket.read(&mut buf).await.unwrap();
        //         if n != 0 {
        //             log::info!("数据包:{:?}", buf);
        //         }
        //     }
        // });
    // Ok(())
    // stream! {
    //     let mut rng = rand::thread_rng();
    //     loop {
    //         // 随机生成数据包
    //         let packet: Box<dyn Packet + Send> = if rng.gen_bool(0.7) {
    //             Box::new(A::new()) // 70% 概率生成 A 类型
    //         } else {
    //             Box::new(B::new()) // 30% 概率生成 B 类型
    //         };
    //         
    //         // 产生数据包到流
    //         yield packet;
    //         
    //         // 模拟异步等待（例如网络延迟）
    //         // 实际应用中应使用 listener.accept().await 接收真实数据包
    //         tokio::time::sleep(Duration::from_millis(100)).await;
    //     }
    // }
    // while let std::result::Result::Ok((mut socket, clientaddr)) = tcplistener.accept().await {
    //     tokio::spawn(async move {
    //         // 创建 Protobuf 解码器
    //         // let mut decoder = ProtobufDecoder::new();
// 
    //         loop {
    //         let mut buf: [u8; 1024] = [0; 1024];
    //         let n = socket.read(&mut buf).await.unwrap();
    //         if n!=0{
    //             log::info!("数据包:{:?}",buf);
    //         }
    //         Stream::iter(A{})
// 
    //             // 喂入数据并解析包
    //             // decoder.feed(&buf[..n]);
    //             // while let Some(packet) = decoder.next::<Handshake>() {
    //             //     handle_handshake(packet).await;
    //             // }
    //             // while let Some(packet) = decoder.next::<PlayerPosition>() {
    //             //     handle_position(packet).await;
    //             // }
    //         }
    //     });
    // }