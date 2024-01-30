use leptos::*;

#[component]
pub fn ManageS3() -> impl IntoView {
    use crate::pages::global_components::text_editor::Monaco;
    use crate::pages::s3::components::CreateS3;

    view! {
        <title>"Manage S3"</title>

        <CreateS3/>

        <h2>"Edit S3 Files"</h2>

        <Monaco/>
    }
}
