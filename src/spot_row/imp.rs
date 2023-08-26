use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};
use std::cell::RefCell;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/vk3xe/SotaSpots/spot_row.ui")]
pub struct SpotRow {
    #[template_child]
    pub activator_callsign: TemplateChild<Label>,
    #[template_child]
    pub mode: TemplateChild<Label>,
    #[template_child]
    pub frequency: TemplateChild<Label>,
    #[template_child]
    pub summit_code: TemplateChild<Label>,
    #[template_child]
    pub summit_details: TemplateChild<Label>,

    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SpotRow {
    const NAME: &'static str = "SpotRow";
    type Type = super::SpotRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SpotRow {}

impl WidgetImpl for SpotRow {}

impl BoxImpl for SpotRow {}
