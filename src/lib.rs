use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use std::time::Duration;
use std::sync::Arc;

// ─── Data ────────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct Project {
    title: &'static str,
    desc: &'static str,
    long_desc: &'static str,
    tags: &'static [&'static str],
    url: &'static str,
    category: &'static str,
    status: &'static str, // "WIP", "Hiatus", "Finished"
}

fn all_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Project Intleacht Shaorga",
            desc: "A proof-of-concept showcasing how a Large Language Model (LLM), accessed via an API key, can be used as an intelligent data quality assistant.",
            long_desc: "Instead of manually writing repetitive validation and cleaning logic, this tool, accepts structured instructions and a statistics about a pandas dataframe, Builds a prompt from a predefined, constrained skeleton (which also has a helper_reg for additional constraints),
            Sends that prompt to an LLM (GLM-4.7) via an API (Cerebras Inference), Receives suggested transformations and reasonings/justifications, 
            Then also enables the user to send the stats and suggested transformations back to the LLM for additional suggestions
            The goal is not blind automation, but transparent, explainable data tidying.",
            tags: &["ML / AI", "Python"],
            url: "https://github.com/Cisplat1n/Project_Intleacht_Shaorga",
            category: "Python",
            status: "Hiatus"
        },
        Project {
            title: "This Portfolio",
            desc: "Built entirely in Rust + Leptos, compiled to WebAssembly. Zero JavaScript.",
            long_desc: "A portfolio site written entirely in Rust using the Leptos framework, compiled to WebAssembly via Trunk. No JavaScript was written — all interactivity including the filter system, animations, and sprite mascots are driven by Rust compiled to WASM. Deployed automatically to GitHub Pages via a GitHub Actions CI pipeline on every push.",
            tags: &["Rust", "WebAssembly"],
            url: "#",
            category: "Rust",
            status: "Finished"
        },
        Project {
            title: "Project FiligineachtV2-RUST",
            desc: "Discordance resolution pipeline for phylogenetic quartets leveraging PhySquirrel to build consensus networks.",
            long_desc: "A port of my MSc dissertation project (orignally written in python) in Rust, it looks to handle some of the performance and scaling issues flagged during my MSc presentation (file scaling, compute cost, process time etc.). It extracts quartets of taxa from gene trees, roots them to make semi directed where appropriate, and then feeds them into the PhySquirrel python library (via PyO3 bindings) to make a level 1 semi directed consensus network. ",
            tags: &["Rust", "Data Science"],
            url: "https://github.com/Cisplat1n/Project_FiligineachtV2-RUST",
            category: "Rust",
            status: "WIP"
        },
        Project {
            title: "BioClick",
            desc: "A collaborative effort with a University friend to use Electron, Rust and Python to build a GUI, operating system Agnostic, BLAST wrapper that works with the BLAST web API and local installations.",
            long_desc: "This project was made in conjunction with a friend, it aimed to produce a simple and sleek GUI application that could be used to interact with BLAST without command line tools or having to navigate to a web browser. It also seeks to reduce the complexity and confusion around BLAST parameters for new users whilst still allowing power users the flexibility to fine tune their parameters for their needs. ",
            tags: &["Bioinformatics", "Python", "Rust"],
            url: "https://github.com/Cisplat1n/BioClick",
            category: "Collaboration",
            status: "Hiatus"
        },


        Project {
            title: "Project Taispeantas Radharc",
            desc: "A compact showcase demonstrating how to transform raw data into interactive, browser-ready charts using three different programming languages.",
            long_desc: "Each demo operates independently, but all share a common goal. Accept raw tabular data (CSV, JSON, or similar), process and transform the data as needed, generate interactive, web-compatible visualisations, and display results in a browser (or wherever  appropriate).The goal is to provide clear, comparable examples of data visualisation across ecosystems.",
            tags: &["Python", "R", "JavaScript"],
            url: "https://github.com/Cisplat1n/Project_Taispeantas_Radharc",
            category: "Python",
            status: "WIP"

        },

        Project {
            title: "Project Vantage Point",
            desc: "A demonstration of interacting with a SQL database to extract data, transform it and then gain insight. This is done via python.",
            long_desc: "A collaborative demo with a friend where we split the project work in half and work with a database. This is done via a SQL database hosted on SUPABASE. Data will be loaded into the database, then we will extract it, transform it and retrieve insights. This serves as an example scenario of what might happen in a real working team. ",
            tags: &["Python", "SQL",],
            url: "https://github.com/Cisplat1n/Project_Vantage_Point",
            category: "Databases",
            status: "WIP"

        },

        Project {
            title: "Rust Code Notebook",
            desc: "A code notebook I use whilst learning rust.",
            long_desc: "A notebook I am keeping whilst working through the Rust documentation page and then eventually the Rustling exercises.",
            tags: &["Rust", "Notes"],
            url: "https://github.com/Cisplat1n/rust_notes/tree/main",
            category: "Notes",
            status: "WIP"

        }, 

        Project {
            title: "",
            desc: "",
            long_desc: "",
            tags: &["", ""],
            url: "",
            category: "",
            status: ""

        }
    ]
}

