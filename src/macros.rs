macro_rules! query_strings {
    ($x:ident; $($y:ident),* ) => (
        $(
            pub fn $y(&mut self, param: &str) {
                self.path = format!("{}&{}={}", self.path, stringify!($y), param)
            }
        )*
    )
}

macro_rules! query_groups {
    ($x:ident; $($y:ident),* ) => (
        $(
            pub fn $y(&mut self, param: &[&str]) {
                self.path = format!("{}&{}=[{}]", self.path, stringify!($y), param.join(","))
            }
        )*
    )
}