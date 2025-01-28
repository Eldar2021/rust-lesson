#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_size(list: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    let res = list.into_iter().filter(|e| e.size == size).collect();
    println!("Your size shois is: {:?}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let list = vec![
            Shoe {
                size: 42,
                style: String::from("Adidas"),
            },
            Shoe {
                size: 41,
                style: String::from("Nike"),
            },
            Shoe {
                size: 37,
                style: String::from("Puma"),
            },
            Shoe {
                size: 36,
                style: String::from("Asrtan"),
            },
            Shoe {
                size: 41,
                style: String::from("Cucci"),
            },
        ];

        let result = shoes_in_size(list, 41);

        assert_eq!(
            result,
            vec![
                Shoe {
                    size: 41,
                    style: String::from("Nike"),
                },
                Shoe {
                    size: 41,
                    style: String::from("Cucci"),
                },
            ]
        )
    }
}
