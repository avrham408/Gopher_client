use log::*;
use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};
use url::Url;

pub fn get_ip_adress(url: String, port: u16) -> Result<Vec<SocketAddr>, Box<dyn Error>> {
    debug!("try to find ip from url - {} on port - {}", url, port);
    let st = format!("{}:{}", url, port);
    match &st.to_socket_addrs() {
        Ok(address) => {
            return Ok(address.into());
        },
        Err(E) => return Err(format!("oy").into()),
    }
}

mod tests {
    use super::*;

    #[test]
    fn start_test() {
        let url = "gopher.club".to_string();
        get_ip_adress(url, 70);
    }
}
