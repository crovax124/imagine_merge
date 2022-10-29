fn get_nth_arg(n: usize)-> String{
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args{
  pub  image_1: String,
    pub   image_2: String,
    pub   output: String
}

impl Args {
   pub fn new() -> Self{
        Args{
            image_1: String::new(),
            image_2: String::new(),
            output: String::new(),
        }
    }
    pub fn create_from_arg() -> Self{
        Args{
            image_1:get_nth_arg(1),
            image_2:get_nth_arg(2),
            output :get_nth_arg(3),
        }
    }
}