/*
    Resolves a relative path against a base directory and returns the
    normalized absolute path as a String.
*/

pub fn get_full_path
(
    /* Relative path */
    relative: &str,
    /* Absolute path */
    absolute: &str
)
/* Full path */
-> String
{
    let relative_path = Path::new( relative );
    let absolute_path = Path::new( absolute );

    if relative_path.is_absolute()
    {
        relative_path.to_string_lossy().into_owned()
    }
    else
    {
        absolute_path.join( relative_path ).to_string_lossy().into_owned()
    }
}
