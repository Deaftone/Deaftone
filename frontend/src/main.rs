use yew::prelude::*;

use crate::components::toast::{Toast, Type};

mod components;
mod data;
enum Msg {
    ScanLibrary,
    ApiResponse(String),
}
#[allow(dead_code)]
struct App {
    user: data::Result,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            user: data::Result::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::ScanLibrary => {
                link.send_future(App::scan_library());
                false
            }
            /*             Msg::PopulateMetadata => {
                //link.send_future(App::populate_metadata());
                false
            } */
            Msg::ApiResponse(res) => {
                //log::info!("Update Person: {:?}", { &person });
                log::info!("{:}", res);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="h-screen bg-gray-900 w-full flex flex-col items-center justify-center gap-y-4">
                    <button onclick={ctx.link().callback(|_| { Msg::ScanLibrary })} type="button" class="btn btn-primary">{"Add"}</button>
                  <View/>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        /*         if first_render {}
         */
    }
}
#[function_component(View)]
fn view() -> Html {
    html! {
        <>
        <Toast class="alert-info shadow-lg" r#type={Type::Warning}>
    //    <ArrowCircleLeftOutLined />
       <div>
            <span class="mr-16">{"New software update available."} </span>
            <div class="float justify-right items-right">
            </div>
        </div>
       </Toast>
       <Toast class="alert-info shadow-lg" r#type={Type::Info}>
       //    <ArrowCircleLeftOutLined />
          <div>
               <span class="mr-16">{"New software update available.2"} </span>
               <div class="float justify-right items-right">
               </div>
           </div>
          </Toast>
        </>
    }
}
impl App {
    async fn scan_library() -> Msg {
        let res = reqwest_wasm::get("http://localhost:3030/tasks?task=scan_library")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        return Msg::ApiResponse(res);
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
