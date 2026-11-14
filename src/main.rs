use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Developer Portfolio" }
        document::Meta {
            name: "description",
            content: "A developer portfolio built with Rust and Dioxus."
        }
        document::Stylesheet { href: asset!("/assets/main.css") }

        div { class: "page-wrap",
            header { class: "site-header",
                a { class: "wordmark", href: "#top", "aria-label": "Back to top",
                    span { class: "wordmark-mark", "~/" }
                    span { "portfolio" }
                }
                nav { class: "site-nav", "aria-label": "Main navigation",
                    a { href: "#work", "Work" }
                    a { href: "#about", "About" }
                    a { class: "nav-contact", href: "#contact", "Contact" }
                }
            }

            main { id: "top",
                section { class: "hero", "aria-label": "Introduction",
                    div { class: "terminal-window",
                        div { class: "terminal-bar",
                            div { class: "window-controls", "aria-hidden": "true",
                                span { class: "control control-close" }
                                span { class: "control control-minimize" }
                                span { class: "control control-expand" }
                            }
                            span { class: "terminal-title", "visitor@portfolio: ~" }
                            span { class: "terminal-mode", "zsh" }
                        }
                        div { class: "terminal-content",
                            p { class: "command-line", span { class: "prompt", "$" } " whoami" }
                            p { class: "eyebrow", "RUST · WEB · THOUGHTFUL SOFTWARE" }
                            h1 { "I build useful things, " span { class: "accent-text", "one commit at a time." } }
                            p { class: "hero-copy",
                                "I’m a developer exploring Rust and the web, with a soft spot for fast, clear, well-crafted software."
                            }
                            div { class: "hero-actions",
                                a { class: "button button-primary", href: "#work", "Explore my work", span { "↓" } }
                                a { class: "button button-quiet", href: "#contact", "Say hello", span { "↗" } }
                            }
                            div { class: "terminal-footer",
                                span { class: "prompt", "visitor@portfolio" }
                                span { ":~$ " }
                                span { class: "typed-command", "open --work" }
                                span { class: "cursor", "aria-hidden": "true" }
                            }
                        }
                    }
                    div { class: "hero-note",
                        span { class: "note-dot" }
                        span { "Made with Rust and Dioxus" }
                        span { class: "note-divider", "/" }
                        span { "Deployed as a static site" }
                    }
                }

                section { id: "work", class: "content-section work-section",
                    div { class: "section-heading",
                        p { class: "section-kicker", "01 / SELECTED WORK" }
                        h2 { "A small collection, " span { class: "muted-heading", "made with care." } }
                        p { class: "section-intro",
                            "I’m building this space around projects that solve real problems. Each one will share the idea, the choices, and what I learned."
                        }
                    }
                    div { class: "project-grid",
                        article { class: "project-card project-featured",
                            div { class: "project-topline",
                                span { class: "project-index", "01" }
                                span { class: "project-label", "THIS WEBSITE" }
                            }
                            div { class: "project-icon project-icon-terminal", "aria-hidden": "true", "$_" }
                            h3 { "A home on the web" }
                            p { "A personal portfolio built with Rust and Dioxus, designed to feel like a familiar terminal and ship as a static site." }
                            div { class: "tag-list",
                                span { "Rust" }
                                span { "Dioxus" }
                                span { "WebAssembly" }
                            }
                        }
                        article { class: "project-card project-next",
                            div { class: "project-topline",
                                span { class: "project-index", "02" }
                                span { class: "project-label", "UP NEXT" }
                            }
                            div { class: "project-icon project-icon-plus", "aria-hidden": "true", "+" }
                            h3 { "The next useful thing" }
                            p { "This space is ready for a project you’re proud of. Add a short story about the problem, your approach, and the result." }
                            a { class: "text-link", href: "#contact", "Have an idea? " span { "↗" } }
                        }
                    }
                }

                section { id: "about", class: "content-section about-section",
                    div { class: "section-heading",
                        p { class: "section-kicker", "02 / A LITTLE ABOUT ME" }
                        h2 { "Curious by nature. " span { class: "muted-heading", "Careful by craft." } }
                    }
                    div { class: "about-grid",
                        p { class: "about-copy",
                            "I enjoy the part where a good idea becomes something clear, reliable, and easy to use. This is my corner of the web for sharing what I build and what I learn along the way."
                        }
                        div { class: "toolbox",
                            p { class: "toolbox-title", "IN MY TOOLBOX" }
                            div { class: "tag-list toolbox-tags",
                                span { "Rust" }
                                span { "Dioxus" }
                                span { "HTML & CSS" }
                                span { "Git" }
                            }
                            p { class: "toolbox-note", "Always learning something new." }
                        }
                    }
                }

                section { id: "contact", class: "contact-section",
                    div { class: "contact-terminal-mark", "aria-hidden": "true", "┌─" }
                    p { class: "section-kicker", "03 / YOUR TURN" }
                    h2 { "Have something good in mind?" }
                    p { class: "contact-copy", "I’d love to hear about it. Replace this starter email with the best way to reach you." }
                    a { class: "button button-primary contact-button", href: "mailto:you@example.com",
                        "you@example.com" span { "↗" }
                    }
                }
            }

            footer { class: "site-footer",
                span { "© 2026 · Built with Rust and a little curiosity." }
                a { href: "#top", "Back to top ↑" }
            }
        }
    }
}
