#[macro_export]
macro_rules! toggle{
    ($($x: ident), *) => {
        paste!{

        $(
            #[allow(unused)]
            pub fn [<r_with_$x>](&mut self) -> &mut Self {
               self.$x = true;
               self
            }

            #[allow(unused)]
            pub fn [<r_without_$x>](&mut self) -> &mut Self {
                self.$x = false;
                self
            }

            #[allow(unused)]
            pub fn [<with_$x>](mut self) -> Self {
               self.$x = true;
               self
            }

            #[allow(unused)]
            pub fn [<without_$x>](mut self) -> Self {
                self.$x = false;
                self
            }
        )*

        }
        };
}

#[macro_export]
macro_rules! cond_format {
    ($s: literal, $t: expr) => {
        paste! {

        pub fn [<format_$t>](&self, task: &Task) -> String {
        if self.$t {
            format!($s, task.$t)
        } else {
            "".to_string()
        }
        }
        }
    };
}

#[macro_export]
macro_rules! cond_opt_format {
    ($s: literal, $t: expr) => {
        paste! {
            pub fn [<format_$t>](&self, task: &Task) -> String {
                if self.$t && let Some(t) = &task.$t {
                    format!($s, t)
                } else {
                    "".to_string()
                }
            }
        }
    };
    ($s: literal, $t: expr, $f: expr) => {
        paste! {
            pub fn [<format_$t>](&self, task: &Task) -> String {
                if self.$t && let Some(t) = &task.$t {
                    format!($s, $f(t))
                } else {
                    "".to_string()
                }
            }
        }
    };
}
