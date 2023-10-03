pub mod iters{
    pub fn iter_intro(){
        let v1 = vec![1,2,3,4];
        let v1_iter = v1.iter();

        for val in v1_iter{
            println!("Got: {}",val)
        }
    }

    pub fn iter_adaptors(){
        let v1 = vec![1,2,3,4];

        let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

        assert_eq!(v2,vec![2,3,4,5]);
    }
}

#[derive(PartialEq,Debug)]
pub struct Shoe{
    size: u32,
    style: String,
}

mod shoes_mod{
    use super::Shoe;
    pub fn shoes_in_size(shoes: Vec<Shoe>,shoe_size: u32)-> Vec<Shoe>{
        shoes.into_iter().filter(|s|s.size==shoe_size).collect()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn filters_by_size(){
        let shoes = vec![
            Shoe{
                size:10,
                style: String::from("Sneaker"),
            },
            Shoe{
                size:09,
                style:String::from("sandal"),
            },
            Shoe{
                size:12,
                style:String::from("boot"),
            },
        ];

        let in_my_shoes = shoes_mod::shoes_in_size(shoes,10);

        assert_eq!(
            in_my_shoes,
            vec![
                Shoe{
                    size:10,
                    style: String::from("Sneaker"),
                },
            ]
        );
    }
}