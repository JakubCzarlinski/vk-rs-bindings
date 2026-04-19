use wayland_client;
use wayland_client::protocol::{wl_region, wl_surface};

pub mod __interfaces {
    use wayland_client;
    use wayland_client::protocol::__interfaces::{
        WL_REGION_INTERFACE, WL_SURFACE_INTERFACE, wl_region_interface, wl_surface_interface,
    };

    wayland_scanner::generate_interfaces!("./protocols/ext-background-effect-v1.xml");
}

use self::__interfaces::{
    EXT_BACKGROUND_EFFECT_MANAGER_V1_INTERFACE, EXT_BACKGROUND_EFFECT_SURFACE_V1_INTERFACE,
};

wayland_scanner::generate_client_code!("./protocols/ext-background-effect-v1.xml");
