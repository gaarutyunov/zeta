use crate::transcription::{TranscriptionConfig, TranscriptionProvider};
use dioxus::prelude::*;

#[component]
pub fn Settings(
    config: Signal<TranscriptionConfig>,
    on_close: EventHandler<()>,
) -> Element {
    let mut temp_claude_key = use_signal(|| {
        config.read().claude_api_key.clone().unwrap_or_default()
    });
    let mut temp_openai_key = use_signal(|| {
        config.read().openai_api_key.clone().unwrap_or_default()
    });
    let mut temp_google_key = use_signal(|| {
        config.read().google_api_key.clone().unwrap_or_default()
    });
    let mut selected_provider = use_signal(|| config.read().provider.clone());

    rsx! {
        div { class: "settings-overlay",
            div { class: "settings-modal",
                div { class: "settings-header",
                    h2 { "Settings" }
                    button {
                        class: "btn-close",
                        onclick: move |_| on_close.call(()),
                        "×"
                    }
                }

                div { class: "settings-content",
                    h3 { "Transcription Provider" }
                    p { class: "text-muted", "Choose your preferred voice-to-text service" }

                    div { class: "provider-list",
                        for provider in TranscriptionProvider::all() {
                            div {
                                key: "{provider.name()}",
                                class: if selected_provider() == provider { "provider-card selected" } else { "provider-card" },
                                onclick: move |_| selected_provider.set(provider.clone()),

                                div { class: "provider-header",
                                    input {
                                        r#type: "radio",
                                        name: "provider",
                                        checked: selected_provider() == provider,
                                        onchange: move |_| selected_provider.set(provider.clone())
                                    }
                                    h4 { "{provider.name()}" }
                                    if !provider.requires_api_key() {
                                        span { class: "badge", "No API Key" }
                                    }
                                }

                                p { class: "provider-description", "{provider.description()}" }

                                if provider.requires_api_key() {
                                    div { class: "provider-api-key",
                                        match provider {
                                            TranscriptionProvider::Claude => rsx! {
                                                input {
                                                    r#type: "password",
                                                    placeholder: "sk-ant-api03-...",
                                                    value: "{temp_claude_key}",
                                                    oninput: move |evt| temp_claude_key.set(evt.value().clone())
                                                }
                                                a {
                                                    href: "https://console.anthropic.com/",
                                                    target: "_blank",
                                                    class: "api-link",
                                                    "Get API Key →"
                                                }
                                            },
                                            TranscriptionProvider::OpenAI => rsx! {
                                                input {
                                                    r#type: "password",
                                                    placeholder: "sk-...",
                                                    value: "{temp_openai_key}",
                                                    oninput: move |evt| temp_openai_key.set(evt.value().clone())
                                                }
                                                a {
                                                    href: "https://platform.openai.com/api-keys",
                                                    target: "_blank",
                                                    class: "api-link",
                                                    "Get API Key →"
                                                }
                                            },
                                            TranscriptionProvider::GoogleCloud => rsx! {
                                                input {
                                                    r#type: "password",
                                                    placeholder: "AIza...",
                                                    value: "{temp_google_key}",
                                                    oninput: move |evt| temp_google_key.set(evt.value().clone())
                                                }
                                                a {
                                                    href: "https://console.cloud.google.com/apis/credentials",
                                                    target: "_blank",
                                                    class: "api-link",
                                                    "Get API Key →"
                                                }
                                            },
                                            _ => rsx! {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "settings-footer",
                    button {
                        class: "btn",
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let mut cfg = config.write();
                            cfg.provider = selected_provider();
                            cfg.claude_api_key = if temp_claude_key().is_empty() { None } else { Some(temp_claude_key()) };
                            cfg.openai_api_key = if temp_openai_key().is_empty() { None } else { Some(temp_openai_key()) };
                            cfg.google_api_key = if temp_google_key().is_empty() { None } else { Some(temp_google_key()) };
                            on_close.call(());
                        },
                        "Save Settings"
                    }
                }
            }
        }
    }
}
