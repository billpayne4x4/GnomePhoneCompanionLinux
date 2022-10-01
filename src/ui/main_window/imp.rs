use gtk::{
    prelude::*,
    glib, CompositeTemplate,
    subclass::prelude::*
};
use glib::subclass::InitializingObject;
use adw::subclass::prelude::*;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/bil4x4/gnome-phone-companion/main_window.ui")]
pub struct MainWindow {
    //#[template_child]
    //pub button: TemplateChild<Button>,
}
#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        // Connect to "clicked" signal of `button`
        /*self.button.connect_clicked(move |button| {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
        });*/
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}
impl AdwApplicationWindowImpl for MainWindow {}
