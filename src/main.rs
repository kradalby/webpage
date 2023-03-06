use axum::{extract::State, routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use serde::Serialize;
use std::net::SocketAddr;

use kradalby::Salary;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let salaries: Vec<Salary> = serde_dhall::from_file("./dhall/salaries.dhall").parse()?;

    // let files = SpaRouter::new("/static/css", env!("XESS_PATH"));

    let app = Router::new()
        .route("/", get(handler))
        .route("/posts", get(handler_posts))
        .route("/salary", get(handler_salary))
        .route("/about", get(handler_about))
        .with_state(salaries);
    // .merge(files)

    // run it
    let port = std::env::var("PORT")
        .ok()
        .and_then(|port_str| port_str.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handler() -> Markup {
    base(
        Some("Kristoffer Dalby"),
        html! {
            p {
                "Software engineer at "
                a href="https://tailscale.com" {"Tailscale"}
                " and maintainer of "
                a href="https://github.com/juanfont/headscale" {"Headscale"} "."
            }
            p {
                "Travelling, bouldering, cooking, photography"
            }

            a href="https://github.com/kradalby" { (github(Some(48))) }
            a href="https://linkedin.com/in/kradalby" { (linkedin(Some(48)))}
            a href="https://discordapp.com/users/134699346774982656" { (discord(Some(48))) }
            a href="https://instagram.com/kradalby" { (instagram(Some(48))) }
            a href="https://twitter.com/kradalby" { (twitter(Some(48))) }
        },
    )
}

async fn handler_posts() -> Markup {
    base(Some("Posts"), html! {})
}

async fn handler_salary(State(salaries): State<Vec<Salary>>) -> Markup {
    base(
        Some("Salary transparency"),
        html! {
            p {
                "I am a big supporter of salary transparency and this page lists the salaries I have had in my professional work life. Sharing salaries helps giving the negotiation power back to employees allowing them to ask for what they are worth when negotiating with their employer."
            }

            p {
                "Pay discrimination is a huge problem and different gaps are continuously growing larger. Publicly available data is one of the ways we can empower people to close these gaps by knowing their worth when they come to the negotiation table. People seem to often come to the table with an expectation, that is lower than their actually worth, based on things like lack of salary transparency, imposter syndrom and stigmatising talking and asking for salary."

            }

            p {
                "Companies will rarely and gladly offer a fair salary based on an employees worth based on their peers if they do not ask for it."
            }

            h2 {
                "Ethos"
            }

            p {
                "I believe that the only entity that benefits from hidden salaries are employers/companies. Not employees."
            }

            h2 {
                "Data"
            }

            p {
                "This data is compiled from contracts, payslips and memory, it might not be 100% accurate, but sufficient to show the general idea. These salaries are spread across multiple countries, currencies and different pay models (fixed/bonus/equity) and might not show an easy to compare picture."
            }

            p {
                "Company information has intentionally been omitted and if there were information that would have been sensitive, it has been removed."
            }

            table {
                thead {
                    tr {
                        th {
                            "Title"
                        }
                        th {
                            "Start date"
                        }
                        th {
                            "End date"
                        }
                        th {
                            "How I left"
                        }
                        th {
                            "Salary"
                        }
                        th {
                            "Note"
                        }
                    }
                }
                tbody {
                    @for salary in &salaries {
                        tr {
                            td {
                                (salary.title)
                            }
                            td {
                                (salary.start_date)
                            }
                            td {
                                (salary.end_date)
                            }
                            td {
                                (salary.how_i_left)
                            }
                            td {
                                (salary.salary)
                            }
                            td {
                                (salary.note)
                            }
                        }
                    }
                }
            }

            p {
                "I aim to update this when typically change events happens like:"
            }

            ul {
                li {"I leave a job"}
                li {"My title changes"}
                li {"I get a raise"}
                li {"I am fired"}
            }

            h2 {
                "Contributing"
            }

            p {
                "The best way to help out is to join in and publish your salary information, that way more people get more knowledge and we can remove even more of the stigma related to salaries and make sure people are treated fairly."
            }

        },
    )
}

async fn handler_about() -> Markup {
    base(
        Some("About me"),
        html! {
            h3 { "Hi, I am Kristoffer"}
            p { "I am Software engineer from Norway, currently living in Leiden, The Netherlands and slowly travelling the world." }
            p {
                "I work as a Member of Techinal Staff at "
                a href="https://tailscale.com" { "Tailscale" }
                " where I work on both Tailscale and "
                a href="https://github.com/juanfont/headscale" { "Headscale" }
                ", an open source, self-hosted version of the Tailscale control server."
            }
            p {
                "I have a Master in Informatics from "
                a href="https://www.ntnu.edu" { "Norwegian University of Science and Technology" }
                ". After finishing my master, I moved abroad to participate in a two year Young Graduate Traine program at the "
                a href="https://esa.int" { "European Space Agency" }
                "."
            }
            p {
                "Outside of work, I am a passionate traveller, amateur boulderer and sometimes I dabbles in photography and cooking."
            }

            img src="https://gravatar.com/avatar/e77f306a388b9463ad2310c8f041d42a?s=400&d=robohash&r=x" {}

            h2 { "Public speaking" }
            p { "I have done some public speaking, here is a list of publicly available talks:" }

            ul {
                li { a href="https://fosdem.org/2023/schedule/event/goheadscale/" {
                    "Headscale: How we are using integration testing to reimplement Tailscale [FOSDEM 2023, Brussels]"
                    }
                }
            }
        },
    )
}

fn base(title: Option<&str>, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                @if let Some(title) = title {
                    title { (format!("{title} - kradalby.no")) }
                } @else {
                    title { "kradalby.no" }
                }
                link rel="stylesheet" href="/static/css/kradalby.css";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
            }

            body #top {
                nav {
                    a href="/" {"home"}
                    " - "
                    // a href="/posts" {"posts"}
                    // " - "
                    // a href="/salary" {"salary transparency"}
                    // " - "
                    a href="/about" {"about me"}
                    " - "
                    a href="https://resume.kradalby.no" {"resume ↗"}
                }
                main {
                    header {
                        @if let Some(title) = title {
                            h1 { (title) }
                        } @else {
                            h1 { "kradalby.no" }
                        }
                    }

                    (body)
                }
                footer {
                    p {
                        "Copyright © 2023 - Kristoffer Dalby"
                    }
                }
            }
        }
    }
}

