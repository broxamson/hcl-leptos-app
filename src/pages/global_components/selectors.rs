use crate::pages::global_components::api::*;
use leptos::*;

#[component]
pub fn ListDirectories() -> impl IntoView {
    let tf_dir = "".to_string();

    let branch_resource = create_resource(move || tf_dir.clone(), list_directory);

    let (mod_dir, set_mod_dir) = create_signal("".to_string());

    view! {
        <Suspense>

            {move || {
                branch_resource
                    .get()
                    .map(|wrapped| {
                        wrapped
                            .map(|branches| {
                                view! {
                                    <select on:change=move |ev| {
                                        set_mod_dir(event_target_value(&ev));
                                    }>

                                        <option disabled selected>
                                            "SELECT BRANCH"
                                        </option>
                                        <ul>
                                            <For
                                                each=move || branches.clone()
                                                key=|n| n.clone()
                                                let:branch_dir
                                            >

                                                <option prop:value=&branch_dir>{&branch_dir}</option>

                                            </For>
                                        </ul>
                                    </select>
                                }
                            })
                    })
            }}
            <ListFiles selected=mod_dir/>
        </Suspense>
    }
}

#[component]
pub fn ListFiles(selected: ReadSignal<String>) -> impl IntoView {
    let tf_dir = move || format!("{}/modules/dev_s3", selected.get());

    let terraform_files = create_resource(tf_dir, list_directory);
    let (read_selected, set_selected) = create_signal("".to_string());



    view! {
        <p>Selected Branch: {selected}</p>
        <Suspense>

            {move || {
                terraform_files
                    .get()
                    .map(|tfl| {
                        tfl
                            .map(|tf_files| {
                                view! {
                                    <select on:change=move |ev| {
                                        set_selected(event_target_value(&ev));
                                    }>

                                        <option disabled selected>
                                            "SELECT FILE"
                                        </option>

                                            <For
                                                each=move || tf_files.clone()
                                                key=|n| n.clone()
                                                let:list_files
                                            >
                                                <option value=&list_files>{&list_files}</option>



                                            </For>

                                    </select>
                                    <p>{read_selected}</p>
                                }
                            })
                    })
            }}

        </Suspense>
    }


}
