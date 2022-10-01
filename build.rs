fn main() {
    gio::compile_resources(
        "resources",
        "resources/resources.gresource.xml",
        "gnome-phone-companion.gresource",
    );
}