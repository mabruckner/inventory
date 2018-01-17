#[macro_use] extern crate yew;
extern crate serde;
#[macro_use] extern crate serde_derive;

use yew::html::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{Method, FetchService};
use yew::format::Json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub unit: String,
    pub schema: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Batch {
    pub id: i32,
    pub class: i32,
    pub quantity: String,
    pub data: String
}

struct Context {
    console: ConsoleService,
    fetch: FetchService<Msg>,
    server: String,
}


struct Model {
    name: String,
    batches: Option<Vec<Batch>>,
    classes: Option<HashMap<i32, Class>>
}

enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
    UpdateName(String),
    LoadClasses(Vec<Class>),
    LoadBatches(Vec<Batch>)
}

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::UpdateName(value) => {
            model.name = value
        },
        Msg::LoadClasses(data) => {
            let mut map = HashMap::new();
            for class in data.into_iter() {
                map.insert(class.id, class);
            }
            model.classes = Some(map);
        },
        Msg::LoadBatches(data) => {
            model.batches = Some(data);
        },
        _ => ()
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <link rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.6.2/css/bulma.css", />
            <nav class=("navbar"),>
                <div class=("navbar-menu"),>
                    <div class=("navbar-start"),>
                        <span class=("navbar-item"),>
                            {"Inventory"}
                        </span>
                    </div>
                </div>
            </nav>
            <section class=("section"),>
                <p>{format!("Hello {}!", model.name) }</p>
                <input value=&model.name,
                        oninput=|e: InputData| Msg::UpdateName(e.value),
                        />
                {batch_table(model)}
            </section>
        </div>
    }
}

fn batch_table(model: &Model) -> Html<Msg> {
    match (&model.batches, &model.classes) {
        (&Some(ref batches), &Some(ref classes)) => {
            html! {
                <table class=("table"),>
                    <thead>
                        <tr><th>{"ID"}</th><th>{"Type"}</th><th>{"Amount"}</th></tr>
                    </thead>
                    <tbody>
                        { for batches.iter().map(|batch| {
                            html!{
                                <tr>
                                    <td>{batch.id}</td>
                                    <td>{classes.get(&batch.class).map(|x| &x.name).unwrap_or(&"".into())}</td>
                                    <td>{&batch.quantity}</td>
                                </tr>
                            }
                                                         })
                        }
                </tbody>
                </table>
            }
        },
        _ => {
            html! {
                <p>
                {"Please wait while we load the values"}
                </p>
            }
        }
    }
}

fn main() {
    yew::initialize();
    let mut app = App::new();
    let mut context = Context {
        server: "http://localhost:5000".into(),
        console: ConsoleService,
        fetch: FetchService::new(app.sender()),
    };
    let model = Model {
        name: "you".into(),
        batches: None,
        classes: None
    };

    context.fetch.fetch(Method::Get, &(context.server.clone() + "/api/classes"), None, |Json(data): Json<Result<Vec<Class>, ()>>| Msg::LoadClasses(data.unwrap_or(vec![])));
    context.fetch.fetch(Method::Get, &(context.server.clone() + "/api/batches"), None, |Json(data): Json<Result<Vec<Batch>, ()>>| Msg::LoadBatches(data.unwrap_or(vec![])));

    
    app.mount(context, model, update, view);
    yew::run_loop();
}
