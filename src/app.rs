use anyhow::Result;
use core::panic;
use futures::{channel::mpsc, Stream};
use leptos::{
    html::span,
    leptos_dom::logging::{console_error, console_log},
    prelude::*,
    tachys,
};
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    hooks::use_params,
    params::Params,
    path,
    static_routes::StaticRoute,
    SsrMode,
};
use leptos_use::{
    breakpoints_tailwind, use_breakpoints, use_element_bounding, BreakpointsTailwind as Tailwind,
};
use serde::{Deserialize, Serialize};
use std::{path::Path, time::Duration};
use thiserror::Error;

use crate::catalog::{self, Catalog};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/rpr.css"/>
                <link rel="shortcut icon" type="image/x-icon" href="data:image/x-icon;,"/>
                <MetaTags/>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// Add Tailwind classes here that aren't being picked up
#[component]
fn Dummy() -> impl IntoView {
    view! {
        <div class="font-bold"></div>
    }
}

#[server]
pub async fn asset_slugs() -> Result<Vec<String>, ServerFnError> {
    let entries = std::fs::read_dir("./assets")?;
    let slugs = entries
        .into_iter()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if !path.is_file() || path.extension()? != "md" {
                return None;
            }
            let stem = path.file_stem()?.to_string_lossy().to_string();
            let slug = &stem["0000-00-00-".len()..];
            Some(slug.to_owned())
        })
        .collect();

    Ok(slugs)
}

#[allow(unused)]
fn watch_path(path: &Path) -> impl Stream<Item = ()> {
    #[allow(unused)]
    let (mut tx, rx) = mpsc::channel(0);

    #[cfg(feature = "ssr")]
    {
        use notify::RecursiveMode;
        use notify::Watcher;

        let event_handler = move |res: Result<_, _>| {
            // let x = res.unwrap();
            if res.is_ok() {
                _ = tx.try_send(());
            }
        };

        let config = notify::Config::default().with_poll_interval(Duration::from_secs(1));

        // The notify crate also has an inotify watcher, but this runs into
        // limitations for open file descriptors, especially when using
        // Windows Subsystem for Linux.
        let mut watcher =
            notify::PollWatcher::new(event_handler, config).expect("Failed to create watcher");

        watcher
            .watch(Path::new("./assets"), RecursiveMode::Recursive)
            .expect("Failed to watch path of asset");

        std::mem::forget(watcher);
    }
    rx
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let sidebar_visible = RwSignal::new(true);

    let catalog = RwSignal::new(None);
    let search = RwSignal::new(String::default());

    let get_catalog = Action::new(move |_: &()| async move {
        let c = get_catalog().await.expect("Failed to retrieve catalog");
        catalog.set(Some(c));
    });
    get_catalog.dispatch(());

    provide_context(catalog.read_only());

    view! {
        <h1><a href="/">Ray Peat Rodeo</a></h1>
        <div class="flex">
            <Show when=move||sidebar_visible.get()><Sidebar visible=sidebar_visible search /></Show>
            <div class="flex-auto">
                <button on:click=move|_|sidebar_visible.set(!sidebar_visible.get())>Toggle</button>
                <Router>
                    <FlatRoutes fallback=||"Page not found.">
                        <Route
                            path=path!("/")
                            view=Home
                            // ssr=SsrMode::Static(
                            //     StaticRoute::new()
                            //         .regenerate(|_| watch_path(Path::new("./assets")))
                            // )
                        />
                        <Route
                            path=path!("/about")
                            view=About
                            // ssr=SsrMode::Static(StaticRoute::new())
                        />
                        <Route
                            path=path!("/:slug")
                            view=Asset
                            // ssr=SsrMode::Static(
                            //     StaticRoute::new()
                            //         .prerender_params(|| async move {
                            //             let catalog: &'static Catalog = expect_context();
                            //             let slugs: Vec<String> = catalog
                            //                 .assets
                            //                 .values()
                            //                 .map(|asset| asset.slug.clone())
                            //                 .collect();
                            //             [("slug".into(), slugs)]
                            //                 .into_iter()
                            //                 .collect()
                            //         })
                            //         .regenerate(|params| {
                            //             let catalog: &'static Catalog = expect_context();
                            //             let slug = params.get("slug").unwrap();
                            //             let asset = catalog.assets.get(&slug).unwrap();
                            //             watch_path(&asset.path)
                            //         })
                            // )
                        />
                    </FlatRoutes>
                </Router>
            </div>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h1>About</h1>
    }
}

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct AssetParams {
    slug: Option<String>,
}

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetError {
    #[error("Invalid asset slug.")]
    InvalidSlug,

    #[error("Asset not found.")]
    NotFound,

    #[error("Server error: {0}.")]
    ServerError(String),
}

