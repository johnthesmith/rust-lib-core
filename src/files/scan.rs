/*
    SPDX-License-Identifier: MIT
    SPDX-FileCopyrightText: 2026 Still Swamp
*/


/*
    Match a path against a glob pattern.

    Supported:
        *   — any chars except '/'
        **  — any chars including '/'
        ?   — single char except '/'
*/
fn matches( pattern: &[u8], path: &[u8] ) -> bool
{
    if pattern.is_empty()
    {
        return path.is_empty();
    }

    match pattern[ 0 ]
    {
        /* ** — any depth */
        b'*' if pattern.len() > 1 && pattern[ 1 ] == b'*' =>
        {
            let mut rest = &pattern[ 2.. ];
            if !rest.is_empty() && rest[ 0 ] == b'/'
            {
                rest = &rest[ 1.. ];
            }

            for i in 0..=path.len()
            {
                if matches( rest, &path[ i.. ] )
                {
                    return true;
                }
            }
            false
        }

        /* * — any chars except '/' */
        b'*' =>
        {
            let rest = &pattern[ 1.. ];
            let mut i = 0;
            loop
            {
                if matches( rest, &path[ i.. ] )
                {
                    return true;
                }
                if i >= path.len() || path[ i ] == b'/'
                {
                    return false;
                }
                i += 1;
            }
        }

        /* ? — single char except '/' */
        b'?' =>
        {
            !path.is_empty()
            && path[ 0 ] != b'/'
            && matches( &pattern[ 1.. ], &path[ 1.. ] )
        }

        /* Literal */
        c =>
        {
            !path.is_empty()
            && path[ 0 ] == c
            && matches( &pattern[ 1.. ], &path[ 1.. ] )
        }
    }
}



/*
    Recursive dir walk with glob filter.
    `base` is used to build relative path for matching.
*/
fn walk
(
    dir: &Path,
    base: &Path,
    pattern: &str,
    out: &mut Vec<String>
)
{
    let entries = match std::fs::read_dir( dir )
    {
        Ok( e ) => e,
        Err( _ ) => return,
    };

    for entry in entries.flatten()
    {
        let path = entry.path();

        if path.is_dir()
        {
            walk( &path, base, pattern, out );
        }
        else if path.is_file()
        {
            let rel = match path.strip_prefix( base )
            {
                Ok( r ) => r.to_string_lossy().replace( '\\', "/" ),
                Err( _ ) => continue,
            };

            if matches( pattern.as_bytes(), rel.as_bytes() )
            {
                out.push( path.to_string_lossy().to_string() );
            }
        }
    }
}



/*
    Expand mask into list of files.
    Examples:
*/

//         ./a.txt            — single file
//         ./dir/             — all files under dir, recursive
//         ./src/**/*.rs      — files matching *.rs, recursive
//         ./src/*.rs         — files matching *.rs, top level only

pub fn scan( mask: &str ) -> Vec<String>
{
    let mut result = Vec::new();
    let expanded = expand_path( mask );
    let p = Path::new( &expanded );

    let has_wild = mask
    .chars()
    .any( |c| matches!( c, '*' | '?' | '[' | '{' ));

    /* Plain file */
    if !has_wild && p.is_file()
    {
        result.push( expanded );
        return result;
    }

    /* Directory without wildcards → everything recursive */
    if !has_wild && p.is_dir()
    {
        walk( p, p, "**/*", &mut result );
        result.sort();
        result.dedup();
        return result;
    }

    /* Glob mask — split into base (before first wildcard) and pattern */
    if has_wild
    {
        let cut = expanded
        .char_indices()
        .find( |(_, c)| matches!( c, '*' | '?' | '[' | '{' ) )
        .map( |(i, _)| i )
        .unwrap_or( 0 );

        let prefix = &expanded[ .. cut ];

        let base = match prefix.rfind( '/' )
        {
            Some( 0 ) => "/",
            Some( pos ) => &prefix[ .. pos ],
            None => ".",
        };

        let pattern = expanded[ base.len().. ]
        .trim_start_matches( '/' );

        let base_path = Path::new( base );

        if base_path.is_dir()
        {
            walk( base_path, base_path, pattern, &mut result );
            result.sort();
            result.dedup();
        }
    }

    result
}
