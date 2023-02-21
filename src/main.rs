use gtk::{gdk::Display, gio, prelude::*, Application};
use main_view::MainView;
use relm4::prelude::*;
use std::convert::identity;

#[rustfmt::skip]
mod config;

mod folder_view;
mod main_view;

struct App {
    main_view: Controller<MainView>,
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
        #[root]
        main_window = gtk::ApplicationWindow {
            set_title: Some("Email"),
            set_default_size: (1280, 720),

            model.main_view.widget(),
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            main_view: MainView::builder()
                .launch(())
                .forward(sender.input_sender(), identity),
        };
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
