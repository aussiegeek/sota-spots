use crate::spot_data::SpotData;
use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::SpotObject)]
pub struct SpotObject {
    #[property(name = "id", get, set, type = u64, member = id)]
    #[property(name = "activator-callsign", get, set, type = String, member = activator_callsign)]
    // #[property(name = "timestamp", get,set, type= String, member = timestamp)]
    #[property(name = "comments", get, set, type = String, member = comments)]
    #[property(name = "callsign", get, set, type = String, member = callsign)]
    #[property(name = "frequency", get, set, type = String, member = frequency)]
    #[property(name = "mode", get, set, type = String, member = mode)]
    #[property(name = "summit-details", get, set, type = String, member = summit_details)]
    #[property(name = "summit-code", get, set, type = String, member = summit_code)]
    #[property(name = "highlight-color", get, set, type = String, member = highlight_color)]
    pub data: RefCell<SpotData>,
}

#[glib::object_subclass]
impl ObjectSubclass for SpotObject {
    const NAME: &'static str = "SpotObject";
    type Type = super::SpotObject;
}

#[glib::derived_properties]
impl ObjectImpl for SpotObject {}
