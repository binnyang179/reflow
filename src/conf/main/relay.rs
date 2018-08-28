use std::net::IpAddr;
use std::net::SocketAddr;
use std::fmt;
use conf::RoutingBranch;
use conf::main::util::RefVal;
use conf::main::dns::NameServer;
use conf::NameServerRemote;

pub struct Relay {
    pub resolver: Option<NameServer>,
    pub listen: RelayProto,
    pub rule: RefVal<RoutingBranch>,
}
impl fmt::Debug for Relay {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Relay on {:?}", self.listen)?;
        Ok(())
    }
}
#[derive(Debug)]
pub enum RelayProto {
    Socks5(SocketAddr),
}


impl Relay {
    pub fn nameserver_or_default(&self)-> NameServer {
        match self.resolver {
            Some(ref r) => r.clone(),
            None => {
                NameServer {
                    egress: None,
                    remote: NameServerRemote::Udp(
                        SocketAddr::new(
                            IpAddr::from([8,8,8,8]),
                            53)),
                }
            }
        }
    }
}