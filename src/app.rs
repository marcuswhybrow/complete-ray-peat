use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
use leptos_use::{
    breakpoints_tailwind, use_breakpoints, use_element_bounding, use_element_bounding_with_options,
    BreakpointsTailwind as Tailwind,
};

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
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
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

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <FlatRoutes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </FlatRoutes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let sidebar_visible = RwSignal::new(true);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main class="flex">
            <Show when=move||sidebar_visible.get()><Sidebar visible=sidebar_visible /></Show>
            <div class="bg-slate-200 flex-auto">
                <button on:click=move|_|sidebar_visible.set(!sidebar_visible.get())>Toggle</button>
            </div>
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
                on:click=move|_| {
                    console_log(&format!("Active: {}", &name_string));
                    selected.set(Some(name_string.clone()))
                }
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
fn Sidebar(visible: RwSignal<bool>) -> impl IntoView {
    let screen_width = use_breakpoints(breakpoints_tailwind());
    let gt_large = screen_width.gt(Tailwind::Lg);
    let tab_selected = RwSignal::new(None);
    let tab_current = RwSignal::new(vec!["Filters".to_owned()]);

    Effect::new(move || {
        tab_selected.set(Some("Filters".to_owned()));
    });

    view! {
        <div class="w-64">
            <div class="p-2 flex flex-col gap-2">
                <input
                    type="search"
                    placeholder="search"
                    class="border border-solid border-slate-200 w-full rounded px-2 py-1"
                />
                <SidebarTabs selected=tab_selected current=tab_current />
            </div>
            {move || if gt_large.get() {
                view! { <p>Large or more</p> }
            } else {
                view! { <p>Less than large</p> }
            }}
            <button on:click=move|_|visible.set(false)>Hide</button>
        </div>
    }
}
