use color_eyre::Result;
use gtk::{gdk::Display, gio, prelude::*, Application};
use login_view::LoginView;
use main_view::MainView;
use relm4::component::AsyncComponent;
use relm4::component::AsyncComponentController;
use relm4::component::AsyncController;
use relm4::prelude::*;
use std::convert::identity;

#[rustfmt::skip]
mod config;

mod folder_view;
mod imap_state;
mod login_view;
mod main_view;

struct App {
    main_view: Controller<MainView>,
    login_view: AsyncController<LoginView>,
    active_child: Child,
}

#[derive(Debug)]
pub enum AppMsg {
    ToMainView,
}

#[derive(Debug)]
enum Child {
    LoginView,
    MainView,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        #[name(main_window)]
        gtk::ApplicationWindow {
            set_title: Some("Email"),
            set_default_size: (1280, 720),

            match model.active_child {
               Child::LoginView => {
                    gtk::Box {
                        set_halign: gtk::Align::Fill,
                        set_valign: gtk::Align::Fill,
                        model.login_view.widget(),
                    }
                },
                Child::MainView => {
                    gtk::Box {
                        set_halign: gtk::Align::Fill,
                        set_valign: gtk::Align::Fill,
                        model.main_view.widget(),
                    }
                }
            },
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // TODO: Check if we are logged in and if not go to login
        // FIXME: Remove Placeholder
        let model = App {
            active_child: Child::LoginView,
            main_view: MainView::builder().launch(()).detach(),
            login_view: LoginView::builder()
                .launch(())
                .forward(sender.input_sender(), identity),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::ToMainView => self.active_child = Child::MainView,
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Show traces to find potential performance bottlenecks, for example
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::TRACE)
        .init();
    tracing::info!("Starting application!");

    // Load app resources
    let path = &format!(
        "{}/{}/{}.gresource",
        config::DATADIR,
        config::PKGNAME,
        config::APP_ID
    );
    let res = gio::Resource::load(path).expect("Could not load resources");
    gio::resources_register(&res);

    let app = Application::builder()
        .application_id("dev.nordgedanken.Email")
        .build();
    // Connect to signals
    app.connect_startup(|_| load_css());
    let app = RelmApp::with_app(app);
    app.run::<App>(());
    Ok(())
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource(&format!("{}/style.css", config::PATH_ID));
    if let Some(display) = Display::default() {
        gtk::StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
