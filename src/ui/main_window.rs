use adw::{
    gio, glib, subclass::prelude::*,

};

use gtk::{
CompositeTemplate,prelude::*,subclass::prelude::*,
glib::{Object, subclass::InitializingObject}
};

use crate::ui;

mod template {
    use adw::glib::subclass::InitializingObject;
    use adw::gtk::subclass::prelude::{ApplicationWindowImpl, CompositeTemplateClass, WidgetImpl, WindowImpl};
    use adw::gtk::TemplateChild;
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/bil4x4/gnome-phone-companion/main_window.ui")]
    pub(crate) struct MainWindow {
        #[template_child]
        pub(super) contacts_placeholder: TemplateChild<gtk::Box> ,

        /* #[template_child]
         pub messages_page: TemplateChild<adw::ViewStackPage>,*/
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
            obj.add_pages();
        }
    }

    impl WidgetImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl ApplicationWindowImpl for MainWindow {}
    impl AdwApplicationWindowImpl for MainWindow {}
}

glib::wrapper! {
    pub(crate) struct MainWindow(ObjectSubclass<template::MainWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

fn print_type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create window")
    }

    fn add_pages(&self) {
        let contacts_page = ui::contacts_container::ContactsContainer::new();
        let leaflet = contacts_page.imp().leaflet.get();
        println!("self type: {}", print_type_of(&self));

        let mut placeholder = &self.imp().contacts_placeholder.get();

        placeholder.append(&leaflet);
    }
}