// from https://icons8.com/icon/set/logos/material
fn instagram(size: Option<i32>) -> Markup {
    let w = Some(size.unwrap_or(480).to_string());
    let h = Some(size.unwrap_or(480).to_string());

    html! {
        svg height=[h] y="0px" width=[w] viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" x="0px" {
            path d="M 8 3 C 5.239 3 3 5.239 3 8 L 3 16 C 3 18.761 5.239 21 8 21 L 16 21 C 18.761 21 21 18.761 21 16 L 21 8 C 21 5.239 18.761 3 16 3 L 8 3 z M 18 5 C 18.552 5 19 5.448 19 6 C 19 6.552 18.552 7 18 7 C 17.448 7 17 6.552 17 6 C 17 5.448 17.448 5 18 5 z M 12 7 C 14.761 7 17 9.239 17 12 C 17 14.761 14.761 17 12 17 C 9.239 17 7 14.761 7 12 C 7 9.239 9.239 7 12 7 z M 12 9 A 3 3 0 0 0 9 12 A 3 3 0 0 0 12 15 A 3 3 0 0 0 15 12 A 3 3 0 0 0 12 9 z" {}
        }
    }
}

fn linkedin(size: Option<i32>) -> Markup {
    let w = Some(size.unwrap_or(480).to_string());
    let h = Some(size.unwrap_or(480).to_string());

    html! {
        svg width=[w] xmlns="http://www.w3.org/2000/svg" x="0px" height=[h] viewBox="0 0 24 24" y="0px" {
            path d="M19,3H5C3.895,3,3,3.895,3,5v14c0,1.105,0.895,2,2,2h14c1.105,0,2-0.895,2-2V5C21,3.895,20.105,3,19,3z M9,17H6.477v-7H9 V17z M7.694,8.717c-0.771,0-1.286-0.514-1.286-1.2s0.514-1.2,1.371-1.2c0.771,0,1.286,0.514,1.286,1.2S8.551,8.717,7.694,8.717z M18,17h-2.442v-3.826c0-1.058-0.651-1.302-0.895-1.302s-1.058,0.163-1.058,1.302c0,0.163,0,3.826,0,3.826h-2.523v-7h2.523v0.977 C13.93,10.407,14.581,10,15.802,10C17.023,10,18,10.977,18,13.174V17z" {}
        }
    }
}

