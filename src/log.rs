use crate::colors::*;
use crate::State;
use crate::moment::Moment;
use crate::files::{ expand_path, ensure_directory };
use std::io::Write;


/*
    Type of messages
*/
pub enum Level
{
    Info,
    Warning,
    Trace,
    Debug,
    Error,
    Begin,
    End,
}



/*
    Declare logger
*/
pub struct Log
{
    /**************************************************************************
        Parameters
    */
    /* Line buffer */
    line: String,
    /* Current depth for begin-end sections */
    depth: usize,
    /* Last time */
    last_time: Moment,  
    /* File handle */
    file_handle: Option<std::fs::File>,
    
    /**************************************************************************
        Settings
    */

    /* Enable/disable logging */
    enabled: bool,
    /* File path, stdout if empty */
    file_path: String,
        
    color: bool,

    color_name: Color,
    color_value: Color,
    color_syntax: Color,
}



impl Log 
{
    /*
        Constructor
    */
    pub fn create() -> Self
    {
        let mut m = Moment::create();
        m.now();
    
        Self
        { 
            line: String::new(),
            depth: 0,
            last_time: m,
            file_handle: None,
            enabled: true,

            color: true,
            file_path: String::new(),
            color_value: Color::Green,
            color_syntax: Color::Gray,
            color_name: Color::Cyan,
        }
    }



    /*
        Begin of line
    */
    pub fn line_begin
    (
        &mut self, 
        level: Level, 
        msg: &str
    ) -> &mut Self 
    {
        /* Close line */
        self.eol();

        /* Moment calc */
        let mut now = Moment::create();
        now.now();
        let delta = now.interval( &self.last_time );
        self.last_time = now;

        /* Get color and symbol for current message */
        let (color_code, symbol) = match level
        {
            Level::Info => ( Color::Blue, "I" ),
            Level::Warning => ( Color::Yellow, "W" ),
            Level::Error => ( Color::Red, "X" ),
            Level::Trace => ( Color::Default, "~" ),
            Level::Debug => ( Color::White, "#" ),
            Level::Begin => ( Color::Green, ">" ),
            Level::End => ( Color::Green, "<" ),
        };

        let color_syntax = self.color_syntax;
        let depth = self.depth;

        self
        .color( Color::Gray )
        .text( &delta.to_string() )
        .text( " " )
        .color( color_code )
        .text( symbol )
        .text( " " )
        .color( color_syntax )
        .text( &".".repeat( depth * 2 ))
        .color( color_code )
        .text( msg )
        .color( Color::Default )
        .text( " " )
        ;

        self
    }



    /*
        End of the line
        Send line buffer to the out
    */
    pub fn eol( &mut self ) 
    -> &mut Self
    {
        if self.enabled
        {
            match self.file_handle.is_some()
            {
                true =>
                {
                    if let Some(f) = &mut self.file_handle
                    {
                        let _ = f.write_all(self.line.as_bytes());
                        let _ = f.write_all(b"\n");
                    }
                }
                false =>
                {
                    println!("{}", self.line);
                }
            }
            self.line.clear();
        }
        self
    }



    /*
        Text out
    */
    pub fn text
    (
        &mut self,
        msg: &str,
    )
    -> &mut Self
    {
        self.line.push_str( msg );
        self
    }



    /*
        Write color in to line buffer
    */
    pub fn color
    (
        &mut self, 
        color: Color
    ) -> &mut Self
    {
        if self.color 
        {
            self.text( color.to_str() );
        }
        self
    }



    /*
        Param out
    */
    pub fn prm <T: std::fmt::Display> 
    (
        &mut self, 
        key: &str, 
        val: T
    ) -> &mut Self
    {
        self.color( self.color_name );
        self.text( key );
        
        self.color( self.color_syntax );
        self.text( " = " );
        
        self.color(self.color_value);
        self.text( &val.to_string() );
        
        self.color(self.color_syntax);
        self.text( "; " );
        
        self.color( Color::Default );
        self
    }



    /**************************************************************************
        Messages
    */

    /*
        Begin section
    */
    pub fn begin
    (
        &mut self, 
        msg: &str
    ) -> &mut Self
    {
        self.line_begin( Level::Begin, msg );
        self.depth += 1;
        self
    }
    


    /*
        End section
    */
    pub fn end
    (
        &mut self, 
        msg: &str
    ) -> &mut Self
    {
        if self.depth > 0 
        {
            self.depth -= 1;
        }
        self.line_begin(Level::End, msg);
        self
    }



    pub fn trace
    (
        &mut self,
        msg: &str
    ) -> &mut Self
    {
        self.line_begin( Level::Trace, msg );
        self
    }  

    

    /*
        Info line
    */
    pub fn info
    (
        &mut self, 
        msg: &str
    ) -> &mut Self
    {
        self.line_begin( Level::Info, msg );
        self
    }



    /*
        Error line
    */
    pub fn error
    (
        &mut self,
        msg: &str
    )
    -> &mut Self
    {
        self.line_begin( Level::Error, msg );
        self
    }



    /*
        Warning line
    */
    pub fn warning
    (
        &mut self,
        msg: &str
    )
    -> &mut Self
    {
        self.line_begin
        (
            Level::Warning, 
            msg 
        );

        self
    }


    
    /*
        Debug line
    */
    pub fn debug
    (
        &mut self,
        msg: &str
    ) -> &mut Self
    {
        self.line_begin
        (
            Level::Debug, 
            msg 
        );

        self
    }  




    pub fn dump
    (
        &mut self, 
        title: &str, 
        text: &str
    )
    -> &mut Self 
    {
        self.begin( title ).eol();
        for line in text.lines()
        {
            self.text( line ).eol();
        }
        self.end( "End of dump" );
        self
    }


    /**************************************************************************
        Get
    */

    /* Return color */
    pub fn get_color( &self ) -> bool
    {
        self.color
    }
    

    /*
        Set color
    */
    pub fn set_color
    (
        &mut self, 
        val: bool
    ) -> &mut Self
    {
        self.color = val;
        self
    }



    pub fn get_syntax_color( &self ) -> Color 
    {
        self.color_syntax
    }



    pub fn set_enabled
    (
        &mut self, 
        val: bool
    ) -> &mut Self
    {
        self.enabled = val;
        self
    }




    /*
        Set file
    */
    pub fn set_file_path
    (
        &mut self, 
        path: &str
    ) 
    -> &mut Self 
    {
        if path.is_empty() 
        {
            /* Закрываем дескриптор */
            self.file_handle = None;
            self.file_path.clear();
        }
        else
        {
            self.file_path = expand_path( path );

            if let Err(e) = ensure_directory(&self.file_path) 
            {
                self.file_handle = None;
                eprintln!( "{}", e );
            }
            else
            {
                self.file_handle = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.file_path)
                .ok();
            }
        }
        
        self
    }



    /*
        Get file path
    */
    pub fn get_file_path(&self) -> &str
    {
        &self.file_path
    }



    pub fn dump_state
    (
        &mut self,
        state: &State
    )
    -> &mut Self
    {
        self
        .prm( "code", state.get_code() )
        .prm( "details", &state.get_details().to_string() )
        .eol()
    }
}

