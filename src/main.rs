use dioxus::prelude::*;
fn main() {
    dioxus::desktop::launch(App);
}

fn App(cx: Scope) -> Element {

    let _title:&str = "this is title";

    cx.render(rsx! (
        head {
            link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" }
        }
        body {
            div {
                class:"text-center",
                h1{
                    class:"text-2xl font-serif font-extrabold mt-20",
                    "{_title}"}
                h2{
                    class:"text-xl font-bold mt-4",
                    "h2 line"}
                p{
                    class:"mt-4",
                    "text text"}
                button{
                    class:"px-4 py-2 mt-4 rounded-md bg-blue-400 hover:bg-blue-500",
                    "button"}
            }
       }  
    ))
}
