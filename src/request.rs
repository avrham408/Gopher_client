use crate::networking::*;
use crate::error::Error;
use url::{Url, ParseError};
use log::*;

#[allow (dead_code)]
struct LocalUrl{
    schema: String,
    domain: String,
    port: u16,
    path: String,
}

const CR_LF: &str = "\r\n";

impl Default for LocalUrl{
    fn default() -> Self{
        Self{
            schema: "gopher://".to_string(),
            domain: "".to_string(),
            port: 70,
            path: "".to_string()
        }
    }
}

pub fn get(address: String) -> Result<Vec<u8>, Error>
{
    let mut e = Error::UndefinedError;
    let url = parse_url(address)?;
    let ips = get_ip_address(url.domain, url.port)?;
    for ip in ips{
        let mut stream = match open_connection(&ip){
            Ok(connection ) => connection,
            Err(er) => {
                e = er;
                continue;}
        };
        let msg = create_msg(&url.path);
        write_to_stream(&mut stream, msg)?;
        let v = read_from_stream(&mut stream)?;

        //println!("{:?}", v);
        return Ok(v)
    }
    Err(e)
}

fn create_msg(path: &String) -> Vec<u8>{
    let msg = format!("{}{}", path, CR_LF);
    msg.as_bytes().into()
}

fn parse_url(address: String) -> Result<LocalUrl, Error>{
    let mut local_url = LocalUrl::default();
    let url = match Url::parse(&address){
        Ok(url) => url,
        Err(e) => {
            if e == ParseError::RelativeUrlWithoutBase {
                return parse_url(add_schema(address));
            }
            info!("parsing {} failed", address);
            return Err(Error::ParseUrl(address));
        }
    };

    if let Some(domain) = url.domain(){
        local_url.domain = domain.into();
    } else{ // handle url that not contain schema but contain port
        if let Some(domain) = url.host_str(){
            local_url.domain = domain.into(); }
        else { return handle_url_without_schema_but_with_port(address, url);}
    }
    if let Some(port) = url.port(){
        local_url.port = port;
    }
    if let Some(path) = url.path().into() {
        if path != "/" {
            local_url.path = path.into();
        }
    }

    Ok(local_url)
}

fn add_schema(address: String) -> String{
    format!("gopher://{}", address).into()
}

fn handle_url_without_schema_but_with_port(address: String, url: Url)
    -> Result<LocalUrl, Error> {
    /* when url is: "google.com:96" the url crate just handle google.com as schema
       and 70 as domain.
       this function fix the url to gopher://google.com:96 and if the port is bigger than
       u16 or another problem the function return Parse error
    */

    if let Some(path) = url.path().into() {
        let num: Result<u16, _> = path.parse();
        return match num {
            Ok(_) => parse_url(add_schema(address)),
            Err(_) => Err(Error::ParseUrl(address)),
        }
    }

    return Err(Error::ParseUrl(address));
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_add_schema(){
        let st = "asdsad".into();
        let new_st = add_schema(st);
        assert_eq!(new_st, "gopher://asdsad".to_string());
        let st = "sadsasdwdfqwerfwkork323242.com/sadasd/asda".into();
        let new_st = add_schema(st);
        assert_eq!(new_st, "gopher://sadsasdwdfqwerfwkork323242.com/sadasd/asda".to_string())
    }

    #[test]
    fn test_parse_full_url(){
        let url = parse_url(
           "gopher://gopher.com:99/archive/2018/2018-01-08/items/16097089"
               .into()).unwrap();
        assert_eq!(url.schema, "gopher://".to_string());
        assert_eq!(url.domain, "gopher.com".to_string());
        assert_eq!(url.port, 99);
        assert_eq!(url.path, "/archive/2018/2018-01-08/items/16097089");

    }

    #[test]
    fn test_parse_with_out_port(){
        let url = parse_url(
            "gopher://gopher.com/archive/2018/2018-01-08/items/16097089"
                .into()).unwrap();
        assert_eq!(url.schema, "gopher://".to_string());
        assert_eq!(url.domain, "gopher.com".to_string());
        assert_eq!(url.port, 70);
        assert_eq!(url.path, "/archive/2018/2018-01-08/items/16097089");

    }

    #[test]
    fn test_parse_with_out_domain(){
        let address = "160970454589/adar".into();
        match parse_url(address){
            Ok(_) => unreachable!(),
            Err(e) => assert_eq!(e, Error::ParseUrl("gopher://160970454589/adar".into())),
        }
    }

    #[test]
    fn test_parse_with_port(){
        let url = parse_url("google.com:86".into()).unwrap();
        assert_eq!(url.schema, "gopher://".to_string());
        assert_eq!(url.domain, "google.com".to_string());
        assert_eq!(url.port, 86);
        assert_eq!(url.path, "".to_string());
    }
    #[test]
    fn test_parse_with_port_big_then_u16(){
        let address = "google.com:75000".into();
        match parse_url(address){
            Ok(_) => unreachable!(),
            Err(e) => assert_eq!(e, Error::ParseUrl("google.com:75000".into())),
        }
    }
    #[test]
    fn test_parse_url_local_host(){
        let url = parse_url("localhost:8000".into()).unwrap();
        assert_eq!(url.schema, "gopher://".to_string());
        assert_eq!(url.domain, "localhost".to_string());
        assert_eq!(url.port, 8000);
        assert_eq!(url.path, "".to_string());
    }

    #[test]
    fn test_parse_url_with_ip(){
        let url = parse_url("127.0.0.1:8000".into()).unwrap();
        assert_eq!(url.schema, "gopher://".to_string());
        assert_eq!(url.domain, "127.0.0.1".to_string());
        assert_eq!(url.port, 8000);
        assert_eq!(url.path, "".to_string());
    }

    #[test]
    fn test_get_req(){
        let addresses = "bitreich.org".to_string();
        let v = get(addresses).unwrap();
        assert!(!v.is_empty())
    }
}
