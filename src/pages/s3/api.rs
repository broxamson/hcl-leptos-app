use leptos::*;

use leptos::server;

#[server(CreateS3Bucket)]
pub async fn create_bucket(bucket_name: String) -> Result<String, ServerFnError> {
    use crate::git_func::{
        checkout_branch, clone_repo, commit_changes, create_new_branch, create_pull_request,
        delete_comitted_change, git_add_file, push_to_repository, PullRequest,
    };
    use crate::pages::s3::new_bucket::new_bucket;
    use dotenvy_macro::dotenv;
    use std::path::Path;
    const REPO_PATH: &str = dotenv!("REPO_DIR");

    let branch_name = bucket_name.to_string(); // Replace with your branch name
    let pull_request = PullRequest {
        title: branch_name.to_string(),
        description: format!("Creating new Bucket: {}", branch_name).to_string(),
        source_branch: branch_name.to_string(),
        destination_branch: "master".to_string(),
        base_url: "bitbucket.org".to_string(),
        project_key: "netreo".to_string(),
        repository_slug: "terraform".to_string(),
    };

    let url_base = pull_request.base_url.to_string();
    // The URL of the Git repository you want to clone
    let repo_url = format!("https://{}/netreo/terraform", url_base);

    let branch_dir = format!("{}/tf/{}", REPO_PATH, branch_name);

    if let Err(err) = delete_comitted_change(&branch_name).await {
        eprintln!("Error: {:?}", err);
        let e = err.to_string();
        return Err(leptos::ServerFnError::ServerError(e));
    }

    let local_path = Path::new(&branch_dir);

    if let Err(e) = clone_repo(&repo_url, local_path).await {
        eprintln!("Error cloning repository: {}", e);
        if let Err(err) = delete_comitted_change(&branch_name).await {
            eprintln!("Error: {:?}", err);
            let e = err.to_string();
            return Err(leptos::ServerFnError::ServerError(e));
        };
        let e = e.to_string();
        return Err(leptos::ServerFnError::ServerError(e));
    }
    println!("Repository cloned successfully to {:?}", local_path);

    if let Err(e) = create_new_branch(local_path, &branch_name).await {
        eprintln!("Error branching repository: {}", e);
        if let Err(err) = delete_comitted_change(&branch_name).await {
            eprintln!("Error: {:?}", err);
            let e = err.to_string();
            return Err(leptos::ServerFnError::ServerError(e));
        };
        let e = e.to_string();
        return Err(leptos::ServerFnError::ServerError(e));
    }
    println!("Branch {} Created.", branch_name);

    if let Err(e) = checkout_branch(local_path, &branch_name).await {
        eprintln!("Error Checking out Branch: {}", e);
        if let Err(err) = delete_comitted_change(&branch_name).await {
            eprintln!("Error: {:?}", err);
            let e = err.to_string();
            return Err(leptos::ServerFnError::ServerError(e));
        };
        let e = e.to_string();
        return Err(leptos::ServerFnError::ServerError(e));
    }
    println!("Branch {} Checked Out.", branch_name);

    new_bucket(&branch_name);

    let file_name = format!("modules/dev_s3/{}.tf", branch_name);

    if let Err(e) = git_add_file(local_path, &file_name).await {
        eprintln!("Error adding file to the staging area: {}", e);
        if let Err(err) = delete_comitted_change(&branch_name).await {
            eprintln!("Error: {:?}", err);
            let e = err.to_string();
            return Err(leptos::ServerFnError::ServerError(e));
        };
        let e = e.to_string();
        return Err(leptos::ServerFnError::ServerError(e));
    }
    println!("File added to the staging area.");

    if let Err(e) =
        commit_changes(local_path, &branch_name, "nicholas", "nvanamen@netreo.com").await
    {
        eprintln!("Error committing and pushing changes: {}", e);
        if let Err(err) = delete_comitted_change(&branch_name).await {
            eprintln!("Error: {:?}", err);
            let e = err.to_string();
            return Err(leptos::ServerFnError::ServerError(e));
        };
        let e = e.to_string();
        return Err(leptos::ServerFnError::ServerError(e));
    }
    println!("Changes committed and pushed successfully.");
    let branch_path = Path::new(&branch_dir);
    if let Err(e) = push_to_repository(branch_path, &branch_name).await {
        eprintln!("Error pushing to the remote repository: {}", e);
        if let Err(err) = delete_comitted_change(&branch_name).await {
            eprintln!("Error: {:?}", err);
            let e = err.to_string();
            return Err(leptos::ServerFnError::ServerError(e));
        };
        let e = e.to_string();
        return Err(leptos::ServerFnError::ServerError(e));
    }

    println!(
        "Branch '{}' pushed successfully to the remote repository.",
        &branch_name
    );

    if let Err(err) = delete_comitted_change(&branch_name).await {
        eprintln!("Error: {:?}", err);
        let e = err.to_string();
        return Err(leptos::ServerFnError::ServerError(e));
    }

    if let Err(err) = create_pull_request(pull_request).await {
        eprintln!("Error: {:?}", err);
        let e = err.to_string();
        return Err(leptos::ServerFnError::ServerError(e));
    }
    let return_message = format!("{} Created Successfully", &branch_name);
    Ok(return_message)
}
