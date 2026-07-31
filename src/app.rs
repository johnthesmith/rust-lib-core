use serde_json::{json};
use crate::state::State;
use crate::log::Log;
use crate::SerdeExt;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;
use std::cell::Ref;


/*
    Application structure
*/
pub struct App
{
    /* State of application */
    pub state: State,
    /* Log subsystem */
    log: Rc<RefCell<Log>>,
    /* Config subsystem */
    pub config: serde_json::Value
}



/*
    Application implementation
*/
impl App
{
    /*
        Create and return application
    */
    pub fn create() -> Self
    {
        Self
        {
            state: State::ok(),
            log: Rc::new(RefCell::new(Log::create())),
            config: serde_json::Value::Null
        }
    }



    /*
        Read config
    */
    pub fn read_config
    (
        &mut self,
        path: &str
    )
    -> &mut Self
    {
        /* Check if config file exists */
        if !std::path::Path::new( path ).exists()
        {
            self.state.set_state
            (
                "config-not-found",
                json!({ "file": path })
            );
        }
        else
        {
            match std::fs::read_to_string( path )
            {
                Ok( content ) =>
                {
                    match serde_yaml::from_str::<serde_yaml::Value>( &content )
                    {
                        Ok( yaml_config ) =>
                        {
                            match serde_json::to_value( &yaml_config )
                            {
                                Ok( json_config ) =>
                                {
                                    self.config = json_config;

                                    /* Set log enabled */
                                    if let Some( enabled ) = self.config
                                        [ "application" ]
                                        [ "log" ]
                                        [ "enabled" ].as_bool()
                                    {
                                        self.get_log_mut().set_enabled( enabled );
                                    }
                                }
                                Err( e ) =>
                                {
                                    self.state.set_state
                                    (
                                        "json-conversion-error",
                                        json!({ "message": e.to_string(), "file": path })
                                    );
                                }
                            }
                        }
                        Err( e ) =>
                        {
                            self.state.set_state
                            (
                                "yaml-parse-error",
                                json!({ "message": e.to_string(), "file": path })
                            );
                        }
                    }
                }
                Err( e ) =>
                {
                    self.state.set_state
                    (
                        "cannot-read-config",
                        json!({ "message": e.to_string(), "file": path })
                    );
                }
            }
        }

        self
    }

    /*
        Return list of set names from config
    */
    pub fn get_sets( &mut self )
    -> Vec<String>
    {
        let mut result = Vec::new();

        if let Some(sets) = self.config
        ["application"]
        ["sets"]
        .as_object()
        {
            for key in sets.keys()
            {
                result.push(key.to_string());
            }
        }

        result
    }



    /*
        Read sets from config and merge into config
    */
    pub fn read_sets( &mut self ) -> &mut Self
    {
        let set_name = self.config[ "set" ].get_str( "" );
        if !set_name.is_empty()
        {
            if let Some( set ) = self.config
            [ "application" ]
            [ "sets" ]
            [ set_name ].as_object()
            {
                let set_value = serde_json::Value::Object(set.clone());
                self.config = self.config.merge( &set_value );
            }
        }
        self
    }



    pub fn read_cli( &mut self )
    -> &mut Self
    {
        let args: Vec<String> = std::env::args().collect();
        let mut map = serde_json::Map::new();

        let mut i = 1;
        while i < args.len()
        {
            let arg = &args[i];

            if arg.starts_with( "--" )
            {
                let parts: Vec<&str> = arg[2..].splitn(2, '=').collect();
                let key = parts[0];
                let value = if parts.len() > 1
                {
                    serde_json::Value::String(parts[1].to_string())
                }
                else if i + 1 < args.len() && !args[i + 1].starts_with( '-' )
                {
                    i += 1;
                    serde_json::Value::String( args[i].clone() )
                }
                else
                {
                    serde_json::Value::Bool( true )
                };
                map.insert( key.to_string(), value );
            }
            else if arg.starts_with( "-" )
            {
                let parts: Vec<&str> = arg[ 1..].splitn( 2, '=' ).collect();
                let key = parts[ 0 ];
                let value = if parts.len() > 1
                {
                    serde_json::Value::String( parts[1].to_string() )
                }
                else if i + 1 < args.len() && !args[ i + 1 ].starts_with( '-' )
                {
                    i += 1;
                    serde_json::Value::String( args[ i ].clone() )
                }
                else
                {
                    serde_json::Value::Bool( true )
                };
                map.insert( key.to_string(), value );
            }
            else
            {
                let pos = map.len();
                map.insert
                (
                    format!("_{}", pos),
                    serde_json::Value::String( arg.clone() )
                );
            }

            i += 1;
        }


        match &mut self.config
        {
            serde_json::Value::Object(existing) =>
            {
                for (k, v) in map {
                    existing.insert(k, v);
                }
            }
            serde_json::Value::Null =>
            {
                self.config = serde_json::Value::Object(map);
            }
            _other =>
            {
                self.get_log_mut().warning( "Cannot merge CLI into non-mapping config" );
            }
        }

        self
    }



    /*
        Config dump
    */
    pub fn dump_config( &mut self )
    -> &mut Self
    {
        if !&self.config.is_null()
        {
            self.get_log_mut().begin( "Config dump" );
            match serde_yaml::to_string(&self.config)
            {
                Ok(yaml_str) =>
                {
                    for line in yaml_str.lines()
                    {
                        self.get_log_mut().trace(line);
                    }
                }
                Err(e) =>
                {
                    self.get_log_mut().warning( "Cannot serialize config to YAML" );
                    self.get_log_mut().prm( "error", &e.to_string() );
                }
            }
            self.get_log_mut().end( "" );
        }
        else
        {
            self.get_log_mut().warning( "No config loaded" );
        }

        self
    }



    /**************************************************************************
        Setters and getteers
    */



    /*
        Return shared reference to log
    */
    pub fn get_log(&self)
    -> Ref<'_, Log>
    {
        self.log.borrow()
    }



    /*
        Return mutable reference to log (via RefCell)
    */
    pub fn get_log_mut( &mut self )
    -> RefMut<'_, Log>
    {
        self.log.borrow_mut()
    }



    pub fn get_log_rc(&self)
    -> Rc<RefCell<Log>>
    {
        self.log.clone()
    }
}

