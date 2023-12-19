use leptos::*;


fn main() {
 mount_to_body(|| view!{<App />});
}


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);  
    let double_count = move || count() *2;
    view! {
    <button 
        on:click=move |_| {
        set_count.update(|n| *n +=1 );
        }
        class:red=move || count() % 2 == 1



        >"Click me: "
        {move||count}
        </button>
        <ProgressBar progress=count />
        <ProgressBar progress=double_count />

    }
}

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    #[prop(default = 100)]
    max:u16,
    progress: F) -> impl IntoView {
    view! {
        <progress max=max value=progress />
    }

}
