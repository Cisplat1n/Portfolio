use leptos::prelude::*;
use wasm_bindgen::prelude::*;
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
}

fn all_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Inference Cluster",
            desc: "Distributed llama.cpp inference across BC-250 + MI50 over 10GbE RPC.",
            long_desc: "A homelab distributed inference cluster built from budget GPU hardware — an ASRock BC-250 (16GB GDDR6) and AMD MI50 (32GB HBM2) connected over 10 Gigabit Ethernet. Uses llama.cpp's RPC backend to split model layers across both nodes, enabling larger models than either card could run alone. Managed via Ollama with Open WebUI as the frontend. The whole setup runs on ROCm and cost under $400 in hardware.",
            tags: &["Rust", "Hardware"],
            url: "#",
            category: "Rust",
        },
        Project {
            title: "This Portfolio",
            desc: "Built entirely in Rust + Leptos, compiled to WebAssembly. Zero JavaScript.",
            long_desc: "A portfolio site written entirely in Rust using the Leptos framework, compiled to WebAssembly via Trunk. No JavaScript was written — all interactivity including the filter system, animations, and sprite mascots are driven by Rust compiled to WASM. Deployed automatically to GitHub Pages via a GitHub Actions CI pipeline on every push.",
            tags: &["Rust", "WebAssembly"],
            url: "#",
            category: "Rust",
        },
        Project {
            title: "LLM Data Pipeline",
            desc: "High-throughput data pipeline for LLM training dataset curation and filtering.",
            long_desc: "A scalable data pipeline for curating and filtering large-scale text datasets used in LLM pretraining and fine-tuning. Handles deduplication, quality filtering, language detection, and formatting into standardised training formats. Built to process hundreds of gigabytes efficiently with parallelised Python workers and streaming I/O.",
            tags: &["ML / AI", "Python"],
            url: "#",
            category: "ML / AI",
        },
        Project {
            title: "Model Evaluation Harness",
            desc: "Automated evaluation framework for benchmarking inference quality and latency.",
            long_desc: "An automated evaluation framework for systematically benchmarking LLM quality and performance. Supports multiple model backends, configurable benchmark suites, and produces structured reports comparing accuracy, latency, and throughput across model versions. Designed to integrate into CI pipelines to catch regressions before deployment.",
            tags: &["ML / AI", "Python"],
            url: "#",
            category: "ML / AI",
        },
        Project {
            title: "Quartet Extraction Engine",
            desc: "Music analysis tool for extracting instrument parts from quartet recordings.",
            long_desc: "A signal processing tool for separating individual instrument parts from mixed string quartet recordings. Uses source separation techniques combined with pitch tracking and onset detection to isolate violin, viola, and cello lines. Outputs per-instrument audio and MIDI transcriptions for further analysis or practice use.",
            tags: &["Python", "Data Science"],
            url: "#",
            category: "Data Science",
        },
        Project {
            title: "Statistical Modelling Suite",
            desc: "R-based Bayesian modelling toolkit for time-series and regression analysis.",
            long_desc: "A collection of Bayesian statistical models built in R using Stan and brms for time-series forecasting and regression analysis. Includes hierarchical models, changepoint detection, and posterior predictive checks. Designed for reproducible research with fully documented model specifications and diagnostic plots.",
            tags: &["R", "Data Science"],
            url: "#",
            category: "Data Science",
        },
    ]
}

const ALL_FILTERS: &[&str] = &[
    "Rust", "ML / AI", "Python", "Data Science", "R", "WebAssembly",
];

const CATEGORIES: &[(&str, &str)] = &[
    ("Rust", "🦀"),
    ("ML / AI",""),
    ("Python", "🐍"),
    ("Data Science",""),
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
    view! {
        <header class="site-header">
            <div class="header-left">
                <h1 class="site-name">"Luke Sal" <span>""</span></h1>
                <p class="site-tagline">"Rust & Python Progammer · Data Scientist "</p>
            </div>

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
    let p = project.clone();
    view! {
        <div class="card" on:click=move |_| set_modal.set(Some(p.clone()))>
            <h3 class="card-title">{project.title}</h3>
            <p class="card-desc">{project.desc}</p>
            <div class="card-tags">
                {project.tags.iter().map(|&t| view! {
                    <span class="card-tag">{t}</span>
                }).collect_view()}
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
                    if pos_x.get() > 400.0 {
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
                    if pos_x.get() < -150.0 {
                        set_phase.set(SpritePhase::Hidden);
                    }
                }
            }
        },
        Duration::from_millis(60),
    );

    let container_style = move || {
        if phase.get() == SpritePhase::Hidden { return "display:none;".to_string(); }
        let x = pos_x.get() as i64;
        format!("display:block;position:fixed;left:{x}px;bottom:0px;\
                 width:166px;height:125px;z-index:9999;pointer-events:none;")
    };
    let sprite_style = move || {
        let src = which.get();
        if src.is_empty() { return "display:none;".to_string(); }

        let f = frame.get();
        
        // 1. Explicitly use f64 for all math to avoid casting errors
        let frames_per_row = 12.0f64;
        let img_width = 2000.0f64;
        let img_height = 250.0f64;

        // Each cell is exactly 166x125
        let cell_width = 166.0f64;
        let cell_height = 125.0f64;

        let col = (f as f64 % frames_per_row).floor();
        let row = (f as f64 / frames_per_row).floor();

        let off_x = col * cell_width;
        let off_y = row * cell_height;

        let flip = if phase.get() == SpritePhase::RunOut { "transform:scaleX(-1);" } else { "" };

        format!(
            "width:{cell_width}px;\
             height:{cell_height}px;\
             background-image:url('public/{src}.png');\
             background-repeat:no-repeat;\
             background-size:{img_width}px {img_height}px;\
             background-position:-{off_x}px -{off_y}px;\
             image-rendering:pixelated;\
             {flip}"
        )
    };

    view! {
        <div style=container_style>
            <div style=sprite_style></div>
            <Show when=move || show_bang.get()>
                <div style="position:absolute;top:-38px;left:32px;font-size:40px;\
                            font-weight:900;color:#ffdd44;text-shadow:0 0 12px #ffaa00;\
                            animation:bang-pop 0.35s ease-out;">"!"</div>
            </Show>
        </div>
    }
}