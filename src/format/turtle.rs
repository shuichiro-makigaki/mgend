use crate::models::regex;
use std::collections::HashSet;
use std::fmt::Display;
use std::io;
use std::io::Write;

pub trait ToTurtle {
    fn to_ttl(&self) -> io::Result<String>;

    fn string<T: AsRef<str>>(&self, str: T) -> String {
        format!("\"{}\"", str.as_ref())
    }

    fn pname<T: AsRef<str>>(&self, str: T) -> String {
        // ~.-!$&'()*+,;=/?#@%_
        let r = regex!(r"([\.,/#])");
        format!("{}", r.replace_all(str.as_ref(), "\\$1"))
    }

    fn write_vec<F, T, D: Display, S: AsRef<str>>(
        &self,
        buf: &mut Vec<u8>,
        vec: &[T],
        pred: S,
        func: F,
    ) -> io::Result<()>
    where
        F: Fn(&T) -> D,
    {
        for (i, x) in vec.iter().enumerate() {
            if i == 0 {
                write!(buf, " ;\n  {} {}", pred.as_ref(), func(x))?;
            } else {
                write!(buf, " ,\n    {}", func(x))?;
            }
        }

        Ok(())
    }

    fn write_set<F, T, D: Display, S: AsRef<str>>(
        &self,
        buf: &mut Vec<u8>,
        vec: &HashSet<T>,
        pred: S,
        func: F,
    ) -> io::Result<()>
    where
        F: Fn(&T) -> D,
    {
        for (i, x) in vec.iter().enumerate() {
            if i == 0 {
                write!(buf, " ;\n  {} {}", pred.as_ref(), func(x))?;
            } else {
                write!(buf, " ,\n    {}", func(x))?;
            }
        }

        Ok(())
    }
}
