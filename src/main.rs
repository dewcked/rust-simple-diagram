//! Example: File Explorer
//! -------------------------
//!
//! This is a fun little desktop application that lets you explore the file system.
//!
//! This example is interesting because it's mixing filesystem operations and GUI, which is typically hard for UI to do.

use std::{path::{PathBuf, Path}, env};

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

fn main() {
    // simple_logger::init_with_level(log::Level::Debug).unwrap();
    dioxus_desktop::launch_cfg(App, Config::new()
        .with_window(WindowBuilder::new().with_resizable(true).with_inner_size(
                dioxus_desktop::wry::application::dpi::LogicalSize::new(400.0, 800.0),
            )
        )
    );
}

static App: Component<()> = |cx| {
    let files = use_ref(&cx, || Files::new());
	
	

    cx.render(rsx!(div {
        style { include_str!("./style.css") }
        header {
            i { class: "material-icons icon-menu", "menu" }
            h1 { "Files: ", files.read().current()}
            span { }
            i { class: "material-icons", onclick: move |_| files.write().go_up(), "↩" }
        }
        main {
            files.read().path_names.iter().enumerate().map(|(dir_id, path)| {
				let realpath = Path::new(path);
				let is_dir = realpath.is_dir();
				let child_cnt_tooltip = if is_dir {
					realpath.read_dir().unwrap().count()
				} else {
					0
				};
				let path_end = path.split(std::path::MAIN_SEPARATOR).last().unwrap_or(path.as_str());
                let icon_type = if is_dir {
                    "📁"
                } else {
                    "📄"
                };
                rsx! (
                    div { class: "folder", key: "{path}",
                        i { class: "material-icons",
                            onclick: move |_| files.write().enter_dir(dir_id),
                            "{icon_type}"
                            p { class: "cooltip", "{child_cnt_tooltip} folder or files" }
                        }
                        h1 { "{path_end}" }
                    }
                )
            })
            files.read().err.as_ref().map(|err| {
                rsx! (
                    div {
                        code { "{err}" }
                        button { onclick: move |_| files.write().clear_err(), "x" }
                    }
                )
            })
        }

    }))
};

struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    err: Option<String>,
}

impl Files {
    fn new() -> Self {
        let mut files = Self {
            path_stack: vec![env::current_dir().unwrap().into_os_string().into_string().unwrap()],
            path_names: vec![],
            err: None,
        };

        files.reload_path_list();

        files
    }

    fn reload_path_list(&mut self) {
        let cur_path = self.path_stack.last().unwrap();
        log::info!("Reloading path list for {:?}", cur_path);
        let paths = match std::fs::read_dir(cur_path) {
            Ok(e) => e,
            Err(err) => {
                let err = format!("An error occured: {:?}", err);
                self.err = Some(err);
                self.path_stack.pop();
                return;
            }
        };
        let collected = paths.collect::<Vec<_>>();
        log::info!("Path list reloaded {:#?}", collected);

        // clear the current state
        self.clear_err();
        self.path_names.clear();

        for path in collected {
            self.path_names
                .push(path.unwrap().path().display().to_string());
        }
        log::info!("path namees are {:#?}", self.path_names);
    }

    fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }

    fn enter_dir(&mut self, dir_id: usize) {
        let path = &self.path_names[dir_id];
        self.path_stack.push(path.clone());
        self.reload_path_list();
    }

    fn current(&self) -> &str {
        self.path_stack.last().unwrap()
    }
    fn clear_err(&mut self) {
        self.err = None;
    }
}