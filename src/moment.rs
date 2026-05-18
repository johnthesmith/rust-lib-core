/*
    Works with moment
*/
use std::time::{ SystemTime, UNIX_EPOCH };



/*
    Constant
*/
pub const MICROSECOND: u64 = 1;
pub const MILLISECOND: u64 = 1000;
pub const SECOND: u64 = 1_000_000;
pub const MINUTE: u64 = 60 * SECOND;
pub const HOUR: u64 = 60 * MINUTE;
pub const DAY: u64 = 24 * HOUR;



/*
    Moment of time
*/
pub struct Moment
{
    micros: i64,
}



/*
    Time interval
*/
pub struct Interval
{
    micros: i64,
}




/******************************************************************************
    Interval implementations
*/
impl Interval
{
    /*
        Constructor
    */
    pub fn create
    (
        micros: i64
    ) -> Self
    {
        Self { micros }
    }



    /*
        Get moment
    */
    pub fn get( &self ) -> i64
    {
        self.micros
    }



    /*
        Set moment
    */
    pub fn set
    (
        &mut self,
        val: i64
    ) -> &mut Self
    {
        self.micros = val;
        self
    }


    /*
        Convert to string
    */
    pub fn to_string( &self ) -> String
    {
        let abs_micros = self.micros.abs() as u64;
        let h = abs_micros / HOUR;
        let m = (abs_micros % HOUR) / MINUTE;
        let s = (abs_micros % MINUTE) / SECOND;
        let us = abs_micros % SECOND;
        
        if self.micros < 0
        {
            format!( "-{:02}:{:02}:{:02}.{:06}", h, m, s, us )
        } else {
            format!( "{:02}:{:02}:{:02}.{:06}", h, m, s, us )
        }
    }
}



/******************************************************************************
    Moment implementations
*/
impl Moment
{
    /*
        Constructor
    */
    pub fn create() -> Self
    {
        Self { micros: 0 }
    }



    /*
        Now
    */
    pub fn now( &mut self ) -> &mut Self
    {
        self.micros = SystemTime::now()
            .duration_since( UNIX_EPOCH )
            .unwrap_or_default()
            .as_micros() as i64;
        self
    }



    /*
        Add time
    */
    pub fn add
    (
        &mut self, 
        val: i64
    ) -> &mut Self
    {
        self.micros += val;
        self
    }



    /*
        Return delta moment
    */
    pub fn delta
    (
        &self, 
        other: &Self
    ) -> i64
    {
        self.micros - other.micros
    }



    /*
        Return interval
    */
    pub fn interval
    (
        &self, 
        other: &Moment
    ) -> Interval
    {
        Interval::create( self.micros - other.micros )
    }



    /**************************************************************************
        Setters and geters
    */

    /*
        Get moment
    */    
    pub fn get( &self ) -> i64
    {
        self.micros
    }



    /*
        Set moment
    */
    pub fn set
    (
        &mut self, 
        val: i64
    ) -> &mut Self
    {
        self.micros = val;
        self
    }



    pub fn to_string(&self) -> String 
    {
        let abs_micros = self.micros.abs() as u64;
        let h = abs_micros / HOUR;
        let m = (abs_micros % HOUR) / MINUTE;
        let s = (abs_micros % MINUTE) / SECOND;
        let us = abs_micros % SECOND;
        
        if self.micros < 0 {
            format!("-{:02}:{:02}:{:02}.{:06}", h, m, s, us)
        } else {
            format!("{:02}:{:02}:{:02}.{:06}", h, m, s, us)
        }
    }
}
