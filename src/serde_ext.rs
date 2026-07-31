/*
    SPDX-License-Identifier: MIT
    SPDX-FileCopyrightText: 2026 Still Swamp
*/

use serde_json::Value as JsonValue;


pub trait SerdeExt
{
    /*
        Dump JSON to console with pretty formatting
    */
    fn dump(&self);



    /*
        Get boolean value
    */
    fn get_bool
    (
        &self,
        /* Default value if not a boolean or missing */
        default: bool
    )
    -> bool;



    /*
        Get string value
    */
    fn get_str
    (
        &self,
        /* Default value if not a string or missing */
        default: &str
    )
    -> String;



    /*
        Get integer value (u64)
    */
    fn get_int
    (
        &self,
        /* Default value if not an integer or missing */
        default: u64
    )
    -> u64;



    /*
        Get size value (usize)
        Converts from u64 to usize
    */
    fn get_size
    (
        &self,
        /* Default value if not an integer or missing */
        default: usize
    )
    -> usize;




    /*
        Get float value (f64)
    */
    fn get_float
    (
        &self,
        /* Default value if not a float or missing */
        default: f64
    )
    -> f64;



    /*
        Get array value
    */
    fn get_array
    (
        &self,
        /* Default value if not an array or missing */
        default: Vec<JsonValue>
    )
    -> Vec<JsonValue>;



    /*
        Get object value
    */
    fn get_object
    (
        &self,
        /* Default value if not an object or missing */
        default: serde_json::Map<String, JsonValue>
    )
    -> serde_json::Map<String, JsonValue>;



    fn merge
    (
        &self,
        src: &Self
    )
    -> Self;



    /*
        Get string value by path
        Path is a json array of string keys or numeric indexes,
        e.g. the rule paths from the request contract config:
        [ "choices", 0, "message", "content" ]
    */
    fn get_by_path_string
    (
        &self,
        /* Path as array of string keys or numeric indexes */
        path: &JsonValue,
        /* Default value if the path does not resolve or is not a string */
        default: &str
    )
    -> String;



    /*
        Get integer value (u64) by path
        Path is a json array of string keys or numeric indexes,
        e.g. the rule paths from the request contract config:
        [ "usage", "prompt_tokens" ]
    */
    fn get_by_path_int
    (
        &self,
        /* Path as array of string keys or numeric indexes */
        path: &JsonValue,
        /* Default value if the path does not resolve or is not an integer */
        default: u64
    )
    -> u64;
}



impl SerdeExt for JsonValue
{
    fn dump( &self )
    {
        match serde_json::to_string_pretty( self )
        {
            Ok( json_str ) => println!( "{}", json_str ),
            Err( e ) => eprintln!( "Error dumping JSON: {}", e ),
        }
    }



    fn get_bool( &self, default: bool ) -> bool
    {
        match self
        {
            JsonValue::Bool( b ) => *b,
            JsonValue::String(s) =>
            {
                match s.to_lowercase().as_str()
                {
                    "true" | "1" | "yes" | "on" => true,
                    "false" | "0" | "no" | "off" => false,
                    _ => default,
                }
            }
            _ => default
        }
    }



    fn get_str
    (
        &self,
        default: &str
    ) -> String
    {
        self.as_str().unwrap_or( default ).to_string()
    }



    fn get_int
    (
        &self,
        default: u64
    )
    -> u64
    {
        match self
        {
            JsonValue::Number(n) => n.as_u64().unwrap_or(default),
            JsonValue::String(s) => s.parse::<u64>().unwrap_or(default),
            _ => default,
        }
    }



    fn get_size
    (
        &self,
        default: usize
    )
    -> usize
    {
        match self
        {
            JsonValue::Number(n)
                => n.as_u64().map(|v| v as usize).unwrap_or(default),
            JsonValue::String(s)
                => s.parse::<usize>().unwrap_or(default),
            _ => default,
        }
    }



    fn get_float
    (
        &self,
        default: f64
    )
    -> f64
    {
        match self
        {
            JsonValue::Number(n) => n.as_f64().unwrap_or(default),
            JsonValue::String(s) => s.parse::<f64>().unwrap_or(default),
            _ => default,
        }
    }



    fn get_array( &self, default: Vec<JsonValue> )
    -> Vec<JsonValue>
    {
        self.as_array().cloned().unwrap_or( default )
    }



    fn get_object
    (
        &self,
        default: serde_json::Map<String, JsonValue>
    )
    -> serde_json::Map<String, JsonValue>
    {
        self.as_object().cloned().unwrap_or( default )
    }



    fn merge
    (
        &self,
        src: &serde_json::Value
    ) -> serde_json::Value
    {
        fn merge_internal
        (
            dst: &serde_json::Value,
            src: &serde_json::Value
        ) -> serde_json::Value
        {
            match (dst, src)
            {
                (
                    serde_json::Value::Object(dst_obj),
                    serde_json::Value::Object(src_obj)
                ) =>
                {
                    let mut result = dst_obj.clone();
                    for (k, v) in src_obj
                    {
                        match result.get_mut(k)
                        {
                            Some(dst_v) =>
                            {
                                let merged = merge_internal(dst_v, v);
                                result.insert(k.clone(), merged);
                            }
                            None =>
                            {
                                result.insert(k.clone(), v.clone());
                            }
                        }
                    }
                    serde_json::Value::Object(result)
                }
                (
                    serde_json::Value::Array(dst_arr),
                    serde_json::Value::Array(src_arr)
                ) =>
                {
                    let mut result = dst_arr.clone();
                    result.extend(src_arr.clone());
                    serde_json::Value::Array(result)
                }
                (_, _) => src.clone(),
            }
        }

        merge_internal(self, src)
    }



    fn get_by_path_string
    (
        &self,
        path: &JsonValue,
        default: &str
    )
    -> String
    {
        let mut current = self;
        if let Some( arr ) = path.as_array()
        {
            for key in arr
            {
                let next = if let Some( idx ) = key.as_u64()
                {
                    current.as_array().and_then( |a| a.get( idx as usize ) )
                }
                else if let Some( key_str ) = key.as_str()
                {
                    current.as_object().and_then( |o| o.get( key_str ) )
                }
                else
                {
                    None
                };

                match next
                {
                    Some( v ) => current = v,
                    None => return default.to_string(),
                }
            }
        }
        current.get_str( default )
    }



    fn get_by_path_int
    (
        &self,
        path: &JsonValue,
        default: u64
    )
    -> u64
    {
        let mut current = self;
        if let Some( arr ) = path.as_array()
        {
            for key in arr
            {
                let next = if let Some( idx ) = key.as_u64()
                {
                    current.as_array().and_then( |a| a.get( idx as usize ) )
                }
                else if let Some( key_str ) = key.as_str()
                {
                    current.as_object().and_then( |o| o.get( key_str ) )
                }
                else
                {
                    None
                };

                match next
                {
                    Some( v ) => current = v,
                    None => return default,
                }
            }
        }
        current.get_int( default )
    }

}
