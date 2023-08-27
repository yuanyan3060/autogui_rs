use std::io::{Read, Write};
use std::net::TcpStream;

use crate::error::AGResult;

use super::{AGError, Controller};

pub struct RecvData {
    pub is_ok: bool,
    pub data: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct AdbBuilder {
    addr: Option<String>,
    timeout: Option<std::time::Duration>,
    bin_path: Option<String>,
    target: Option<String>,
}

impl AdbBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_addr(mut self, addr: &str) -> Self {
        self.addr = Some(addr.to_string());
        self
    }

    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_bin_path(mut self, bin_path: &str) -> Self {
        self.bin_path = Some(bin_path.to_string());
        self
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    pub fn build(self) -> Result<ADB, AGError> {
        let addr = self.addr.unwrap_or("127.0.0.1:5037".to_string());
        let timeout = self.timeout.unwrap_or(std::time::Duration::from_secs(3));
        #[cfg(target_os = "windows")]
        let bin_path = self.bin_path.unwrap_or("adb.exe".to_string());
        #[cfg(not(target_os = "windows"))]
        let bin_path = self.bin_path.unwrap_or("adb".to_string());
        let target = self.target.clone().unwrap_or("127.0.0.1:5555".to_string());
        let stream = match TcpStream::connect(&addr) {
            Ok(stream) => stream,
            Err(_) => {
                ADB::start_daemon(&bin_path)?;
                TcpStream::connect(&addr)?
            }
        };
        stream.set_read_timeout(Some(timeout))?;
        stream.set_write_timeout(Some(timeout))?;
        let mut adb = ADB {
            stream: stream,
            target: target,
        };
        if let Some(target) = self.target {
            adb.connect(&target)?;
        }
        Ok(adb)
    }
}

#[derive(Debug)]
pub struct ADB {
    pub stream: TcpStream,
    pub target: String,
}

impl ADB {
    pub(crate) fn send_data(&mut self, data: &[u8]) -> Result<(), AGError> {
        let length = data.len() as u16;
        let length = hex::encode_upper(length.to_be_bytes());
        self.stream.write(length.as_bytes())?;
        self.stream.write(data)?;
        Ok(())
    }

    pub(crate) fn check_okay(&mut self) -> Result<bool, AGError> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf)?;
        return Ok(buf.eq(b"OKAY"));
    }

    /*pub(crate) fn recv_data(&mut self) -> Result<RecvData, AGError> {
        let mut length_buf = [0u8; 4];
        let mut length_u16 = [0u8; 2];
        let is_ok = self.check_okay()?;
        self.stream.read_exact(&mut length_buf)?;
        hex::decode_to_slice(&length_buf, &mut length_u16).map_err(|_| AGError::Decode)?;
        let length = u16::from_be_bytes(length_u16) as usize;
        let mut data = Vec::with_capacity(length);
        unsafe {
            data.set_len(length);
        }
        self.stream.read_exact(&mut data)?;
        Ok(RecvData { is_ok, data: data })
    }*/

    pub(crate) fn reset(&mut self) -> Result<(), AGError> {
        let addr = self.stream.peer_addr()?;
        let read_timeout = self.stream.read_timeout()?;
        let write_timeout = self.stream.write_timeout()?;
        self.stream.set_read_timeout(read_timeout)?;
        self.stream.set_write_timeout(write_timeout)?;
        self.stream = TcpStream::connect(addr)?;
        Ok(())
    }

    /*pub(crate) fn request(&mut self, data: &[u8]) -> Result<Vec<u8>, AGError> {
        self.reset()?;
        self.send_data(data)?;
        let result = self.recv_data()?;
        if !result.is_ok {
            return Err(AGError::Custom(
                String::from_utf8_lossy(&result.data).to_string(),
            ));
        } else {
            Ok(result.data)
        }
    }*/

    pub(crate) fn transport(&mut self) -> Result<(), AGError> {
        let mut buffer = Vec::with_capacity(200);
        buffer.extend_from_slice(b"host:transport:");
        buffer.extend_from_slice(self.target.as_bytes());
        self.send_data(&buffer)?;
        if !self.check_okay()? {
            return Err(AGError::Custom("transport fail".to_string()));
        }
        return Ok(());
    }
    pub fn shell(&mut self, cmd: &str) -> Result<RecvData, AGError> {
        self.transport()?;
        let mut buffer = Vec::with_capacity(200);
        buffer.extend_from_slice(b"shell:");
        buffer.extend_from_slice(cmd.as_bytes());
        self.send_data(&buffer)?;
        let is_ok = self.check_okay()?;
        let mut data = Vec::new();
        self.stream.read_to_end(&mut data)?;
        self.reset()?;
        Ok(RecvData { is_ok, data })
    }

    pub fn exec(&mut self, cmd: &str) -> Result<RecvData, AGError> {
        self.transport()?;
        let mut buffer = Vec::with_capacity(200);
        buffer.extend_from_slice(b"exec:");
        buffer.extend_from_slice(cmd.as_bytes());
        self.send_data(&buffer)?;
        let is_ok = self.check_okay()?;
        let mut data = Vec::new();
        self.stream.read_to_end(&mut data)?;
        self.reset()?;
        Ok(RecvData { is_ok, data })
    }

    pub fn connect(&mut self, target: &str) -> Result<(), AGError> {
        self.transport()?;
        self.send_data(&format!("host:connect:{}", target).as_bytes())?;
        self.check_okay()?;
        let mut data = Vec::new();
        self.stream.read_to_end(&mut data)?;
        self.reset()?;
        self.target = target.to_string();
        Ok(())
    }

    pub fn start_daemon(bin_path: &str) -> Result<(), AGError> {
        std::process::Command::new(bin_path).arg("start-server").spawn()?;
        Ok(())
    }

    pub fn into_dyn(self) -> Box<dyn Controller> {
        Box::new(self) as _
    }
}

impl Controller for ADB {
    fn screenshot(&mut self) -> AGResult<image::RgbaImage> {
        let recv = self.exec("screencap -p")?;
        if !recv.is_ok {
            return Err(AGError::Custom(String::from_utf8_lossy(&recv.data).to_string()));
        }
        let img = image::load_from_memory(&recv.data)?;
        Ok(img.to_rgba8())
    }

    fn click(&mut self, x: u32, y: u32) -> AGResult<()> {
        self.shell(&format!("input tap {} {}", x, y))?;
        Ok(())
    }

    fn swipe(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) -> AGResult<()> {
        self.shell(&format!("input swipe {} {} {} {}", x1, y1, x2, y2))?;
        Ok(())
    }

    fn press_key(&mut self, keycode: u32) -> AGResult<()> {
        self.shell(&format!("input keyevent {}", keycode))?;
        Ok(())
    }

    fn get_resolution(&mut self) -> AGResult<(u32, u32)> {
        todo!()
    }

    fn input_text(&mut self, text: &str) -> AGResult<()> {
        self.shell(&format!("input text {}", text))?;
        Ok(())
    }
}
