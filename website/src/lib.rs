#![recursion_limit = "256"]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate stdweb;

mod markdown;

use std::collections::HashMap;
use yew::format::{Nothing, Toml};
use yew::services::fetch::FetchTask;
use yew::services::fetch::{FetchService, Request, Response};
use yew::services::Task;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub mod content {
    use chrono::DateTime;
    use chrono::Utc;
    use serde_derive::Deserialize;
    use std::collections::HashMap;
    use yew::Html;
    pub static INDEX_FILENAME: &str = "index.toml";
    pub fn get_content_path(filename: &str) -> String {
        format!("/content/{}", filename)
    }

    pub type Title = String;
    pub type RawData = Vec<u8>;
    pub type Filename = String;
    pub type Tag = String;
    pub type Viewname = String;

    #[derive(Deserialize, Debug, Clone)]
    pub struct Index {
        pub title: Title,
        pub default_view: Viewname,
        pub views: HashMap<Viewname, Filename>,
        pub blog: Blog,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Blog {
        pub files: Vec<Filename>,
    }

    #[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
    pub struct Metadata {
        pub title: Title,
        pub date: DateTime<Utc>,
        pub display: bool,
        pub tags: Vec<Tag>,
        pub media_type: String,
        pub filename: Filename,
    }

    pub fn render(raw_data: &RawData) -> Html {
        let s = std::str::from_utf8(raw_data).unwrap();

        crate::markdown::render_markdown(s)
    }
}

type Error = failure::Error;

pub struct Model {
    fetch_service: FetchService,
    link: ComponentLink<Model>,

    tasks: Vec<FetchTask>,

    initialized: bool,
    index: Option<content::Index>,
    current_content: Option<content::Filename>,
    content_metadata: HashMap<content::Filename, content::Metadata>,
    content_data: HashMap<content::Filename, content::RawData>,
}

#[derive(Debug)]
pub enum Msg {
    FetchIndexReady(Result<content::Index, Error>),
    FetchContentMetadataReady(Result<(content::Filename, content::Metadata), Error>),
    FetchContentReady(Result<(content::Viewname, content::RawData), Error>),
    FetchIndex,
    ChangeContent(usize),
}

impl Model {
    pub fn view_data(&self) -> Html {
        let loading = html! {
            <div class="main">
                <h1>{ "Content loading..." }</h1>
                <button onclick=self.link.callback(|_| Msg::FetchIndex)>{ "Reload manually" }</button>
            </div>
        };

        let content_index = if let Some(content_index) = self.current_content {
            content_index
        } else {
            return loading;
        };

        if !self.initialized {
            return loading;
        };

        let raw_data = self.content_data.get(&content_index).unwrap();
        let main = content::render(raw_data);

        let title = "hello";

        html! { <>
            <title>{ title }</title>

            <div class="menu">
                {
                    self.index.clone().unwrap()
                    .views
                    .keys()
                    .enumerate()
                    .map(|(index, viewname)| {
                        let viewname_outer = viewname.clone();
                        html! {
                            <button onclick=self.link.callback(move |_| Msg::ChangeContent(index))>{ viewname.clone() }</button>
                        }
                }).collect::<Html>()
                }
                <button onclick=self.link.callback(|_| Msg::FetchIndex)>{ "Blog" }</button>
            </div>

            <div class="main"> { main } </div>
        </> }
    }

    pub fn fetch_index(&mut self) -> FetchTask {
        let request = Request::get(content::get_content_path(content::INDEX_FILENAME))
            .body(Nothing)
            .unwrap();

        let callback =
            self.link
                .callback(|response: Response<Toml<Result<content::Index, Error>>>| {
                    let (meta, Toml(data)) = response.into_parts();
                    log::info!("META: {:?}, {:?}", meta, data);
                    Msg::FetchIndexReady(data.map_err(Into::into))
                });

        self.fetch_service.fetch(request, callback)
    }

    pub fn update_from_index(&mut self) -> Vec<FetchTask> {
        self.index
            .clone()
            .unwrap()
            .views
            .values()
            .cloned()
            .chain(self.index.clone().unwrap().blog.files.iter().cloned())
            .map(|file| {
                let path = content::get_content_path(&file);
                log::info!("Preparing request to fetch {}", path);
                let request = Request::get(path).body(Nothing).unwrap();
                let callback = self.link.callback(
                    move |response: Response<Toml<Result<content::Metadata, Error>>>| {
                        let (meta, Toml(data)) = response.into_parts();
                        log::info!("META: {:?}, {:?}", meta, data);
                        Msg::FetchContentMetadataReady(data.map(|data| (file.clone(), data)))
                    },
                );

                self.fetch_service.fetch(request, callback)
            })
            .collect::<Vec<_>>()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Model {
            link,

            fetch_service: Default::default(),
            tasks: Default::default(),

            initialized: false,
            index: None,
            current_content: None,
            content_metadata: Default::default(),
            content_data: Default::default(),
        };

        let fetch_index_task = model.fetch_index();
        model.tasks.push(fetch_index_task);

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("[Model::update]");

        self.tasks.retain(|task| task.is_active());

        match msg {
            Msg::ChangeContent(content) => {
                self.current_content = Some(content);
                true
            }

            Msg::FetchIndex => {
                let task = self.fetch_index();
                self.tasks.push(task);
                false
            }

            // FetchIndexReady
            Msg::FetchIndexReady(Ok(index)) => {
                log::debug!("{:?}", index);
                self.index = Some(index);
                let tasks = self.update_from_index();
                self.tasks.extend(tasks.into_iter());
                false
            }
            Msg::FetchIndexReady(Err(e)) => {
                log::error!("{}", e);
                false
            }

            // FetchContentMetadataReady
            Msg::FetchContentMetadataReady(Ok((filename, content_metadata))) => {
                log::debug!("{:?}", content_metadata);

                let content_filename = content_metadata.filename.clone();
                self.content_metadata.insert(filename, content_metadata);

                let request = {
                    let path = content::get_content_path(&content_filename.to_string());
                    log::info!("Preparing request to fetch {}", path);
                    Request::get(path).body(Nothing).unwrap()
                };

                let callback = self.link.callback(
                    move |response: Response<Result<content::RawData, Error>>| {
                        let (meta, data) = response.into_parts();
                        log::info!("META: {:?}", meta);
                        Msg::FetchContentReady(data.map(|data| (content_filename.clone(), data)))
                    },
                );

                let fetch_task = self.fetch_service.fetch_binary(request, callback);
                self.tasks.push(fetch_task);

                false
            }
            Msg::FetchContentMetadataReady(Err(e)) => {
                log::error!("{}", e);
                false
            }

            // FetchContentReady
            Msg::FetchContentReady(Ok((metadata, content))) => {
                log::debug!("");

                self.content_data.insert(content.0, content.1);
                self.initialized |= self.tasks.is_empty();

                true
            }
            Msg::FetchContentReady(Err(e)) => {
                log::error!("{}", e);
                false
            }
        }
    }

    fn view(&self) -> Html {
        log::info!("[Model::view]");

        self.view_data()
    }
}
