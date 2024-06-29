pub mod macros1 {
    #[macro_export]
    macro_rules! ident_one {
        ($key:ident, $value:literal) => {
            let $key = $value;
            println!("ident_one: {:?}", $key);
        }
    }

    #[macro_export]
    macro_rules! ident_two {
        ($($key:ident, $value:literal); *) => {
            $(
                let $key = $value;
                println!("ident_two: {:?}", $key);
            )*
        }
    }

    #[macro_export]
    macro_rules! ident_three {
        ($key: ident, $value:tt) => {
            let $key = $value;
            println!("ident_three: {:?}", $key);
        };
    }
}

pub mod macros2 {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum OptRelNodeTyp {
        Projection,
        Filter,
    }

    pub trait RelNodeTyp: PartialEq + Eq + Clone + 'static + Send + Sync {}

    impl RelNodeTyp for OptRelNodeTyp {}

    pub enum RuleMatcher<T: RelNodeTyp> {
        /// Match a node of type `typ`.
        MatchAndPickNode {
            typ: T,
            children: Vec<Self>,
            pick_to: usize,
        },
        /// Match a node of type `typ`.
        MatchNode { typ: T, children: Vec<Self> },
        /// Match anything,
        PickOne { pick_to: usize, expand: bool },
        /// Match all things in the group
        PickMany { pick_to: usize },
        /// Ignore one
        IgnoreOne,
        /// Ignore many
        IgnoreMany,
    }

    #[macro_export]
    macro_rules! define_matcher {
        ( $pick_num:ident, ( $typ:expr $(, $children:tt )* ) ) => {
            RuleMatcher::MatchNode {
                typ: $typ,
                children: vec![
                    $( define_matcher!($pick_num, $children) ),*
                ],
            }
        };
        ( $pick_num:ident, [$pick_one:tt] ) => {
            RuleMatcher::PickOne {
                pick_to: { let x = $pick_num; $pick_num += 1; x },
                expand: true,
            }
        };
        ( $pick_num:ident, $pick_one:tt ) => {
            RuleMatcher::PickOne {
                pick_to: { let x = $pick_num; $pick_num += 1; x },
                expand: false,
            }
        };
    }

    #[macro_export]
    macro_rules! define_rule {
        ($($matcher:tt)+) => {
            let mut pick_num = 0;
            define_matcher!(pick_num, $($matcher)+);
            println!("{:#?}", pick_num);
        }
    }
}
