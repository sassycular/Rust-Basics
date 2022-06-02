// dependencies: rand = "0.8.5"
use rand::{self,Rng};

fn main(){
  let sky = vec!["cloudy", "sunny", "rainy"];
  // can be "cloudy", "sunny", "rainy"
  let temperature = vec!["warm", "cold", "freezing"];
  // can be "warm", "cold", "freezing"
  let a=(sky[rand::thread_rng().gen_range(0..sky.len())],temperature[rand::thread_rng().gen_range(0..temperature.len())]);
  match a {
    ("cloudy", "warm") => println!("It's cloudly but it's warm"),
    ("cloudy", "cold") => println!("Pleasant weather"),
    ("cloudy", "freezing") => println!("It's freezing outside"),
    ("sunny", "warm") => println!("It's hot sunny day"),
    ("cloudy", "cold") => println!("It's a cold sunny day"),
    ("rainy", "warm") => println!("It's raining"),
    ("rainy", "cold") => println!("It's raining and it's cold"),
    ("rainy", "freezing") => println!("It's raining and it's freezing"),
    _ => println!("not sure what to do?")
     // TODO Match the sky and temperature and print the appropriate message
     /*
        cloudly + warm = It's cloudly but it's warm
        cloudly + cold = Plesant weather
        cloudly + freezing =It's freezing outside
        sunny + warm = It's hot sunny day
        sunny + cold = It's a cold sunny day
        rainy + warm = It's raining
        rainy + cold = It's raining and it's cold
        rainy + freezing = It's raining and it's freezing
        others = Not sure what to do
     */
    }
}
