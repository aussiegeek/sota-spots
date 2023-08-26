mod imp;
use crate::spot_data::SpotData;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct SpotObject(ObjectSubclass<imp::SpotObject>);
}

impl SpotObject {
    pub fn new(data: &SpotData) -> Self {
        Object::builder()
            .property("id", data.id)
            .property("activator-callsign", data.activator_callsign.clone())
            .property("comments", data.comments.clone())
            .property("callsign", data.callsign.clone())
            .property("frequency", data.frequency.clone())
            .property("mode", data.mode.clone())
            .property("summit-code", data.summit_code.clone())
            .property("summit-details", data.summit_details.clone())
            .build()
    }
}
