use crate::folder_view::{FolderInit, MailboxInit, MailboxesInit, MailboxesView};
use gtk::prelude::*;
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::prelude::*;
use std::convert::identity;

use self::about_dialog::AboutWindow;

mod about_dialog;

pub struct MainView {
    mailboxes: Controller<MailboxesView>,
    about_view: Controller<AboutWindow>,
}

#[derive(Debug)]
pub enum Msg {}

#[relm4::component(pub)]
impl Component for MainView {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = Msg;

    view! {
        #[root]
        main_window = gtk::Box {
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
                    },

                    gtk::MenuButton {
                        set_margin_all: 8,
                        set_halign: gtk::Align::End,
                        set_hexpand: true,
                        set_icon_name: "settings-symbolic",
                        #[wrap(Some)]
                        set_popover = &gtk::PopoverMenu::from_model(Some(&settings_menu)) {},
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
                        set_shrink_end_child: false,
                        set_hexpand: true,
                        #[wrap(Some)]
                        set_start_child = model.mailboxes.widget(),

                        #[wrap(Some)]
                        set_end_child = &gtk::Paned {
                            set_orientation: gtk::Orientation::Vertical,
                            set_halign: gtk::Align::Fill,
                            set_valign: gtk::Align::Fill,
                            set_shrink_start_child: false,
                            set_shrink_end_child: false,
                            set_position: 300,

                            #[wrap(Some)]
                            set_start_child = &gtk::ScrolledWindow {
                                set_hscrollbar_policy: gtk::PolicyType::Never,

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
            }
    }

    // Settingsmenu
    menu! {
        settings_menu: {
            "About" => AboutAction,
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

        let about_view = AboutWindow::builder()
            .transient_for(root)
            .launch(())
            .detach();

        let model = MainView {
            mailboxes,
            about_view,
        };
        let widgets = view_output!();

        let group = RelmActionGroup::<WindowActionGroup>::new();
        let action: RelmAction<AboutAction> = {
            let sender = model.about_view.sender().clone();
            RelmAction::new_stateless(move |_| {
                sender.send(()).unwrap();
            })
        };
        group.add_action(&action);

        let actions = group.into_action_group();
        widgets
            .main_window
            .insert_action_group("win", Some(&actions));

        ComponentParts { model, widgets }
    }
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
