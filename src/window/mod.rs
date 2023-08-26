mod imp;

use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};

use crate::spot_data;
use crate::spot_object::SpotObject;
use crate::spot_row::SpotRow;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn tasks(&self) -> gio::ListStore {
        // Get state
        self.imp()
            .spots
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn setup_spots(&self) {
        let model = gio::ListStore::new::<SpotObject>();

        self.imp().spots.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.tasks()));
        self.imp().spots_list.set_model(Some(&selection_model));
    }

    fn add_spots(&self) {
        let spots = spot_data::load_spots();
        spots.iter().for_each(|spot| {
            let spot_object = SpotObject::new(&spot);
            self.tasks().append(&spot_object);
        })
    }

    fn setup_callbacks(&self) {}
    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let task_row = SpotRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&task_row));
        });

        factory.connect_bind(move |_, list_item| {
            let spot_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<SpotObject>()
                .expect("The item has to be an `SpotObject`.");

            let spot_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<SpotRow>()
                .expect("The child has to be a `SpotRow`.");

            spot_row.bind(&spot_object);
        });

        factory.connect_unbind(move |_, list_item| {
            let spot_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<SpotRow>()
                .expect("The child has to be a `SpotRow`.");

            spot_row.unbind();
        });

        self.imp().spots_list.set_factory(Some(&factory));
    }
}
