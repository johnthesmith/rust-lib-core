use serde_json::Value;



#[derive( Clone, serde::Serialize, serde::Deserialize )]



pub struct State 
{
    code: String,
    details: Value,
}



impl State 
{
    const OK_CODE: &'static str = "OK";



    /*
        Create OK State
    */
    pub fn ok() -> Self 
    {
        Self 
        {
            code: Self::OK_CODE.to_string(),
            details: Value::Null,
        }
    }



    /*
        Check if State is OK
    */
    pub fn is_ok
    (
        &self
    ) 
    -> bool 
    {
        self.code == Self::OK_CODE
    }



    /*
        Set State with code and details
    */
    pub fn set_state
    (
        &mut self,
        /* State code */
        code: &str,
        /* State details */
        details: Value,
    ) 
    -> &mut Self 
    {
        self.code = code.to_string();
        self.details = details;
        self
    }



    /*
        Get State code
    */
    pub fn get_code
    (
        &self
    ) 
    -> &str 
    {
        &self.code
    }



    /*
        Get State details
    */
    pub fn get_details
    (
        &self
    ) 
    -> &Value 
    {
        &self.details
    }



    /*
        Copy State to other
    */
    pub fn state_to
    (
        &self,
        /* destination State */
        other: &mut State,
    ) 
    -> &Self 
    {
        other.code = self.code.clone();
        other.details = self.details.clone();
        self
    }



    /*
        Copy State from other
    */
    pub fn state_from
    (
        &mut self,
        /* source State */
        other: &State,
    ) 
    -> &mut Self 
    {
        self.code = other.code.clone();
        self.details = other.details.clone();
        self
    }
}
