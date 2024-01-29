use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to NetreOxide!!"</h1>
        <p>"Select An AWS Service"</p>
        <ul style="list-style-type: none;">
            <li>
                <a href="/">Home</a>
            </li>
            <li>
                <a href="/s3">S3 Bucket</a>
            </li>
            <li>
                <a href="/lt">Launch Template</a>
            </li>
            <li>
                <a href="/lb">Load Balancer</a>
            </li>
            <li>
                <a href="/asg">ASG</a>
            </li>
            <li>
                <a href="/cert">Certificates</a>
            </li>
            <li>
                <a href="/ec2">Launch EC2</a>
            </li>
        </ul>
    }
}
