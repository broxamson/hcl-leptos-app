use leptos::{server, ServerFnError};

#[server()]
pub async fn make_branch(branch_name: String) -> Result<String, ServerFnError> {
    use crate::git_func::{checkout_branch, clone_repo, create_new_branch, delete_comitted_change};
    use dotenvy_macro::dotenv;
    use std::path::Path;

    const URL_BASE: &str = dotenv!("REPO_URL");
    const REPO_PATH: &str = dotenv!("REPO_DIR");

    // The URL of the Git repository you want to clone
    let repo_url = format!("https://{}/netreo/terraform", URL_BASE);

    let branch_dir = format!("{}/tf/{}", REPO_PATH, branch_name);

    if let Err(err) = delete_comitted_change(&branch_name).await {
        eprintln!("Error: {:?}", err);
        let e = err.to_string();
        return Err(ServerFnError::ServerError(e));
    }

    let local_path = Path::new(&branch_dir);

    if let Err(e) = clone_repo(&repo_url, local_path).await {
        eprintln!("Error cloning repository: {}", e);
        if let Err(err) = delete_comitted_change(&branch_name).await {
            eprintln!("Error: {:?}", err);
            let e = err.to_string();
            return Err(ServerFnError::ServerError(e));
        };
        let e = e.to_string();
        return Err(ServerFnError::ServerError(e));
    }
    println!("Repository cloned successfully to {:?}", local_path);

    if let Err(e) = create_new_branch(local_path, &branch_name).await {
        eprintln!("Error branching repository: {}", e);
        if let Err(err) = delete_comitted_change(&branch_name).await {
            eprintln!("Error: {:?}", err);
            let e = err.to_string();
            return Err(ServerFnError::ServerError(e));
        }
    }

    if let Err(err) = checkout_branch(local_path, &branch_name).await {
        eprintln!("Error branching repository: {}", err);
        if let Err(err) = delete_comitted_change(&branch_name).await {
            eprintln!("Error: {:?}", err);
            let e = err.to_string();
            return Err(leptos::ServerFnError::ServerError(e));
        }
    }
    let return_message = format!("{} Created Successfully", &branch_name);
    Ok(return_message)
}

#[server()]
pub async fn list_directory(directory_path: String) -> Result<Vec<String>, ServerFnError> {
    extern crate glob;
    use glob::glob;
    use dotenvy_macro::dotenv;

    let dir = dotenv!("REPO_DIR");
    dbg!(&dir);
    let path = {if directory_path.is_empty() { "" } else { &directory_path }};
    let full_path = format!("{}/{}", dir, path);
    let directory_path_glob = format!("{}/*", full_path);

    dbg!(&directory_path_glob);

    let files: Vec<String> = glob(directory_path_glob.as_str())
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter_map(|entry| entry.to_str().map(String::from))
        .map(|p| p.replace(dir, ""))
        .map(|ph| ph.replace(&directory_path, ""))
        .map(|ph| ph.replace('/', ""))
        .collect();

    dbg!(&files);
    if Some(&files).is_some() {

        Ok(files)
    } else {
        let e = "No Files Found".to_string();
        Err(ServerFnError::ServerError(e))
    }
}


#[server()]
pub async fn open_hcl_file(file_path: String) -> Result<String, ServerFnError> {
    use std::fs::{ OpenOptions};

    use std::io::{Read};
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    // Read the file contents into a string
    let mut content = String::new();
    let hcl_file = file.read_to_string(&mut content)?;

    Ok(hcl_file.to_string())
}
#[server()]
pub async fn save_hcl_file(modified_content: String) -> Result<String, ServerFnError> {
    use std::fs::{ OpenOptions};

   use std::io::{ Write};
    let mut file = OpenOptions::new().read(true).write(true).open("test.tf")?;

        // Read the file contents into a string

   file.write_all(modified_content.as_bytes())?;

        // Close the file explicitly (not required, but good practice)
   file.sync_all()?;
   Ok("".to_string())

}