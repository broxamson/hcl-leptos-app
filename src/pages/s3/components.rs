use leptos::*;

#[component]
pub fn CreateS3() -> impl IntoView {
    use crate::pages::s3::api::create_bucket;
    use leptos::ev::SubmitEvent;
    use leptos::html::Input;

    let (bucket_name, set_bucket_name) = create_signal("".to_string());
    let (status, set_status) = create_signal("".to_string());

    let input_element: NodeRef<Input> = create_node_ref();
    let on_create_bucket = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().expect("<input> to exist").value();
        set_bucket_name(value);

        let name = bucket_name.get();
        spawn_local(async move {
            if let Err(err) = create_bucket(name.to_string()).await.map(set_status) {
                eprintln!("Error: {:?}", err);
            }
        })
    };

    view! {
            <h3>"Create S3 Bucket "</h3>
        <form on:submit=on_create_bucket>
            <input type="text"
                value=bucket_name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>{status}</p>
    }
}
