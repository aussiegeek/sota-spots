mod imp;
use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::spot_object::SpotObject;

glib::wrapper! {
    pub struct SpotRow(ObjectSubclass<imp::SpotRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for SpotRow {
    fn default() -> Self {
        Self::new()
    }
}

impl SpotRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, spot_object: &SpotObject) {
        let mut bindings = self.imp().bindings.borrow_mut();

        bindings.push(
            spot_object
                .bind_property(
                    "activator-callsign",
                    &self.imp().activator_callsign.get(),
                    "label",
                )
                .sync_create()
                .build(),
        );

        bindings.push(
            spot_object
                .bind_property("mode", &self.imp().mode.get(), "label")
                .sync_create()
                .build(),
        );

        bindings.push(
            spot_object
                .bind_property("frequency", &self.imp().frequency.get(), "label")
                .sync_create()
                .build(),
        );

        bindings.push(
            spot_object
                .bind_property("summit-code", &self.imp().summit_code.get(), "label")
                .sync_create()
                .build(),
        );

        bindings.push(
            spot_object
                .bind_property("summit-details", &self.imp().summit_details.get(), "label")
                .sync_create()
                .build(),
        );
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
