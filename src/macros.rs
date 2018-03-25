macro_rules! query_strings {
    ($x:ident; $($y:ident),* ) => (
        $(
            pub fn $y(mut self, param: &str) -> $x<'a> {
                self.path = format!("{}&{}={}", self.path, stringify!($y), param);
                self
            }
        )*
    )
}

macro_rules! query_groups {
    ($x:ident; $($y:ident),* ) => (
        $(
            pub fn $y(mut self, param: &[&str]) -> $x<'a> {
                self.path = format!("{}&{}=[{}]", self.path, stringify!($y), param.join(","));
                self
            }
        )*
    )
}
