/*
    Path
*/

pub struct Path
{
    /* Key list */
    pub keys: Vec<String>,
}



impl Path
{
    /*
        Create empty path
    */
    #[allow(dead_code)]
    pub fn create() -> Self
    {
        Self
        {
            keys: Vec::new()
        }
    }



    /*
        Строка вида a/b/c в Path
    */
    pub fn from_string
    (
        /* строка */
        str: &str
    )
    -> Self
    {
        Self
        {
            keys: str.split('/').map( |s| s.to_string() ).collect()
        }
    }



    /*
        Path в строку вида a/b/c
    */
    #[allow(dead_code)]
    pub fn to_string
    (
        &self
    )
    -> String
    {
        self.keys.join( "/" )
    }



    /*
        Добавить ключ
    */
    #[allow(dead_code)]
    pub fn push
    (
        &mut self,
        /* ключ */
        part: &str
    )
    -> &mut Self
    {
        self.keys.push( part.to_string() );
        self
    }



    /*
        Убрать последний ключ
    */
    #[allow(dead_code)]
    pub fn pop
    (
        &mut self
    )
    -> &mut Self
    {
        self.keys.pop();
        self
    }



    /*
        Keys count
    */
    #[allow(dead_code)]
    pub fn len
    (
        &self
    )
    -> usize
    {
        self.keys.len()
    }



    /*
        Доступ к ключам
    */
    #[allow(dead_code)]
    pub fn keys
    (
        &self
    )
    -> &Vec<String>
    {
        &self.keys
    }
}