const ALL_FILTERS: &[&str] = &[
    "Rust", "ML / AI", "Python", "Data Science", "R", "WebAssembly", "Bioinformatics", "SQL", "Notes"
];

const CATEGORIES: &[(&str, &str)] = &[
    ("Rust", "🦀"),
    ("ML / AI",""),
    ("Python", "🐍"),
    ("Data Science",""),
    ("Collaboration", ""),
    ("Notes",""),
    ("Databases",""),
];

// ─── Sprite State ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
enum SpritePhase {
    Hidden,
    WalkIn,
    Exclamation,
    RunOut,
}

// ─── Entry ───────────────────────────────────────────────────────────────────

#[wasm_bindgen(start)]
pub fn main() {
    // Preload sprite images so they're cached on first use
    let document = web_sys::window().unwrap().document().unwrap();
    for src in &["crab", "snake"] {
        let img = document.create_element("img").unwrap();
        img.set_attribute("src", &format!("/portfolio/public/{}.png", src)).unwrap();
        img.set_attribute("style", "display:none").unwrap();
        document.body().unwrap().append_child(&img).unwrap();
    }

    leptos::mount::mount_to_body(App);
}

// ─── Root ────────────────────────────────────────────────────────────────────

#[component]
pub fn App() -> impl IntoView {
    let (active_tag, set_tag) = signal::<Option<&'static str>>(None);
    let (sprite_trigger, set_sprite_trigger) = signal::<&'static str>("");

    let projects = StoredValue::new(all_projects());

    let filtered = Memo::new(move |_| {
        projects.with_value(|ps| {
            ps.iter()
                .filter(|p| active_tag.get().map_or(true, |t| p.tags.contains(&t)))
                .cloned()
                .collect::<Vec<_>>()
        })
    });

    let on_select = move |tag: Option<&'static str>| {
        set_tag.set(tag);
        match tag {
            Some("Rust")   => set_sprite_trigger.set("crab"),
            Some("Python") => set_sprite_trigger.set("snake"),
            _              => set_sprite_trigger.set(""),
        }
    };

    let (modal_project, set_modal) = signal::<Option<Project>>(None);

    view! {
        <div class="app">
            <Header />
            <main class="main-content">
                <FilterBar active=active_tag on_select=on_select />
                <ProjectGrid projects=filtered set_modal=set_modal />
            </main>
            <SpriteBar trigger=sprite_trigger />
            <Show when=move || modal_project.get().is_some()>
                <ProjectModal
                    project=modal_project.get().unwrap()
                    on_close=move || set_modal.set(None)
                />
            </Show>
        </div>
    }
}

// ─── Header ──────────────────────────────────────────────────────────────────

#[component]
fn Header() -> impl IntoView {
    let (dark, set_dark) = signal(true);

    Effect::new(move |_| {
        let doc = web_sys::window().unwrap().document().unwrap();
        let html = doc.document_element().unwrap();
        if dark.get() {
            html.remove_attribute("data-theme").unwrap();
        } else {
            html.set_attribute("data-theme", "light").unwrap();
        }
    });

    view! {
        <header class="site-header">
            <div class="header-left">
                <h1 class="site-name">"Luke Sal" <span>""</span></h1>
                <p class="site-tagline">"Rust & Python Programmer · Data Scientist"</p>
            </div>
            <button class="theme-toggle" on:click=move |_| set_dark.update(|d| *d = !*d)>
                {move || if dark.get() { "☀ Light" } else { "☾ Dark" }}
            </button>
        </header>
        <div class="header-divider" />
    }
}


// ─── Filter Bar ──────────────────────────────────────────────────────────────

