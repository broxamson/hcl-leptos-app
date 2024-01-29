use leptos::*;

#[component]
pub fn CreateLT() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Create New Launch Template!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
