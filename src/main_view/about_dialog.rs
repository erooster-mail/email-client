use crate::config::{APP_ID, VERSION};
use gtk::prelude::GtkWindowExt;
use relm4::*;

pub struct AboutWindow {}

impl SimpleComponent for AboutWindow {
    type Init = ();
    type Widgets = gtk::AboutDialog;
    type Input = ();
    type Output = ();
    type Root = gtk::AboutDialog;

    fn init_root() -> Self::Root {
        gtk::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            .license_type(gtk::License::Agpl30)
            .version(VERSION)
            .modal(true)
            .authors(vec!["MTRNord"])
            .artists(vec!["MTRNord"])
            .comments("A Rust GTK4 email client")
            .website_label("Github repository")
            .website("https://github.com/erooster-mail/email-client")
            .copyright("Licensed AGPL-v3.0-or-later license")
            .program_name("Email Client")
            .build()
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = root.clone();

        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.present();
    }
}
