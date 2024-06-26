mod float_16;
mod unsigned_8;

pub use crate::dpt::float_16::{*};
pub use crate::dpt::unsigned_8::{*};

use std::fmt::{Display, Formatter};
use byteorder::{BigEndian, ByteOrder};
use crate::knxnet::KnxNetIpError;


pub trait DPT{
    fn encode(&self, buf: &mut Vec<u8>);
    fn decode(&mut self, buf: &[u8]) -> Result<(), KnxNetIpError> where Self: Sized;
    fn bit_len(&self) -> u16;
    fn unit(&self) -> &str {""}
}

impl DPT for Vec<u8> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend(self)
    }

    fn decode(&mut self,buf: &[u8]) -> Result<(), KnxNetIpError> {
        self.clear();
        if buf.len() > 1 {
            self.extend_from_slice(&buf[1..]);
        } else if buf.len() == 1{
            self.push(buf[0] & 0x3F)
        }
        Ok(())
    }

    fn bit_len(&self) -> u16 {
        8 * self.len() as u16
    }
}

impl DPT for bool {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(*self as u8)
    }

    fn decode(&mut self,buf: &[u8]) -> Result<(), KnxNetIpError> {
        if buf.len() < 1 {
            return Err(KnxNetIpError::MessageTooShort(buf.len()))
        }
        *self = buf[0] & 0x1 > 0;
        return Ok(())
    }

    fn bit_len(&self) -> u16 {
        1
    }
}

impl DPT for () {
    fn encode(&self, buf: &mut Vec<u8>) {
    }

    fn decode(&mut self,buf: &[u8]) -> Result<(), KnxNetIpError> where Self: Sized {
        Ok(())
    }

    fn bit_len(&self) -> u16 {
        return 0
    }
}

impl DPT for u16 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }

    fn decode(&mut self, buf: &[u8]) -> Result<(), KnxNetIpError> where Self: Sized {
        if buf.len() < 2 {
            return Err(KnxNetIpError::MessageTooShort(buf.len()))
        }
        *self = BigEndian::read_u16(&buf[0..2]);
        Ok(())
    }

    fn bit_len(&self) -> u16 {
        16
    }
}



