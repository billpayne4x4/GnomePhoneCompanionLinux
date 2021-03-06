#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <gtkmm.h>
#include "widgets/tabs/contactsTab.h"
#include "server/server.h"

namespace gpc
{
    class mainWindow
    {
    public:
        mainWindow(const Glib::RefPtr<Gtk::Application> app);
        virtual ~mainWindow();

    protected:
    private:
        Glib::RefPtr<Gtk::Application> application;
        Glib::RefPtr<Glib::MainLoop> mainLoop;
        Glib::RefPtr<Gtk::Builder> builder;
        Gtk::Window *window;
        Gtk::Viewport *viewportContacts;
        widgets::tabs::contactsTab contacts;
        server::server appServer;

        void startServer();
    };
}

#endif // MAINWINDOW_H