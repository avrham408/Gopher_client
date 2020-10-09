#![allow(dead_code)]

use crate::error::Error;
use log::*;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

const TIME_OUT: std::time::Duration = Duration::from_secs(5);

pub fn get_ip_address(url: String, port: u16) -> Result<Vec<SocketAddr>, Error> {
    debug!("try to find ip from url - {} on port - {}", url, port);
    let st = format!("{}:{}", url, port);
    match &st.to_socket_addrs() {
        Ok(address) => {
            let mut vec_of_add = Vec::new();
            for a in address.clone() {
                vec_of_add.push(a);
            }
            return Ok(vec_of_add);
        }
        Err(_) => return Err(Error::GetIpFromAddrs((url, port))),
    }
}

pub fn open_connection(address: &SocketAddr) -> Result<TcpStream, Error> {
    info!("try to connect {}:{}", address.ip(), address.port());
    let stream = TcpStream::connect_timeout(address, TIME_OUT);
    match stream {
        Ok(st) => Ok(st),
        Err(_) => {
            info!("connection to {}:{} failed", address.ip(), address.port());
            Err(Error::ConnectToServer((
                address.ip().to_string(),
                address.port(),
            )))
        }
    }
}

pub fn write_to_stream(stream: &mut TcpStream, msg: Vec<u8>) -> Result<(), Error> {
    info!("write msg to {:?}", stream);
    let res = stream.write(&msg);
    match res {
        Err(_) => {
            error!("write to server failed");
            Err(Error::WriteError)
        }
        Ok(size) => {
            if size != msg.len() {
                warn!("not all buffer sent to server");
                return Err(Error::PartialBufferWriten);
            }
            return Ok(());
        }
    }
}

pub fn read_from_stream(stream: &mut TcpStream ) ->  Result<Vec<u8>, Error>
{
    let mut buf = vec![];
    match stream.read_to_end(&mut buf){
        Ok(_) => Ok(buf),
        Err(_) => {
            info!("reading from stream failed");
            Err(Error::PartialBufferWriten)
        },
    }
}



mod tests {
    #![allow(unused_imports)]
    use super::*;
    use std::net::{Ipv4Addr};

    #[test]
    fn test_get_ip_address_valid() {
        let url = "gopher.club".to_string();
        let r = get_ip_address(url, 70).unwrap();
        assert_eq!(r[0].ip(), Ipv4Addr::new(205, 166, 94, 17));
    }
    #[test]
    fn test_get_ip_address_not_exist() {
        let url = "gopher.claub".to_string();
        let r = get_ip_address(url, 60);
        match r {
            Ok(_) => unreachable!(),
            Err(err) => {
                let costume_err = Error::GetIpFromAddrs(("gopher.claub".to_string(), 60));
                assert_eq!(err, costume_err);
            }
        }
    }

    #[test]
    fn test_get_ip_address_many_address() {
        let url = "google.com".to_string();
        let r = get_ip_address(url, 443).unwrap();
        assert!(r.len() > 1);
    }

    #[test]
    fn test_get_ip_address_with_path() {
        let url = "gopher.club/1".to_string();
        let r = get_ip_address(url, 60);
        match r {
            Ok(_) => unreachable!(),
            Err(err) => {
                let costume_err = Error::GetIpFromAddrs(("gopher.club/1".to_string(), 60));
                assert_eq!(err, costume_err);
            }
        }
    }
    #[test]
    fn test_open_connection_ipv4() {
        let addresses = get_ip_address("hngopher.com".into(), 70).unwrap();
        open_connection(&addresses[0]).unwrap();
    }

    #[test]
    fn test_open_connection_failed() {
        let addresses = get_ip_address("gopher.club".into(), 60).unwrap();
        let conn = open_connection(&addresses[0]);
        match conn {
            Ok(_) => unreachable!(),
            Err(err) => {
                let costume_err = Error::ConnectToServer((addresses[0].ip().to_string(), 60));
                assert_eq!(err, costume_err);
            }
        }
    }
    #[test]
    fn test_write() {
        let addresses = get_ip_address("bitreich.org".into(), 70).unwrap();
        let mut conn = open_connection(&addresses[0]).unwrap();
        let msg = "\r\n".to_string();
        let v: Vec<u8> = msg.as_bytes().into();
        write_to_stream(&mut conn, v).unwrap();
    }

    #[test]
    fn test_read(){
        let addresses = get_ip_address("bitreich.org".into(), 70).unwrap();
        let mut conn = open_connection(&addresses[0]).unwrap();
        let msg = "\r\n".to_string();
        let v: Vec<u8> = msg.as_bytes().into();
        write_to_stream(&mut conn, v).unwrap();
        read_from_stream(&mut conn).unwrap();
    }
}