#[server]
async fn get_catalog() -> Result<Catalog, ServerFnError> {
    println!("Creating asset catalog");
    let catalog = Catalog::new(&Path::new("./assets"))
        .await
        .expect("Failed to build catalog");
    Ok(catalog)
}

#[component]
fn Asset() -> impl IntoView {
    let params = use_params::<AssetParams>();

    let catalog: ReadSignal<Option<Catalog>> = expect_context();

    let asset = Signal::derive(move || {
        let Some(catalog) = catalog.get() else {
            return None;
        };

        let slug = params
            .get()
            .expect("Failed to determine URL params")
            .slug
            .expect("Failed to determine slug from URL params");
        let Some(asset) = catalog.assets.get(&slug) else {
            console_error(&format!("Asset {slug} not found in catalog"));
            return None;
        };
        Some(asset.clone())
    });

    // Effect::new(move || {
    //     let slug = params
    //         .get()
    //         .expect("Failed to find URL params")
    //         .slug
    //         .expect("Failed to find slug");

    //     let opts = RequestInit::new();
    //     opts.set_method("GET");
    //     opts.set_mode(RequestMode::Cors);
    //     let url = format!("/derived/{}.yml", slug);

    //     let request =
    //         Request::new_with_str_and_init(&url, &opts).expect("Failed to create request");
    //     request
    //         .headers()
    //         .set("Accept", "text/yaml")
    //         .expect("Failed to set headers");

    //     let window = web_sys::window().expect("Failed to unwrap window");
    //     let response_future = window.fetch_with_request(&request);

    //     let _ =
    //         wasm_bindgen_futures::JsFuture::from(response_future).and_then(|js_value| async move {
    //             let str = js_value.as_string().expect("Response was not a string");
    //             let a: crate::asset::Asset =
    //                 serde_yaml::from_str(&str).expect("Failed to deserialize asset");
    //             asset.set(Some(a));
    //             Ok(())
    //         });
    // });

    view! {
        <Suspense fallback=move|| view! { <p>Loading post...</p> }>
            <div class="p-16">
                <h1 class="text-4xl font-bold tracking-tight mb-8">{ move||asset.get().map(|a| a.to_title()) }</h1>
                { move || {
                    if let Some(asset) = asset.get() {
                        asset.elements.into_view().into_any()
                    } else {
                        span().child("No view").into_any()
                    }
                }}
            </div>
        </Suspense>
    }
}

#[component]
fn Home() -> impl IntoView {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Ray Peat Rodeo"/>
        <main>
            <div>Home</div>
        </main>
    }
}

#[component]
fn SidebarTab<'a>(
    name: &'a str,
    selected: RwSignal<Option<String>>,
    current: RwSignal<Vec<String>>,
    node_ref: NodeRef<leptos::html::A>,
) -> impl IntoView + use<'a> {
    let name_string = name.to_owned();
    let is_selected = Signal::derive({
        let name_string = name_string.clone();
        move || selected.get().is_some_and(|n| n == name_string)
    });
    let is_current = Signal::derive({
        let name_string = name_string.clone();
        move || current.get().contains(&name_string)
    });
    view! {
        <li
            class="flex-grow z-10"
            aria-selected=move||if is_selected.get() { Some("true") } else { None }
            aria-current=move||if is_current.get() { Some("true") } else { None }
        >
            <a
                node_ref=node_ref
                href=""
                class="w-full grid content-center justify-center rounded text-slate-800 text-md py-0.5"
                on:click=move|_| selected.set(Some(name_string.clone()))
            >{ name }</a>
        </li>
    }
}

