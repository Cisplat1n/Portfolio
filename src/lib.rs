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
            <DnaBackground />
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

// ─── DNA ──────────────────────────────────────────────────────────────────


#[component]
fn DnaBackground() -> impl IntoView {
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("dna-canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let w = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let h = window.inner_height().unwrap().as_f64().unwrap() as u32;
        canvas.set_width(w);
        canvas.set_height(h);

        let col_width = 52.0f64;   // width per helix column
        let char_h = 16.0f64;      // vertical spacing between chars
        let num_cols = (w as f64 / col_width).ceil() as i32;
        let num_rows = (h as f64 / char_h).ceil() as i32 + 4;
        let chars = ['A', 'T', 'G', 'C'];

        let t = std::rc::Rc::new(std::cell::Cell::new(0.0f64));
        let draw: std::rc::Rc<std::cell::RefCell<Option<wasm_bindgen::closure::Closure<dyn FnMut()>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let draw_clone = draw.clone();
        let t_clone = t.clone();

        *draw.borrow_mut() = Some(wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let time = t_clone.get();
            t_clone.set(time + 0.018);

            // Fade trail
            ctx.set_fill_style_str("rgba(13, 15, 20, 0.15)");
            ctx.fill_rect(0.0, 0.0, w as f64, h as f64);

            ctx.set_font("11px monospace");

            for col in 0..num_cols {
                let cx = col as f64 * col_width + col_width / 2.0;
                // Each column scrolls at slightly different speed
                let speed = 0.8 + (col as f64 * 0.17) % 0.6;
                let scroll = (time * speed * char_h) % (num_rows as f64 * char_h);

                for row in 0..num_rows {
                    let y = row as f64 * char_h - scroll + (num_rows as f64 * char_h);
                    let y = y % (num_rows as f64 * char_h);

                    // Sine wave offset for left strand
                    let wave_phase = (row as f64 * 0.45) + time * speed;
                    let strand_offset = wave_phase.sin() * 18.0;

                    // Depth for brightness
                    let depth = (wave_phase.sin() + 1.0) / 2.0;
                    let alpha_strand = 0.06 + depth * 0.14;
                    let alpha_rung = 0.04;

                    // Left strand char
                    let ci_l = (row as usize + col as usize * 3 + (time * 3.0) as usize) % 4;
                    ctx.set_fill_style_str(&format!("rgba(79, 200, 180, {:.2})", alpha_strand));
                    ctx.fill_text(
                        &chars[ci_l].to_string(),
                        cx - 10.0 + strand_offset,
                        y,
                    ).unwrap();

                    // Right strand char (opposite phase)
                    let ci_r = (row as usize + col as usize * 5 + 2 + (time * 3.0) as usize) % 4;
                    let alpha_r = 0.06 + (1.0 - depth) * 0.14;
                    ctx.set_fill_style_str(&format!("rgba(79, 200, 180, {:.2})", alpha_r));
                    ctx.fill_text(
                        &chars[ci_r].to_string(),
                        cx + 10.0 - strand_offset,
                        y,
                    ).unwrap();

                    // Rung between strands when they cross
                    if strand_offset.abs() < 6.0 {
                        ctx.set_fill_style_str(&format!("rgba(120, 220, 200, {:.2})", alpha_rung));
                        ctx.fill_text("|", cx, y).unwrap();
                    }
                }
            }

            let cb = draw_clone.borrow();
            let func = cb.as_ref().unwrap().as_ref().unchecked_ref::<js_sys::Function>();
            web_sys::window().unwrap().request_animation_frame(func).unwrap();
        }) as Box<dyn FnMut()>));

        let cb = draw.borrow();
        let func = cb.as_ref().unwrap().as_ref().unchecked_ref::<js_sys::Function>();
        web_sys::window().unwrap().request_animation_frame(func).unwrap();
    });

    view! {
        <canvas id="dna-canvas" style="position:fixed;top:0;left:0;width:100%;height:100%;z-index:0;pointer-events:none;"/>
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