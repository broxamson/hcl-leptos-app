use crate::pages::global_components::api::{open_hcl_file, make_branch};
use leptos::*;

#[component]
pub fn Monaco() -> impl IntoView {
    use crate::pages::global_components::selectors::ListDirectories;
    let tf_dir = "".to_string();

    let load_hcl_file = create_resource(move || tf_dir.clone(), open_hcl_file);

    view! {
        {move || {
            load_hcl_file
                .get()
                .map(|wrapped| {
                    wrapped
                        .map(|file| {
                            view! {
        <link
            rel="stylesheet"
            data-name="vs/editor/editor.main"
            href="https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/editor/editor.main.min.css"
        />

        <script
            src="https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/loader.min.js"
            integrity="sha512-ZG31AN9z/CQD1YDDAK4RUAvogwbJHv6bHrumrnMLzdCrVu4HeAqrUX7Jsal/cbUwXGfaMUNmQU04tQ8XXl5Znw=="
            crossorigin="anonymous"
            referrerpolicy="no-referrer"
        ></script>

        <script
            src="https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/editor/editor.main.nls.min.js"
            integrity="sha512-v1Et8DY+94KH4DbAnoYA7xfAg/Kg8vVVXs9m5SiBgXeUMTgjIfg9OrRyr4KgUBgWHi2rtuz270jgTAFHZMDq5w=="
            crossorigin="anonymous"
            referrerpolicy="no-referrer"
        ></script>
        <script
            src="https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/editor/editor.main.js"
            integrity="sha512-1q+Hl4daVyNZ3RG+9k2rQSivZY83Fxd69INiHpgV+7q2onCTVlaUpLgAdKmf61HObyxgec2mDirHMH+6+3OB0w=="
            crossorigin="anonymous"
            referrerpolicy="no-referrer"
        ></script>
        <script
            src="https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/basic-languages/hcl/hcl.min.js"
            integrity="sha512-HIyr6DxEsHpUKt8b9FJIqDSv1wjXMgMFUqYwJH9eJq2obcFbFpTR/oRZGgJimdsdE9DS63l1H6zgTWt1227O8A=="
            crossorigin="anonymous"
            referrerpolicy="no-referrer"
        ></script>

        <CreateBranch/>

        <ListDirectories/>

        <div id="editor" style="height:400px;border:1px solid black;"></div>

        <button onclick="saveText()">Edit</button>

                                <script>
                                    require.config({ paths: { "vs": "https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/" } });
                                    
                                    require(["vs/editor/editor.main"], function () {
                                      var editor = monaco.editor.create(document.getElementById("editor"), {
                                        value: file,
                                        language: "hcl",
                                        theme: "vs-dark" ,
                                        lineNumbers: "on"//
                                      });

                                    
                                    });

                                </script>
                            }
                        })
                })
        }}
    }
}










#[component]
pub fn CreateBranch() -> impl IntoView {
    use leptos::ev::SubmitEvent;
    use leptos::html::Input;

    let (branch_name, set_branch_name) = create_signal("Create S3 branch".to_string());
    let (status, _set_status) = create_signal("".to_string());

    let input_element: NodeRef<Input> = create_node_ref();
    let on_create_branch = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().expect("<input> to exist").value();
        set_branch_name(value);

        let name = branch_name.get();
        spawn_local(async move {
            make_branch(name.to_string())
                .await
                .expect("Nothing Entered");
        })
    };

    view! {
        <form on:submit=on_create_branch>
            <input type="text" value=branch_name node_ref=input_element/>
            <input type="submit" value="Create"/>
        </form>
        <p>{status}</p>
    }
}