#[component]
fn FilterBar(
    active: ReadSignal<Option<&'static str>>,
    on_select: impl Fn(Option<&'static str>) + Copy + 'static,
) -> impl IntoView {
    view! {
        <div class="filter-section">
            <p class="filter-label">"Filters:"</p>
            <div class="filter-pills">
                {ALL_FILTERS.iter().map(|&tag| {
                    let is_active = move || active.get() == Some(tag);
                    view! {
                        <button
                            class=move || if is_active() { "pill pill-active" } else { "pill" }
                            on:click=move |_| {
                                if active.get() == Some(tag) {
                                    on_select(None);
                                } else {
                                    on_select(Some(tag));
                                }
                            }
                        >
                            {tag}
                            {move || if is_active() {
                                view! { <span class="pill-x">" ×"</span> }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </button>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

// ─── Project Grid ────────────────────────────────────────────────────────────

#[component]
fn ProjectGrid(projects: Memo<Vec<Project>>, set_modal: WriteSignal<Option<Project>>) -> impl IntoView {
    view! {
        <div class="project-grid">
            {CATEGORIES.iter().map(|&(cat, emoji)| {
                let cat_projects = Memo::new(move |_| {
                    projects.get()
                        .into_iter()
                        .filter(|p| p.category == cat)
                        .collect::<Vec<_>>()
                });
                view! {
                    <Show when=move || !cat_projects.get().is_empty()>
                        <div class="category-col">
                            <h2 class="category-title">
                                <span>{emoji}</span>
                                " " {cat}
                            </h2>
                            <div class="card-stack">
                                <For
                                    each=move || cat_projects.get()
                                    key=|p| p.title
                                    children=move |p| view! { <ProjectCard project=p set_modal=set_modal /> }
                                />
                            </div>
                        </div>
                    </Show>
                }
            }).collect_view()}
        </div>
    }
}

// ─── Project Card ────────────────────────────────────────────────────────────

#[component]
fn ProjectCard(project: Project, set_modal: WriteSignal<Option<Project>>) -> impl IntoView {
    let (commits, set_commits) = signal::<Option<String>>(None);
    let (updated, set_updated) = signal::<Option<String>>(None);

    let url = project.url;
    let repo_path = url.trim_start_matches("https://github.com/").to_string();

    if !repo_path.is_empty() && url.contains("github.com") {
        let repo_path_clone = repo_path.clone();
        spawn_local(async move {
            let api_url = format!("https://api.github.com/repos/{}", repo_path_clone);
            if let Ok(resp) = gloo_net::http::Request::get(&api_url)
                .header("Accept", "application/vnd.github.v3+json")
                .send()
                .await
            {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(pushed) = json["pushed_at"].as_str() {
                        let date = pushed.get(..10).unwrap_or(pushed).to_string();
                        set_updated.set(Some(date));
                    }
                }
            }

            let commits_url = format!(
                "https://api.github.com/repos/{}/commits?per_page=1",
                repo_path_clone
            );
            if let Ok(resp) = gloo_net::http::Request::get(&commits_url)
                .header("Accept", "application/vnd.github.v3+json")
                .send()
                .await
            {
                let count = resp
                    .headers()
                    .get("link")
                    .and_then(|link| {
                        link.split(',')
                            .find(|s| s.contains("rel=\"last\""))
                            .and_then(|s| {
                                s.split("&page=").nth(1)
                                    .or_else(|| s.split("?page=").nth(1))
                                    .and_then(|s| s.split('>').next())
                                    .and_then(|n| n.parse::<u32>().ok())
                            })
                    })
                    .unwrap_or(1);
                set_commits.set(Some(count.to_string()));
            }
        });
    }

    let p = project.clone();
    view! {
        <div class="card" on:click=move |_| set_modal.set(Some(p.clone()))>
            <h3 class="card-title">{project.title}</h3>
            <span class=format!("status-badge status-{}", project.status.to_lowercase())>
            {project.status}
            </span>
            <p class="card-desc">{project.desc}</p>
            <div class="card-tags">
                {project.tags.iter().map(|&t| view! {
                    <span class="card-tag">{t}</span>
                }).collect_view()}
            </div>
            <div class="card-stats">
                <span class="stat">
                    "commits: "
                    {move || commits.get().unwrap_or_else(|| "…".to_string())}
                </span>
                <span class="stat">
                    "updated: "
                    {move || updated.get().unwrap_or_else(|| "…".to_string())}
                </span>
            </div>
        </div>
    }
}
// ─── Project Modal ───────────────────────────────────────────────────────────

#[component]
fn ProjectModal(project: Project, on_close: impl Fn() + 'static) -> impl IntoView {
    let close = Arc::new(on_close);
    let close2 = close.clone();
    let url = project.url;
    let has_url = url != "#";
    view! {
        <div class="modal-backdrop" on:click=move |_| close()>
            <div class="modal" on:click=move |e| e.stop_propagation()>
                <button class="modal-close" on:click=move |_| close2()>"×"</button>
                <h2 class="modal-title">{project.title}</h2>
                <div class="modal-tags">
                    {project.tags.iter().map(|&t| view! {
                        <span class="card-tag">{t}</span>
                    }).collect_view()}
                </div>
                <p class="modal-desc">{project.long_desc}</p>
                <Show when=move || has_url>
                    <a href=url class="modal-link" target="_blank" rel="noopener">
                        "View on GitHub →"
                    </a>
                </Show>
            </div>
        </div>
    }
}

// ─── Sprite Bar ──────────────────────────────────────────────────────────────

#[component]
fn SpriteBar(trigger: ReadSignal<&'static str>) -> impl IntoView {
    let (phase, set_phase)     = signal(SpritePhase::Hidden);
    let (pos_x, set_pos_x)     = signal(-150.0f64);
    let (frame, set_frame)     = signal(0u32);
    let (which, set_which)     = signal(String::new());
    let (show_bang, set_bang)  = signal(false);
    let (wait_ticks, set_wait) = signal(0u32);

    // Get window width for responsive sprite stop position
    let win_width = move || {
        web_sys::window()
            .unwrap()
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(1024.0)
    };

    Effect::new(move |_| {
        let t = trigger.get();
        if t.is_empty() { return; }
        set_which.set(t.to_string());
        set_pos_x.set(-150.0);
        set_frame.set(0);
        set_bang.set(false);
        set_wait.set(0);
        set_phase.set(SpritePhase::WalkIn);
    });

    leptos::leptos_dom::helpers::set_interval(
        move || {
            match phase.get() {
                SpritePhase::Hidden => {}
                SpritePhase::WalkIn => {
                    set_frame.update(|f| *f = (*f + 1) % 6);
                    set_pos_x.update(|x| *x += 4.0);
                    // Stop at 40% of screen width so it's always visible
                    let stop = win_width() * 0.4;
                    if pos_x.get() > stop {
                        set_phase.set(SpritePhase::Exclamation);
                        set_bang.set(true);
                        set_frame.set(0);
                        set_wait.set(0);
                    }
                }
                SpritePhase::Exclamation => {
                    set_wait.update(|w| *w += 1);
                    if wait_ticks.get() > 25 {
                        set_bang.set(false);
                        set_phase.set(SpritePhase::RunOut);
                    }
                }
                SpritePhase::RunOut => {
                    set_frame.update(|f| *f = 6 + (*f + 1) % 2);
                    set_pos_x.update(|x| *x -= 8.0);
                    // Hide once fully off the left edge
                    if pos_x.get() < -160.0 {
                        set_phase.set(SpritePhase::Hidden);
                    }
                }
            }
        },
        Duration::from_millis(60),
    );

    let disp: u32 = 64;
    let sheet_w: u32 = 512;
    let sheet_h: u32 = 64;

    let container_style = move || {
        if phase.get() == SpritePhase::Hidden { return "display:none;".to_string(); }
        let x = pos_x.get() as i64;
        format!("display:block;position:fixed;left:{x}px;bottom:0px;width:{disp}px;height:{disp}px;z-index:9999;pointer-events:none;")
    };

    let sprite_style = move || {
        let src = which.get();
        if src.is_empty() { return "display:none;".to_string(); }
        let f = frame.get();
        let off = f * disp;
        let flip = if phase.get() == SpritePhase::RunOut { "transform:scaleX(-1);" } else { "" };
        format!(
            "width:{disp}px;height:{disp}px;background-image:url('/portfolio/public/{src}.png');background-repeat:no-repeat;background-size:{sheet_w}px {sheet_h}px;background-position:-{off}px 0px;image-rendering:pixelated;{flip}"
        )
    };

    view! {
        <div id="sprite-mascot" style=container_style>
            <div style=sprite_style></div>
            <Show when=move || show_bang.get()>
                <div style="position:absolute;top:-38px;left:32px;font-size:40px;\
                            font-weight:900;color:#ffdd44;text-shadow:0 0 12px #ffaa00;\
                            animation:bang-pop 0.35s ease-out;">"!"</div>
            </Show>
        </div>
    }
}