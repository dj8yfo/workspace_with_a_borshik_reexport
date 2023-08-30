use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, BorshSchema, PartialEq, Debug)]
struct B {
    x: u64,
    y: i32,
    c: C,
}

#[derive(BorshSerialize, BorshDeserialize, BorshSchema, PartialEq, Debug)]
enum C {
    C1,
    C2(u64),
    C3(u64, u64),
    C4 { x: u64, y: u64 },
    C5(D),
}

#[derive(BorshSerialize, BorshDeserialize, BorshSchema, PartialEq, Debug)]
struct D {
    x: u64,
}

#[cfg(test)]
mod tests {

    use super::{B, C, D};
    use borsh::{from_slice, to_vec};

    use std::collections::BTreeMap;

    macro_rules! map(
        () => { BTreeMap::new() };
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = BTreeMap::new();
                $(
                    m.insert($key.to_string(), $value);
                )+
                m
            }
         };
    );

    #[test]
    fn b_roundtrip() {
        let s = B {
            x: 3443,
            y: -2422,
            c: C::C5(D { x: 10 }),
        };

        let buf = to_vec(&s).unwrap();
        let actual_s = from_slice::<B>(&buf).expect("failed to deserialize a string");
        assert_eq!(actual_s, s);
    }

    use borsh::schema::{BorshSchema, Definition, Fields};

    #[test]
    fn b_schema() {
        let mut defs = Default::default();
        B::add_definitions_recursively(&mut defs);
        assert_eq!(
            map! {
            "B" => Definition::Struct{ fields: Fields::NamedFields(vec![
                    ("x".to_string(), "u64".to_string()),
                    ("y".to_string(), "i32".to_string()),
                    ("c".to_string(), "C".to_string())
                ]
                )},
            "C" => Definition::Enum { variants:  vec![
                    ("C1".to_string(), "CC1".to_string()), 
                    ("C2".to_string(), "CC2".to_string()), 
                    ("C3".to_string(), "CC3".to_string()), 
                    ("C4".to_string(), "CC4".to_string()), 
                    ("C5".to_string(), "CC5".to_string())
                ]},
            "CC1" => Definition::Struct{ fields: Fields::Empty },
            "CC2" => Definition::Struct{ fields: Fields::UnnamedFields(vec![
                    "u64".to_string(),
                ]
                ) },
            "CC3" => Definition::Struct{ fields: Fields::UnnamedFields(vec![
                    "u64".to_string(),
                    "u64".to_string(),
                ]
                ) },
            "CC4" => Definition::Struct{ fields: Fields::NamedFields(vec![
                    ("x".to_string(), "u64".to_string()),
                    ("y".to_string(), "u64".to_string()),
                ]
                )},
            "CC5" => Definition::Struct{ fields: Fields::UnnamedFields(vec![
                    "D".to_string(),
                ]
                ) },
            "D" => Definition::Struct{ fields: Fields::NamedFields(vec![
                    ("x".to_string(), "u64".to_string()),
                ]
                )}
            },
            defs
        );
    }
}
