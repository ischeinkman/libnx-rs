#![macro_escape]

#[macro_export]
macro_rules! resultok
{
    ($rc:expr) =>
    {{
        if $rc != 0
        {
            return Err($rc);
        }
    }};
}

#[macro_export]
macro_rules! resultfinal
{
    ($rc:expr) =>
    {{
        return match $rc
        {
            0 => Ok(()),
            _ => Err($rc),
        };
    }};

    ($rc:expr, $val:expr) =>
    {{
        return match $rc
        {
            0 => Ok($val),
            _ => Err($rc),
        };
    }};
}