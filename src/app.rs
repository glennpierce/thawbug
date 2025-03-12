
use thaw::{Theme, Spinner};

use gloo_storage::{LocalStorage, Storage};

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::StaticSegment;
use leptos_router::components::ParentRoute;
use leptos_router::hooks::{use_navigate, use_location};
use leptos_router::nested_router::Outlet;
use leptos_router::components::{Router, Routes, Route};

use crate::home::ReportCreation;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/comboboxtest.css"/>
                // <link rel="stylesheet" id="leptos" href="/pkg/leptos_tailwind.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <script src="https://cdn.plot.ly/plotly-2.14.0.min.js"></script>
                // <MetaTags/>
                <title>Redwood</title>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {

    #[cfg(feature = "ssr")]
    log::info!("SSR Mode");

    #[cfg(feature = "csr")]
    log::info!("CSR Mode");

    #[cfg(feature = "hydrate")]
    log::info!("Hydrate Mode");


    view! {

        <Router>

            <main>

                <Routes fallback=|| {
                    view! { }.into_view()
                }>

                     
                    <Route
                        path=StaticSegment("")

                        view=move || {
                            view! {
                                <ReportCreation
                                   
                                />
                            }
                        }
                    />

                </Routes>

            </main>
        </Router>
    }
}