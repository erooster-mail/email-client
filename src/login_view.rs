use crate::imap_state::ImapSession;
use crate::imap_state::IMAP_SESSION;
use crate::AppMsg;
use color_eyre::{eyre::bail, Result};
use gtk::prelude::*;
use once_cell::sync::Lazy;
use quick_xml::de::from_str;
use relm4::component::AsyncComponent;
use relm4::component::AsyncComponentParts;
use relm4::loading_widgets::LoadingWidgets;
use relm4::prelude::*;
use relm4::AsyncComponentSender;
use relm4::{view, Worker, WorkerController};
use reqwest::StatusCode;
use serde::Deserialize;
use std::convert::identity;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

pub struct LoginView {
    // stores entered values
    email: gtk::EntryBuffer,
    password: gtk::PasswordEntryBuffer,
    login_worker: WorkerController<AsyncLoginHandler>,
}

#[derive(Debug)]
pub enum LoginMsg {
    Finished,
    StartLogin,
}

#[relm4::component(async, pub)]
impl AsyncComponent for LoginView {
    type Init = ();
    type Input = LoginMsg;
    type Output = AppMsg;
    type CommandOutput = LoginMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Fill,
            set_valign: gtk::Align::Fill,
            set_hexpand: true,
            set_vexpand: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 8,
                set_spacing: 8,
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,

