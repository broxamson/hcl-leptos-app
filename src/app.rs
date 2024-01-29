use crate::error_template::{AppError, ErrorTemplate};
//use leptos::server_fn::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    use crate::pages::acm::CreateCert;
    use crate::pages::asg::CreateASG;
    use crate::pages::ec2::LaunchEC2;
    use crate::pages::global_components::nav_bar::NavBar;
    use crate::pages::home::HomePage;
    use crate::pages::launch_template::CreateLT;
    use crate::pages::load_balancer::CreateLB;
    use crate::pages::s3::route::ManageS3;

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <NavBar/>

        // injects a stylesheet into the document <head/>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/noxide-fe.css"/>

        // sets the document title
        <Title text="Welcome to NetreOxide"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="s3" view=ManageS3/>
                    <Route path="lt" view=CreateLT/>
                    <Route path="lb" view=CreateLB/>
                    <Route path="asg" view=CreateASG/>
                    <Route path="acm" view=CreateCert/>
                    <Route path="ec2" view=LaunchEC2/>
                </Routes>
            </main>
        </Router>
    }
}
