/*
    Conver ~ path in to home path
*/
pub fn expand_path
(
    path: &str
) 
-> String 
{
    if path.starts_with("~/") 
    {
        format!("{}/{}", std::env::var("HOME").unwrap_or_default(), &path[2..])
    } 
    else if path.starts_with('~')
    {
        format!("{}/{}", std::env::var("HOME").unwrap_or_default(), &path[1..])
    } 
    else 
    {
        path.to_string()
    }
}



/*
    Check path
*/
pub fn ensure_directory
(
    path: &str
)
-> Result<(), String>
{
    let expanded = expand_path(path);
    
    // If path ends with '/', use it as directory, otherwise use parent
    let target = if expanded.ends_with('/')
    {
        expanded
    }
    else
    {
        match std::path::Path::new(&expanded).parent()
        {
            Some(parent) => parent.to_str().unwrap_or("").to_string(),
            None => return Err("Invalid path: cannot extract parent directory".to_string()),
        }
    };
    
    if let Err(e) = std::fs::create_dir_all(&target)
    {
        return Err(format!("Failed to create directory {}: {}", target, e));
    }
    
    Ok(())
}



pub fn extract_path( full_path: &str)
-> String
{
    std::path::Path::new(full_path)
        .parent()
        .unwrap_or_else(|| std::path::Path::new(""))
        .to_str()
        .unwrap_or("")
        .to_string()
}