fn github(size: Option<i32>) -> Markup {
    let w = Some(size.unwrap_or(480).to_string());
    let h = Some(size.unwrap_or(480).to_string());

    html! {
        svg width=[w] height=[h] viewBox="0 0 24 24" x="0px" xmlns="http://www.w3.org/2000/svg" y="0px" {
            path d="M 12 2 C 6.476563 2 2 6.476563 2 12 C 2 17.523438 6.476563 22 12 22 C 17.523438 22 22 17.523438 22 12 C 22 6.476563 17.523438 2 12 2 Z M 12 4 C 16.410156 4 20 7.589844 20 12 C 20 12.46875 19.953125 12.929688 19.875 13.375 C 19.628906 13.320313 19.265625 13.253906 18.84375 13.25 C 18.53125 13.246094 18.140625 13.296875 17.8125 13.34375 C 17.925781 12.996094 18 12.613281 18 12.21875 C 18 11.257813 17.53125 10.363281 16.78125 9.625 C 16.988281 8.855469 17.191406 7.535156 16.65625 7 C 15.074219 7 14.199219 8.128906 14.15625 8.1875 C 13.667969 8.070313 13.164063 8 12.625 8 C 11.933594 8 11.273438 8.125 10.65625 8.3125 L 10.84375 8.15625 C 10.84375 8.15625 9.964844 6.9375 8.34375 6.9375 C 7.777344 7.507813 8.035156 8.953125 8.25 9.6875 C 7.484375 10.417969 7 11.28125 7 12.21875 C 7 12.546875 7.078125 12.859375 7.15625 13.15625 C 6.878906 13.125 5.878906 13.03125 5.46875 13.03125 C 5.105469 13.03125 4.542969 13.117188 4.09375 13.21875 C 4.03125 12.820313 4 12.414063 4 12 C 4 7.589844 7.589844 4 12 4 Z M 5.46875 13.28125 C 5.863281 13.28125 7.0625 13.421875 7.21875 13.4375 C 7.238281 13.492188 7.257813 13.542969 7.28125 13.59375 C 6.851563 13.554688 6.019531 13.496094 5.46875 13.5625 C 5.101563 13.605469 4.632813 13.738281 4.21875 13.84375 C 4.1875 13.71875 4.148438 13.597656 4.125 13.46875 C 4.5625 13.375 5.136719 13.28125 5.46875 13.28125 Z M 18.84375 13.5 C 19.242188 13.503906 19.605469 13.570313 19.84375 13.625 C 19.832031 13.691406 19.796875 13.746094 19.78125 13.8125 C 19.527344 13.753906 19.109375 13.667969 18.625 13.65625 C 18.390625 13.652344 18.015625 13.664063 17.6875 13.6875 C 17.703125 13.65625 17.707031 13.625 17.71875 13.59375 C 18.058594 13.546875 18.492188 13.496094 18.84375 13.5 Z M 6.09375 13.78125 C 6.65625 13.785156 7.183594 13.824219 7.40625 13.84375 C 7.929688 14.820313 8.988281 15.542969 10.625 15.84375 C 10.222656 16.066406 9.863281 16.378906 9.59375 16.75 C 9.359375 16.769531 9.113281 16.78125 8.875 16.78125 C 8.179688 16.78125 7.746094 16.160156 7.375 15.625 C 7 15.089844 6.539063 15.03125 6.28125 15 C 6.019531 14.96875 5.929688 15.117188 6.0625 15.21875 C 6.824219 15.804688 7.097656 16.5 7.40625 17.125 C 7.683594 17.6875 8.265625 18 8.90625 18 L 9.03125 18 C 9.011719 18.109375 9 18.210938 9 18.3125 L 9 19.40625 C 6.691406 18.472656 4.933594 16.5 4.28125 14.0625 C 4.691406 13.960938 5.152344 13.855469 5.5 13.8125 C 5.660156 13.792969 5.863281 13.777344 6.09375 13.78125 Z M 18.625 13.90625 C 19.074219 13.917969 19.472656 14.003906 19.71875 14.0625 C 19.167969 16.132813 17.808594 17.855469 16 18.90625 L 16 18.3125 C 16 17.460938 15.328125 16.367188 14.375 15.84375 C 15.957031 15.554688 16.988281 14.863281 17.53125 13.9375 C 17.910156 13.910156 18.355469 13.898438 18.625 13.90625 Z M 12.5 18 C 12.773438 18 13 18.226563 13 18.5 L 13 19.9375 C 12.671875 19.980469 12.339844 20 12 20 L 12 18.5 C 12 18.226563 12.226563 18 12.5 18 Z M 10.5 19 C 10.773438 19 11 19.226563 11 19.5 L 11 19.9375 C 10.664063 19.894531 10.324219 19.832031 10 19.75 L 10 19.5 C 10 19.226563 10.226563 19 10.5 19 Z M 14.5 19 C 14.742188 19 14.953125 19.175781 15 19.40625 C 14.675781 19.539063 14.34375 19.660156 14 19.75 L 14 19.5 C 14 19.226563 14.226563 19 14.5 19 Z" {}
        }
    }
}

