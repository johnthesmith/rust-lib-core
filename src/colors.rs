#[derive( Debug, Clone, Copy, PartialEq, Eq )]

pub enum Color
{
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    BoldBlack,
    BoldRed,
    BoldGreen,
    BoldYellow,
    BoldBlue,
    BoldMagenta,
    BoldCyan,
    BoldWhite,
    BoldGray,
    UnderlineBlack,
    UnderlineRed,
    UnderlineGreen,
    UnderlineYellow,
    UnderlineBlue,
    UnderlineMagenta,
    UnderlineCyan,
    UnderlineWhite,
    UnderlineGray,
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    BgGray,
    BgBrightRed,
    BgBrightGreen,
    BgBrightYellow,
    BgBrightBlue,
    BgBrightMagenta,
    BgBrightCyan,
    BgBrightWhite,
}

impl Color
{
    pub fn to_str(&self) -> &'static str
    {
        match self
        {
            Color::Default => "\x1b[0m",
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Gray => "\x1b[90m",
            Color::BoldBlack => "\x1b[30;1m",
            Color::BoldRed => "\x1b[31;1m",
            Color::BoldGreen => "\x1b[32;1m",
            Color::BoldYellow => "\x1b[33;1m",
            Color::BoldBlue => "\x1b[34;1m",
            Color::BoldMagenta => "\x1b[35;1m",
            Color::BoldCyan => "\x1b[36;1m",
            Color::BoldWhite => "\x1b[37;1m",
            Color::BoldGray => "\x1b[90;1m",
            Color::UnderlineBlack => "\x1b[30;4m",
            Color::UnderlineRed => "\x1b[31;4m",
            Color::UnderlineGreen => "\x1b[32;4m",
            Color::UnderlineYellow => "\x1b[33;4m",
            Color::UnderlineBlue => "\x1b[34;4m",
            Color::UnderlineMagenta => "\x1b[35;4m",
            Color::UnderlineCyan => "\x1b[36;4m",
            Color::UnderlineWhite => "\x1b[37;4m",
            Color::UnderlineGray => "\x1b[90;4m",
            Color::BgBlack => "\x1b[40m",
            Color::BgRed => "\x1b[41m",
            Color::BgGreen => "\x1b[42m",
            Color::BgYellow => "\x1b[43m",
            Color::BgBlue => "\x1b[44m",
            Color::BgMagenta => "\x1b[45m",
            Color::BgCyan => "\x1b[46m",
            Color::BgWhite => "\x1b[47m",
            Color::BgGray => "\x1b[100m",
            Color::BgBrightRed => "\x1b[101m",
            Color::BgBrightGreen => "\x1b[102m",
            Color::BgBrightYellow => "\x1b[103m",
            Color::BgBrightBlue => "\x1b[104m",
            Color::BgBrightMagenta => "\x1b[105m",
            Color::BgBrightCyan => "\x1b[106m",
            Color::BgBrightWhite => "\x1b[107m",
        }
    }



    pub fn colorize
    (
        start_color: Color,
        text: &str,
        end_color: Color,
        use_colors: bool
    ) -> String
    {
        if use_colors
        {
            format!("{}{}{}", start_color.to_str(), text, end_color.to_str())
        } else {
            text.to_string()
        }
    }


}
