use leptos::*;

#[component]
pub fn LaunchEC2() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Launch New EC2"</h1>
        <label for="Launch Template">Select a Launch Template</label>

        <select name="Launch Template" id="Launch Template">
            <option value="volvo">Volvo</option>
            <option value="saab">Saab</option>
            <option value="mercedes">Mercedes</option>
            <option value="audi">Audi</option>
        </select>
        <label>"Instance Count" <input type="number" name="count"/></label>
        <button on:click=on_click>"Launch Instances: " {count}</button>
    }
}