fn discord(size: Option<i32>) -> Markup {
    let w = Some(size.unwrap_or(480).to_string());
    let h = Some(size.unwrap_or(480).to_string());

    html! {
        svg x="0px" xmlns="http://www.w3.org/2000/svg" width=[w] height=[h] viewBox="0 0 24 24" y="0px" {
            path d="M19.952,5.672c-1.904-1.531-4.916-1.79-5.044-1.801c-0.201-0.017-0.392,0.097-0.474,0.281 c-0.006,0.012-0.072,0.163-0.145,0.398c1.259,0.212,2.806,0.64,4.206,1.509c0.224,0.139,0.293,0.434,0.154,0.659 c-0.09,0.146-0.247,0.226-0.407,0.226c-0.086,0-0.173-0.023-0.252-0.072C15.584,5.38,12.578,5.305,12,5.305S8.415,5.38,6.011,6.872 c-0.225,0.14-0.519,0.07-0.659-0.154c-0.14-0.225-0.07-0.519,0.154-0.659c1.4-0.868,2.946-1.297,4.206-1.509 c-0.074-0.236-0.14-0.386-0.145-0.398C9.484,3.968,9.294,3.852,9.092,3.872c-0.127,0.01-3.139,0.269-5.069,1.822 C3.015,6.625,1,12.073,1,16.783c0,0.083,0.022,0.165,0.063,0.237c1.391,2.443,5.185,3.083,6.05,3.111c0.005,0,0.01,0,0.015,0 c0.153,0,0.297-0.073,0.387-0.197l0.875-1.202c-2.359-0.61-3.564-1.645-3.634-1.706c-0.198-0.175-0.217-0.477-0.042-0.675 c0.175-0.198,0.476-0.217,0.674-0.043c0.029,0.026,2.248,1.909,6.612,1.909c4.372,0,6.591-1.891,6.613-1.91 c0.198-0.172,0.5-0.154,0.674,0.045c0.174,0.198,0.155,0.499-0.042,0.673c-0.07,0.062-1.275,1.096-3.634,1.706l0.875,1.202 c0.09,0.124,0.234,0.197,0.387,0.197c0.005,0,0.01,0,0.015,0c0.865-0.027,4.659-0.667,6.05-3.111 C22.978,16.947,23,16.866,23,16.783C23,12.073,20.985,6.625,19.952,5.672z M8.891,14.87c-0.924,0-1.674-0.857-1.674-1.913 s0.749-1.913,1.674-1.913s1.674,0.857,1.674,1.913S9.816,14.87,8.891,14.87z M15.109,14.87c-0.924,0-1.674-0.857-1.674-1.913 s0.749-1.913,1.674-1.913c0.924,0,1.674,0.857,1.674,1.913S16.033,14.87,15.109,14.87z" {}
        }
    }
}

fn twitter(size: Option<i32>) -> Markup {
    let w = Some(size.unwrap_or(480).to_string());
    let h = Some(size.unwrap_or(480).to_string());

    html! {
        svg xmlns="http://www.w3.org/2000/svg" y="0px" x="0px" width=[w] height=[h] viewBox="0 0 24 24" {
            path d="M22,3.999c-0.78,0.463-2.345,1.094-3.265,1.276c-0.027,0.007-0.049,0.016-0.075,0.023c-0.813-0.802-1.927-1.299-3.16-1.299 c-2.485,0-4.5,2.015-4.5,4.5c0,0.131-0.011,0.372,0,0.5c-3.353,0-5.905-1.756-7.735-4c-0.199,0.5-0.286,1.29-0.286,2.032 c0,1.401,1.095,2.777,2.8,3.63c-0.314,0.081-0.66,0.139-1.02,0.139c-0.581,0-1.196-0.153-1.759-0.617c0,0.017,0,0.033,0,0.051 c0,1.958,2.078,3.291,3.926,3.662c-0.375,0.221-1.131,0.243-1.5,0.243c-0.26,0-1.18-0.119-1.426-0.165 c0.514,1.605,2.368,2.507,4.135,2.539c-1.382,1.084-2.341,1.486-5.171,1.486H2C3.788,19.145,6.065,20,8.347,20 C15.777,20,20,14.337,20,8.999c0-0.086-0.002-0.266-0.005-0.447C19.995,8.534,20,8.517,20,8.499c0-0.027-0.008-0.053-0.008-0.08 c-0.003-0.136-0.006-0.263-0.009-0.329c0.79-0.57,1.475-1.281,2.017-2.091c-0.725,0.322-1.503,0.538-2.32,0.636 C20.514,6.135,21.699,4.943,22,3.999z" {}
        }
    }
}
