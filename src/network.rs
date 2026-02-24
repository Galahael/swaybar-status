use std::error::Error;
use std::fmt;
use zbus::blocking::{Connection, Proxy};

const DEFAULT_NETWORK: Connectivity = Connectivity::Unknown;

pub struct NetworkState {
    connectivity: Connectivity,
}
impl NetworkState {
    pub fn update(&mut self, proxy: &Proxy) {
        let value: u32 = proxy.get_property("Connectivity").unwrap();
        self.connectivity = Connectivity::from_u32(value);
    }
}
impl fmt::Display for NetworkState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let icon = match self.connectivity {
            Connectivity::Full => "✓",
            Connectivity::Limited => "⚠",
            Connectivity::Unknown => "?",
            _ => "✗",
        };
        write!(f, "{}", icon)
    }
}
impl Default for NetworkState {
    fn default() -> Self {
        Self {
            connectivity: DEFAULT_NETWORK,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Connectivity {
    Full,
    Limited,
    Portal,
    None,
    Unknown,
}
impl Connectivity {
    fn from_u32(value: u32) -> Self {
        match value {
            4 => Connectivity::Full,
            3 => Connectivity::Limited,
            2 => Connectivity::Portal,
            1 => Connectivity::None,
            _ => Connectivity::Unknown,
        }
    }
}

pub fn setup_proxy() -> Result<Proxy<'static>, Box<dyn Error>> {
    let connection = Connection::system()?;
    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
    )?;
    Ok(proxy)
}