#[component]
fn SidebarTabs(
    selected: RwSignal<Option<String>>,
    current: RwSignal<Vec<String>>,
) -> impl IntoView {
    let names = ["Filters", "List", "Table"];
    let names: Vec<(&str, NodeRef<leptos::html::A>)> = names
        .into_iter()
        .map(|name| (name, NodeRef::new()))
        .collect();
    let selected_ref = Signal::derive({
        let names = names.clone();
        move || {
            names
                .iter()
                .find(|name| selected.get().is_some_and(|n| n == name.0))
                .map(|x| x.1)
        }
    });
    let parent_ref = NodeRef::new();
    let highlight_inset = Signal::derive(move || match selected_ref.get() {
        Some(selected_ref) => {
            let parent = use_element_bounding(parent_ref);
            let selected = use_element_bounding(selected_ref);
            let top = selected.top.get() - parent.top.get();
            let right = parent.right.get() - selected.right.get();
            let bottom = parent.bottom.get() - selected.bottom.get();
            let left = selected.left.get() - parent.left.get();
            format!("{}px {}px {}px {}px", top, right, bottom, left)
        }
        None => "0px".to_owned(),
    });

    view! {
        <ul class="rounded p-2 bg-slate-100 flex gap-2 relative" node_ref=parent_ref>
            <For each=move||names.clone() key=|name|name.0.to_owned() let:name>
                <SidebarTab name=name.0 node_ref=name.1 selected current />
            </For>
            <Show when=move||selected_ref.get().is_some()>
                <div
                    class="bg-white absolute z-5 rounded transition-all"
                    style:inset=move||highlight_inset.get()
                ></div>
            </Show>
        </ul>
    }
}

#[component]
fn Sidebar(visible: RwSignal<bool>, search: RwSignal<String>) -> impl IntoView {
    let screen_width = use_breakpoints(breakpoints_tailwind());
    let gt_large = screen_width.gt(Tailwind::Lg);
    let tab_selected = RwSignal::new(None);
    let tab_current = RwSignal::new(vec!["Filters".to_owned()]);

    Effect::new(move || {
        tab_selected.set(Some("Filters".to_owned()));
    });

    let catalog: ReadSignal<Option<Catalog>> = expect_context();

    let assets = Signal::derive(move || {
        let Some(catalog) = catalog.get() else {
            return vec![];
        };
        catalog
            .assets
            .values()
            .filter(|asset| {
                asset
                    .to_title()
                    .to_lowercase()
                    .contains(&search.get().trim().to_lowercase())
            })
            .cloned()
            .collect::<Vec<crate::asset::Asset>>()
    });

    view! {
        <div class="w-64 shrink-0">
            <div class="p-2 flex flex-col gap-2">
                <input
                    type="search"
                    placeholder="search"
                    class="border border-solid border-slate-200 w-full rounded px-2 py-1"
                    prop:value=move||search.get()
                    on:keyup=move|event| {
                        let value = event_target_value(&event);
                        search.set(value);
                    }
                />
                <SidebarTabs selected=tab_selected current=tab_current />
            </div>
            {move || if gt_large.get() {
                view! { <p>Large or more</p> }
            } else {
                view! { <p>Less than large</p> }
            }}
            <button on:click=move|_|visible.set(false)>Hide</button>
            <Suspense>
                <ul class="p-8 flex flex-col gap-4">
                    <For
                        each=move||assets.get()
                        key=|asset| asset.slug.clone()
                        let:asset
                    >
                        <li>
                            <a
                                href=format!("/{}", asset.slug)
                                class="hover:underline"
                            >{asset.to_title()}</a>
                        </li>
                    </For>
                </ul>
            </Suspense>
        </div>
    }
}
