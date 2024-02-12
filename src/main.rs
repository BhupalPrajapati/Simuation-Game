#[warn(non_snake_case)]

struct Dog{
    id:u32,
    name:String,
    age : u32,
}

impl Dog {
    fn new(id: u32, name: String, age: u32) -> Self {
       Dog{id,name,age}
    }
}

trait Trade {
    type Pet;
    fn trade(&self, other: &Self::Pet) -> bool;
    
}

impl Trade for Dog{
    type Pet = Dog;
    fn trade(&self, other: &Self::Pet) -> bool {
        if self.id>other.id{
            println!("{} is Tradeted with {}",self.name,self.age);
            true
        }else {
            println!("{} is Not tradeted with {}",self.name,self.age);
            false
        }
    }

}

// here we define the supper trait by using the child trait colon parent trait if shows no implemenation
trait Playable:Trade{}   

// Empty Implementation of the trait
impl Playable for Dog{

}

fn main() {
    let dog1 = Dog::new(1, String::from("Shut"), 2);
    let dog2 = Dog::new(3,String::from("Manxe") , 4);
    dog1.trade(&dog1);
    dog1.trade(&dog2);

}
