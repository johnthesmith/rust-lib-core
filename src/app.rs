use serde_yaml::Value;
use crate::state::State;
use crate::log::Log;


/* 
    Application structure
*/
pub struct App
{
    /* State of application */
    pub state: State,
    /* Log subsystem */
    log: Log,
    /* Config subsystem */
    pub config: serde_yaml::Value
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
            log: Log::create(),
            config: serde_yaml::Value::Null,
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
        use serde_json::json;

        /* Check if config file exists */
        if !std::path::Path::new(path).exists()
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
                    match serde_yaml::from_str( &content )
                    {
                        Ok( config ) =>
                        {
                            self.config = config;

                            /* Set log enabled */
                            if let Some( enabled ) = self.config
                            [ "application" ]
                            [ "log" ]
                            [ "enabled" ].as_bool()
                            {
                                self.log.set_enabled( enabled );
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


    pub fn read_cli( &mut self ) 
    -> &mut Self 
    {
        let args: Vec<String> = std::env::args().collect();
        let mut map = serde_yaml::Mapping::new();

        let mut i = 1;
        while i < args.len() {
            let arg = &args[i];

            if arg.starts_with("--")
            {
                let parts: Vec<&str> = arg[2..].splitn(2, '=').collect();
                let key = parts[0];
                let value = if parts.len() > 1 {
                    Value::String(parts[1].to_string())
                } 
                else if i + 1 < args.len() && !args[i + 1].starts_with('-')
                {
                    i += 1;
                    Value::String(args[i].clone())
                }
                else
                {
                    Value::Bool( true )
                };
                map.insert( Value::String(key.to_string()), value);
            }
            else if arg.starts_with('-') && arg != "-" 
            {
                let key = &arg[1..];
                map.insert
                (
                    Value::String(key.to_string()), 
                    Value::Bool( true )
                );
            }
            else if !arg.starts_with('-') 
            {
                let pos = map.len();
                map.insert
                (
                    Value::String
                    (
                        format!("_{}", pos)), 
                        Value::String(arg.clone()
                    )
                );
            }

            i += 1;
        }


        match &mut self.config
        {
            Value::Mapping(existing) =>
            {
                for (k, v) in map {
                    existing.insert(k, v);
                }
            }
            Value::Null =>
            {
                self.config = Value::Mapping(map);
            }
            _other =>
            {
                self.log.warning( "Cannot merge CLI into non-mapping config" );
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
            self.log.begin( "Config dump" );
            match serde_yaml::to_string(&self.config)
            {
                Ok(yaml_str) =>
                {
                    for line in yaml_str.lines()
                    {
                        self.log.trace(line);
                    }
                }
                Err(e) =>
                {
                    self.log.warning( "Cannot serialize config to YAML" );
                    self.log.prm( "error", &e.to_string() );
                }
            }           
            self.log.end( "" );
        } 
        else 
        {
            self.log.warning( "No config loaded" );
        }
        
        self
    }



    /**************************************************************************
        Setters and getteers
    */

    /*
        Return log
    */
    pub fn get_log( &self ) 
    -> &Log 
    {
        &self.log
    }



    /*
        Return log
    */
    pub fn get_log_mut( &mut self ) 
    -> &mut Log 
    {
        &mut self.log
    }

}
