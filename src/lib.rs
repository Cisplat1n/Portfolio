use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use std::time::Duration;

// ─── Data ────────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct Project {
    title: &'static str,
    desc: &'static str,
    tags: &'static [&'static str],
    url: &'static str,
    category: &'static str,
}

fn all_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Inference Cluster",
            desc: "Distributed llama.cpp inference across BC-250 + MI50 over 10GbE RPC.",
            tags: &["Rust", "Hardware"],
            url: "#",
            category: "Rust",
        },
        Project {
            title: "This Portfolio",
            desc: "Built entirely in Rust + Leptos, compiled to WebAssembly. Zero JavaScript.",
            tags: &["Rust", "WebAssembly"],
            url: "#",
            category: "Rust",
        },
        Project {
            title: "LLM Data Pipeline",
            desc: "High-throughput data pipeline for LLM training dataset curation and filtering.",
            tags: &["ML / AI", "Python"],
            url: "#",
            category: "ML / AI",
        },
        Project {
            title: "Model Evaluation Harness",
            desc: "Automated evaluation framework for benchmarking inference quality and latency.",
            tags: &["ML / AI", "Python"],
            url: "#",
            category: "Python",
        },

        Project {
            title: "Quartet Extraction Engine",
            desc: "Music analysis tool for extracting instrument parts from quartet recordings.",
            tags: &["Python", "Data Science"],
            url: "#",
            category: "Data Science",
        },
        Project {
            title: "Statistical Modelling Suite",
            desc: "R-based Bayesian modelling toolkit for time-series and regression analysis.",
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

    view! {
        <div class="app">
            <Header />
            <main class="main-content">
                <FilterBar active=active_tag on_select=on_select />
                <ProjectGrid projects=filtered />
            </main>
            <SpriteBar trigger=sprite_trigger />
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
fn ProjectGrid(projects: Memo<Vec<Project>>) -> impl IntoView {
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
                                    children=|p| view! { <ProjectCard project=p /> }
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
fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <a href={project.url} class="card" target="_blank" rel="noopener">
            <h3 class="card-title">{project.title}</h3>
            <p class="card-desc">{project.desc}</p>
            <div class="card-tags">
                {project.tags.iter().map(|&t| view! {
                    <span class="card-tag">{t}</span>
                }).collect_view()}
            </div>
        </a>
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
                    // Cycles top row frames 0-5
                    set_frame.update(|f| *f = (*f + 1) % 6);
                    set_pos_x.update(|x| *x += 4.0);
                    if pos_x.get() > 400.0 {
                        set_phase.set(SpritePhase::Exclamation);
                        set_bang.set(true);
                        set_frame.set(6); 
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
                    // Cycles bottom row frames 12-17
                    set_frame.update(|f| {
                         if *f < 12 || *f >= 17 { *f = 12 } else { *f += 1 }
                    });
                    set_pos_x.update(|x| *x -= 8.0);
                    if pos_x.get() < -150.0 {
                        set_phase.set(SpritePhase::Hidden);
                    }
                }
            }
        },
        Duration::from_millis(60),
    );

    let disp: u32 = 96;
    let frames_per_row: u32 = 12;

    let container_style = move || {
        if phase.get() == SpritePhase::Hidden { return "display:none;".to_string(); }
        let x = pos_x.get() as i64;
        format!("display:block;position:fixed;left:{x}px;bottom:0px;\
                 width:{disp}px;height:{disp}px;z-index:9999;pointer-events:none;")
    };
    let sprite_style = move || {
        let src = which.get();
        if src.is_empty() { return "display:none;".to_string(); }

        let f = frame.get();
        
        // 1. Explicitly use f64 for all math to avoid casting errors
        let frames_per_row = 12.0f64; 
        let img_width = 1414.0f64;
        let img_height = 176.0f64;
        
        // 2. Calculate cell size based on your 1414x176 image
        let cell_width = img_width / frames_per_row;   // ~117.83px
        let cell_height = img_height / 2.0;            // 88.0px
        
        // 3. FIX: Use the variable in the math to clear the 'unused' warning
        // Use floor() to ensure we stay within the correct grid cell
        let col = (f as f64 % frames_per_row);
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