use axum::Router;
use leptos::prelude::*;
use leptos_axum::LeptosRoutes;
use rpr::app::shell;
use rpr::catalog::Catalog;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    color_backtrace::install();

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let (routes, static_routes) = leptos_axum::generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    static_routes.generate(&leptos_options).await;

    // build our application with a route
    let app = Router::new()
        // .nest_service("/derived", tower_http::services::fs::ServeDir::new("./cache/assets"))
        .leptos_routes(
            &leptos_options, 
            routes,
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            }
        )
        .fallback(leptos_axum::file_and_error_handler(|options: LeptosOptions| view! {
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <AutoReload options=options.clone() />
                    <HydrationScripts options/>
                    <link rel="stylesheet" id="leptos" href="/pkg/rpr.css"/>
                    <link rel="shortcut icon" type="image/x-icon" href="data:image/x-icon;,"/>
                </head>
                <body>
                </body>
            </html>
        }))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
