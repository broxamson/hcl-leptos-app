use leptos::*;

#[component]
pub fn CreateCert() -> impl IntoView {
    let (cert_name, _set_cert_name) = create_signal("");
    let on_create_cert = move |_| {
        let _cert_name = cert_name.get();
        todo!();
    };

    view! {
        <h1>"Create New SSL Certificate"</h1>
        // Input field for name
        <input type="text" placeholder="Domain Name" bind:value=cert_name/>
        <button on:click=on_create_cert>"Create Certificate"</button>
    }
}
