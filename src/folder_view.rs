use gtk::{prelude::*, Align, Orientation};
use relm4::{factory::FactoryVecDeque, prelude::*};

pub struct MailboxesView {
    mailboxes: FactoryVecDeque<Mailbox>,
}

pub struct MailboxesInit {
    pub mailboxes: Vec<MailboxInit>,
}

#[derive(Debug)]
pub enum MailboxesMsg {
    AddMailbox(MailboxInit),
    // TODO: Is this the correct type?
    RemoveMailbox(MailboxInit),
}

#[relm4::component(pub)]
impl SimpleComponent for MailboxesView {
    type Init = MailboxesInit;
    type Input = MailboxesMsg;
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_propagate_natural_width: true,
            #[local_ref]
            mailbox_list_box -> gtk::Box {
                set_orientation: Orientation::Vertical,
                set_halign: Align::Fill,
                set_valign: Align::Fill,
                set_hexpand: true,
                set_homogeneous: true,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = MailboxesView {
            mailboxes: FactoryVecDeque::new(gtk::Box::default(), sender.input_sender()),
        };

        for mailbox in init.mailboxes {
            model.mailboxes.guard().push_back(mailbox);
        }
        let mailbox_list_box = model.mailboxes.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: MailboxesMsg, _sender: ComponentSender<Self>) {
        match msg {
            MailboxesMsg::AddMailbox(mailbox) => self.mailboxes.guard().push_back(mailbox),
            MailboxesMsg::RemoveMailbox(mailbox) => todo!(),
        };
    }
}

struct Mailbox {
    icon_name: String,
    mailbox_name: String,
    folders: FactoryVecDeque<Folder>,
}

#[derive(Debug)]
enum MailboxInput {
    SetIconName(String),
    SetName(String),
}

#[derive(Debug)]
pub struct MailboxInit {
    pub icon_name: String,
    pub mailbox_name: String,
    pub folders: Vec<FolderInit>,
}

#[relm4::factory]
impl FactoryComponent for Mailbox {
    type Init = MailboxInit;
    type Input = MailboxInput;
    type Output = ();
    type CommandOutput = ();
    type ParentInput = MailboxesMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Expander {
            set_expanded: true,
            add_css_class: "expander",
            set_halign: Align::Fill,
            set_hexpand: true,
            #[wrap(Some)]
            set_label_widget = &gtk::Box {
                gtk::Image {
                    set_margin_end: 8,
                    #[watch]
                    set_icon_name: Some(&self.icon_name)
                },
                gtk::Label {
                    #[watch]
                    set_label: &self.mailbox_name,
                }
            },

            #[wrap(Some)]
            set_child = self.folders.widget(),
        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let box_parent = gtk::Box::default();
        box_parent.set_homogeneous(true);
        box_parent.set_halign(Align::Fill);
        let mut model = Self {
            icon_name: value.icon_name,
            mailbox_name: value.mailbox_name,
            folders: FactoryVecDeque::new(box_parent, sender.input_sender()),
        };

        for folder in value.folders {
            model.folders.guard().push_back(folder);
        }

        model
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            MailboxInput::SetIconName(icon_name) => {
                self.icon_name = icon_name;
            }
            MailboxInput::SetName(mailbox_name) => {
                self.mailbox_name = mailbox_name;
            } // TODO: Add and Remove Folders
        }
    }
}

struct Folder {
    icon_name: String,
    folder_name: String,
    pub depth: i32,
}

#[derive(Debug)]
enum FolderInput {
    SetIconName(String),
    SetName(String),
}

#[derive(Debug)]
pub struct FolderInit {
    pub icon_name: String,
    pub folder_name: String,
    pub depth: i32,
}

#[relm4::factory]
impl FactoryComponent for Folder {
    type Init = FolderInit;
    type Input = FolderInput;
    type Output = ();
    type CommandOutput = ();
    type ParentInput = MailboxInput;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Button {
            add_css_class: "tree-item",
            set_has_frame: false,
            set_halign: Align::Fill,

            gtk::Box {
                set_margin_start: 40_i32*self.depth,
                set_halign: Align::Fill,
                gtk::Image {
                    set_margin_end: 8,
                    #[watch]
                    set_icon_name: Some(&self.icon_name)
                },
                gtk::Label {
                    #[watch]
                    set_label: &self.folder_name,
                }
            }

        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            depth: value.depth,
            icon_name: value.icon_name,
            folder_name: value.folder_name,
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            FolderInput::SetIconName(icon_name) => {
                self.icon_name = icon_name;
            }
            FolderInput::SetName(folder_name) => {
                self.folder_name = folder_name;
            }
        }
    }
}
