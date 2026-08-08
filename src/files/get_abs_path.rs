/*
    Resolves a path against the current working directory (pwd).
    Uses get_full_path, which resolves relative paths against a base directory.
*/

pub fn get_abs_path
(
    /* Path to resolve */
    path: &str
)
/* Absolute path */
-> String
{
    let pwd = std::env::current_dir()
    .map( |p| p.to_string_lossy().into_owned() )
    .unwrap_or_default();

    get_full_path( path, &pwd )
}
