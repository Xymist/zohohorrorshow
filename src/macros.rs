macro_rules! query_strings {
    ($($y:ident),* ) => (
        $(
            pub fn $y(mut self, param: &str) -> Self {
                let mut frp = Self {
                    auth_token: self.auth_token,
                    model_path: self.model_path,
                    params: self.params,
                };
                frp.params.push((stringify!($y), param));
                frp
            }
        )*
    )
}

macro_rules! query_groups {
    ($($y:ident),* ) => (
        $(
            pub fn $y(mut self, param: &[&str]) -> Self {
                let mut frp = Self {
                    auth_token: self.auth_token,
                    model_path: self.model_path,
                    params: self.params,
                };
                frp.params.push((stringify!($y), format!("%5B{}%5D", param.join(","))));
                frp
            }
        )*
    )
}
