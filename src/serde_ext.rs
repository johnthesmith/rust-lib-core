/*
    SPDX-License-Identifier: MIT
    SPDX-FileCopyrightText: 2026 Still Swamp
*/

use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

pub trait SerdeExt
{
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
}

impl SerdeExt for YamlValue
{
    fn get_bool( &self, default: bool ) -> bool
    {
        self.as_bool().unwrap_or( default )
    }

    fn get_str( &self, default: &str ) -> String
    {
        self.as_str().unwrap_or( default ).to_string()
    }

    fn get_int( &self, default: u64 ) -> u64
    {
        self.as_u64().unwrap_or( default )
    }

    fn get_float( &self, default: f64 ) -> f64
    {
        self.as_f64().unwrap_or( default )
    }

    fn get_array( &self, default: Vec<JsonValue> ) -> Vec<JsonValue>
    {
        self.as_sequence()
            .map(|s| s.iter().map(|v| serde_json::to_value(v).unwrap()).collect())
            .unwrap_or( default )
    }

    fn get_object( &self, default: serde_json::Map<String, JsonValue> ) -> serde_json::Map<String, JsonValue>
    {
        self.as_mapping()
            .map(|m| {
                let mut map = serde_json::Map::new();
                for (k, v) in m {
                    if let Some(key) = k.as_str() {
                        map.insert( key.to_string(), serde_json::to_value(v).unwrap() );
                    }
                }
                map
            })
            .unwrap_or( default )
    }
}



impl SerdeExt for JsonValue
{
    fn get_bool( &self, default: bool ) -> bool
    {
        self.as_bool().unwrap_or( default )
    }



    fn get_str( &self, default: &str ) -> String
    {
        self.as_str().unwrap_or( default ).to_string()
    }



    fn get_int( &self, default: u64 ) -> u64
    {
        self.as_u64().unwrap_or( default )
    }



    fn get_float( &self, default: f64 ) -> f64
    {
        self.as_f64().unwrap_or( default )
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
}
