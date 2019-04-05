#![macro_use]

#[macro_export]
macro_rules! resultmake
{
    ($mdl:expr, $desc:expr) =>
    {{
        (($mdl & 0x1ff) | ($desc & 0x1fff) << 9) as u32
    }};
}

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