use folder_view::MailboxesView;
use gtk::{gdk::Display, gio, prelude::*, Application, CssProvider, StyleContext};
use relm4::prelude::*;
use std::convert::identity;

use crate::folder_view::{FolderInit, MailboxInit, MailboxesInit};

mod folder_view;

struct App {
    mailboxes: Controller<MailboxesView>,
}

#[derive(Debug)]
enum Msg {}

#[relm4::component]
impl Component for App {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = Msg;

    view! {
        gtk::ApplicationWindow {
            set_title: Some("Email"),
            set_default_size: (1280, 720),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_halign: gtk::Align::Fill,
                set_valign: gtk::Align::Fill,

                gtk::Box {
                    set_halign: gtk::Align::Fill,
                    set_valign: gtk::Align::Baseline,

                    gtk::Button {
                        set_margin_all: 8,
                        set_label: "Sync"
                    },

                    gtk::Button {
                        set_margin_all: 8,
                        set_label: "Write"
                    }
                },

                gtk::Separator {},

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Fill,
                    set_valign: gtk::Align::Fill,
                    set_vexpand: true,
                    set_hexpand: true,

                    gtk::Paned {
                        set_position: 200,
                        set_shrink_start_child: false,
                        set_hexpand: true,
                        #[wrap(Some)]
                        set_start_child = model.mailboxes.widget(),

                        #[wrap(Some)]
                        set_end_child = &gtk::Paned {
                            set_orientation: gtk::Orientation::Vertical,
                            set_halign: gtk::Align::Fill,
                            set_valign: gtk::Align::Fill,
                            set_shrink_start_child: false,
                            set_position: 300,

                            #[wrap(Some)]
                            set_start_child = &gtk::ScrolledWindow {

                            },
                            #[wrap(Some)]
                            set_end_child = &gtk::Box {
                                add_css_class: "main",
                                set_margin_all: 8,
                                set_halign: gtk::Align::Start,
                                set_valign: gtk::Align::Start,
                                set_spacing: 8,

                                gtk::Label {
                                    set_label: "test2"
                                }
                            }
                        }
                    },
                }
            },
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mailboxes = MailboxesView::builder()
            .launch(MailboxesInit {
                mailboxes: vec![MailboxInit {
                    icon_name: String::from("mail-symbolic"),
                    mailbox_name: String::from("dont@dox.myself"),
                    folders: vec![FolderInit {
                        icon_name: String::from("inbox-symbolic"),
                        folder_name: String::from("Inbox"),
                        depth: 1,
                    }],
                }],
            })
            .forward(sender.input_sender(), identity);
        let model = App { mailboxes };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    // Show traces to find potential performance bottlenecks, for example
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::TRACE)
        .init();
    tracing::info!("Starting application!");

    // Register and include resources
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("dev.nordgedanken.Email")
        .build();
    // Connect to signals
    app.connect_startup(|_| load_css());
    let app = RelmApp::with_app(app);
    app.run::<App>(());
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
