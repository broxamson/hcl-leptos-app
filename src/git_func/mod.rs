#[cfg(feature = "ssr")]
use anyhow::Error;
#[cfg(feature = "ssr")]
use dotenvy_macro::dotenv;
#[cfg(feature = "ssr")]
use git2::{
    BranchType, Cred, FetchOptions, Object, PushOptions, RemoteCallbacks, Repository, Signature,
};
#[cfg(feature = "ssr")]
use std::path::Path;

#[cfg(feature = "ssr")]
extern crate git2;
#[cfg(feature = "ssr")]
use git2::build::{CheckoutBuilder, RepoBuilder};

#[cfg(feature = "ssr")]
use tokio::fs::remove_dir_all;

#[cfg(feature = "ssr")]
pub async fn clone_repo(repo_url: &str, local_path: &Path) -> Result<(), Error> {
    let mut callbacks = RemoteCallbacks::new();
    let username_env = dotenv!("BITBUCKET_USER");
    let password_env = dotenv!("BITBUCKET_PASSWORD");
    callbacks.credentials(|_url, _username, _allowed| {
        // Provide your credentials here
        Cred::userpass_plaintext(username_env, password_env)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_options);

    repo_builder.clone(repo_url, local_path)?;

    Ok(())
}
#[cfg(feature = "ssr")]
pub async fn create_new_branch(repo_path: &Path, branch_name: &str) -> Result<(), Error> {
    // Open the existing Git repository
    let repo = Repository::open(repo_path)?;

    // Get the HEAD reference (current branch or commit)
    let head = repo.head()?;

    // Resolve the HEAD reference to an AnnotatedCommit
    let annotated_commit = repo.reference_to_annotated_commit(&head)?;

    // Resolve the AnnotatedCommit to a Commit
    let commit = repo.find_commit(annotated_commit.id())?;

    // Create a new branch from the Commit
    repo.branch(branch_name, &commit, false)?;

    Ok(())
}
#[cfg(feature = "ssr")]
pub async fn checkout_branch(repo_path: &Path, branch_name: &str) -> Result<(), git2::Error> {
    // Open the Git repository
    let repo = Repository::open(repo_path)?;

    // Lookup the branch by name
    let branch = repo.find_branch(branch_name, BranchType::Local)?;

    // Get the target commit of the branch
    let target_commit = branch.into_reference().peel_to_commit()?;

    // Convert the target commit to an &Object
    let target_object: &Object = target_commit.as_object();

    // Update the HEAD to point to the branch
    repo.set_head(&format!("refs/heads/{}", branch_name))?;

    // Checkout the branch (optional)
    let mut checkout_builder = CheckoutBuilder::new();
    repo.checkout_tree(target_object, Some(&mut checkout_builder))?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn commit_changes(
    repo_path: &Path,
    author_name: &str,
    author_email: &str,
    commit_message: &str,
) -> Result<(), git2::Error> {
    // Open the existing Git repository
    let repo = Repository::open(repo_path)?;

    // Get the index (staging area) to stage changes
    let mut index = repo.index()?;

    // Stage changes you want to commit
    // Committer's identity
    let signature = Signature::now(author_name, author_email)?;

    // Commit the changes
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Get the current HEAD commit (or None if it's the initial commit)
    let parent_commit = match repo.head().and_then(|r| r.peel_to_commit()) {
        Ok(commit) => Some(commit),
        _ => None,
    };

    if let Some(parent) = parent_commit {
        repo.commit(
            Some("HEAD"), // Point HEAD to the new commit
            &signature,
            &signature,
            commit_message,
            &tree,
            &[&parent],
        )?;
    } else {
        repo.commit(
            Some("HEAD"), // Point HEAD to the new commit
            &signature,
            &signature,
            commit_message,
            &tree,
            &[], // Empty array because there's no parent commit
        )?;
    }

    Ok(())
}
#[cfg(feature = "ssr")]
pub async fn push_to_repository(repo_path: &Path, branch_name: &str) -> Result<String, String> {
    let username_env = dotenv!("BITBUCKET_USER");
    let password_env = dotenv!("BITBUCKET_PASSWORD");
    // Open the Git repository
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    // Get the remote (e.g., "origin")
    let mut remote = match repo.find_remote("origin") {
        Ok(remote) => remote,
        Err(e) => return Err(format!("Failed to find remote 'origin': {}", e)),
    };

    // Set up remote callbacks for authentication
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username, _allowed| {
        // Provide your credentials here
        Cred::userpass_plaintext(username_env, password_env)
    });

    // Set up push options with remote callbacks
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // Push the branch to the remote
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    match remote.push(&[&refspec], Some(&mut push_options)) {
        Ok(_) => {
            let message = format!("Pushed branch '{}'", branch_name);
            println!("{}", &message);
            Ok(message)
        }
        Err(e) => Err(format!("Failed to push branch '{}': {}", branch_name, e)),
    }
}
#[cfg(feature = "ssr")]
pub async fn git_add_file(repo_path: &Path, file_path: &str) -> Result<(), git2::Error> {
    // Open the Git repository
    let repo = Repository::open(repo_path)?;

    // Get the repository's index
    let mut index = repo.index()?;

    // Add the file to the index
    index.add_path(file_path.as_ref())?;

    // Write the index to update the staging area
    index.write()?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn delete_comitted_change(dir: &str) -> Result<(), Error> {
    let file_path = Path::new(&dir);
    println!("{}", file_path.display());
    if file_path.exists() {
        println!("Removing {:?}", file_path);
        remove_dir_all(file_path).await.expect("error deleting dir");
    }
    Ok(())
}
