use adw::{
    gtk, subclass::prelude::*,
    gtk::{
        CompositeTemplate,glib,
        prelude::*,
        subclass::prelude::*,
        glib::{Object,subclass::InitializingObject},
    },
};

pub(crate) mod template {
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/bil4x4/gnome-phone-companion/contacts_container.ui")]
    pub(crate) struct ContactsContainer {
        #[template_child]
        pub leaflet: TemplateChild<adw::Leaflet>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ContactsContainer {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "ContactsContainer";
        type Type = super::ContactsContainer;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ContactsContainer {
        fn constructed(&self, obj: &Self::Type) {
            // Call "constructed" on parent
            self.parent_constructed(obj);
        }
    }

    impl BoxImpl for ContactsContainer {}
    impl WidgetImpl for ContactsContainer {}
}

glib::wrapper! {
    pub(crate) struct ContactsContainer(ObjectSubclass<template::ContactsContainer>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ContactsContainer {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create contents container")
    }
}