                gtk::Entry {
                    set_tooltip_text: Some("Username"),
                    set_buffer: &model.email,
                },
                gtk::Entry {
                    set_tooltip_text: Some("Password"),
                    set_buffer: &model.password,
                    set_visibility: false,
                },
                gtk::Button {
                    set_label: "Login",
                    connect_clicked => LoginMsg::StartLogin,
                },
            }
        }
    }

    fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local_ref]
            root {
                // This will be removed automatically by
                // LoadingWidgets when the full view has loaded
                #[name(spinner)]
                gtk::Spinner {
                    start: (),
                    set_halign: gtk::Align::Center,
                }
            }
        }
        Some(LoadingWidgets::new(root, spinner))
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        // TODO: Check if logged in and exit early if needed
        let model = LoginView {
            email: gtk::EntryBuffer::new(None::<String>),
            password: gtk::PasswordEntryBuffer::new(),
            login_worker: AsyncLoginHandler::builder()
                .detach_worker(())
                .forward(sender.input_sender(), identity),
        };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        tracing::info!("Got msg1: {:?}", msg);
        match msg {
            LoginMsg::StartLogin => {
                tracing::info!("Starting login");
                self.login_worker
                    .sender()
                    .send(AsyncLoginHandlerMsg::StartLogin(
                        self.email.text().to_string(),
                        self.password.text().to_string(),
                    ))
                    .unwrap();
            }
            LoginMsg::Finished => {
                tracing::info!("Login finished");
                sender.output(AppMsg::ToMainView).unwrap();
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
enum AsyncLoginHandlerMsg {
    StartLogin(String, String),
}

struct AsyncLoginHandler;

impl Worker for AsyncLoginHandler {
    type Init = ();
    type Input = AsyncLoginHandlerMsg;
    type Output = LoginMsg;

    fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: AsyncLoginHandlerMsg, sender: ComponentSender<Self>) {
        tracing::info!("Got msg: {:?}", msg);
        match msg {
            AsyncLoginHandlerMsg::StartLogin(email, password) => {
                RUNTIME.block_on(async move {
                    tracing::info!("Starting login inner");
                    let server_url = email.split('@').last().unwrap().to_string();
                    let config = get_autoconfig(server_url, email.clone()).await;
                    match config {
                        Ok(config) => {
                            tracing::info!("Got config: {:?}", config);

                            let incoming_server = config
                                .email_provider
                                .incoming_servers
                                .iter()
                                .find(|d| d.socket_type == "SSL")
                                .unwrap();

                            let client = imap::ClientBuilder::new(
                                incoming_server.hostname.clone(),
                                incoming_server.port.parse::<u16>().unwrap(),
                            )
                            .rustls()
                            .unwrap();

                            // TODO: Use OAUth2 if https://wiki.mozilla.org/Thunderbird:Autoconfiguration:ConfigFileFormat#OAuth2 is set

                            let auth_data = match incoming_server.username.as_str() {
                                "%EMAILADDRESS%" => PlainAuth { email, password },
                                "%EMAILLOCALPART%" => {
                                    let email = email.split('@').next().unwrap().to_string();
                                    PlainAuth { email, password }
                                }
                                "%EMAILDOMAIN%" => {
                                    let email = email.split('@').last().unwrap().to_string();
                                    PlainAuth { email, password }
                                }
                                _ => {
                                    tracing::error!("Username type not supported");
                                    return;
                                }
                            };

                            // the client we have here is unauthenticated.
                            // to do anything useful with the e-mails, we need to log in
                            // FIXME: Use correct authentication method
                            let imap_session = client
                                .authenticate("PLAIN", &auth_data)
                                .map_err(|e| e.0)
                                .unwrap();

                            *IMAP_SESSION.write() = ImapSession::TLS(imap_session);
                            sender.output(LoginMsg::Finished).unwrap();
                        }
                        Err(e) => {
                            tracing::warn!("Unable to get config: {:?}", e);
                        }
                    }
                });
            }
        };
    }
}

struct PlainAuth {
    email: String,
    password: String,
}

impl imap::Authenticator for PlainAuth {
    type Response = String;
    fn process(&self, _: &[u8]) -> Self::Response {
        format!("\0{}\0{}", self.email, self.password)
    }
}

async fn get_autoconfig(server_url: String, email: String) -> Result<AutoconfigXML> {
    // First check subdomain with email
    // TODO: Parse xml
    let resp = reqwest::get(format!(
        "https://autoconfig.{server_url}/mail/config-v1.1.xml?emailaddress={email}"
    ))
    .await;

    match resp {
        Ok(resp) => {
            if resp.status() != StatusCode::OK {
                tracing::info!("Not found. Trying well-known");
                return get_autoconfig_wellknown(server_url).await;
            }

            let xml_text = resp.text().await?;
            let parsed_xml: AutoconfigXML = from_str(&xml_text)?;

            Ok(parsed_xml)
        }
        Err(_) => {
            tracing::info!("Not found. Trying well-known");
            get_autoconfig_wellknown(server_url).await
        }
    }
}

async fn get_autoconfig_wellknown(server_url: String) -> Result<AutoconfigXML> {
    let resp = reqwest::get(format!(
        "https://{server_url}/.well-known/autoconfig/mail/config-v1.1.xml"
    ))
    .await?;

    if resp.status() != StatusCode::OK {
        // TODO: Check DNS
        bail!("Unable to find server autoconfig");
    }

    let xml_text = resp.text().await?;
    let parsed_xml: AutoconfigXML = from_str(&xml_text)?;
    Ok(parsed_xml)
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct AutoconfigXML {
    #[serde(rename = "@version")]
    version: String,
    email_provider: EmailProviderXML,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EmailProviderXML {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "domain", default)]
    domains: Vec<String>,
    display_name: String,
    display_short_name: String,
    documentation: Option<DocumentationXML>,
    #[serde(rename = "incomingServer", default)]
    incoming_servers: Vec<IncomingServerXML>,
    #[serde(rename = "outgoingServer", default)]
    outgoing_servers: Vec<OutgoingServerXML>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct DocumentationXML {
    url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct IncomingServerXML {
    #[serde(rename = "@type", default)]
    servertype: String,
    hostname: String,
    port: String,
    socket_type: String,
    authentication: String,
    username: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct OutgoingServerXML {
    #[serde(rename = "@type", default)]
    servertype: String,
    hostname: String,
    port: String,
    socket_type: String,
    authentication: String,
    username: String,
}
