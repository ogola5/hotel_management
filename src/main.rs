use std::io;



pub struct Visitorsname{
    rooms:i32,
    name:String,
    room_occupation:bool,
    amount:bool,


}


fn main(){
    let mut signed_rooms = String::new();
    
    println!("signed up for rooms");

io::stdin()
    .read_line(&mut signed_rooms)
    .expect("Failed to read line");
let signed_rooms: i32 = signed_rooms.trim().parse().expect("Input not an integer");
let available_rooms=30;
let remaining_rooms=(available_rooms-signed_rooms);

println! ("the result is: {}",remaining_rooms);
    //make a mutable string variable
    let mut first_name = String::new(); 
    println!("Enter the first name please");

    //to get input from the user
    io::stdin().read_line(&mut first_name); 
    let first_name: String= first_name.trim().parse().unwrap();
    
    // make a mutable string variable
    let mut second_name = String::new(); 
    println!("Enter the second name please");

    //to get input from the user
    io::stdin().read_line(&mut second_name); 

    // performing shadowing to convert the type 
    let second_name: String = second_name.trim().parse().unwrap();



    // make a mutable string variable
    let mut id= String::new(); 
    println!("Enter id no please");
    //to get input from the user
    io::stdin().read_line(&mut id); 
    // performing shadowing to convert the type
    let id: i32 = id.trim().parse().unwrap(); 
    

    // make a mutable string variable
    let mut age = String::new(); 
    println!("Enter age please");

    //to get input from the user
    io::stdin().read_line(&mut age); 

    // performing shadowing to convert the type 
    let age: i32= age.trim().parse().unwrap();
    
    
    // make a mutable string variable
    let mut county_of_origin = String::new(); 
    println!("Enter the county you are from please");

    //to get input from the user
    io::stdin().read_line(&mut county_of_origin); 

    // performing shadowing to convert the type
    let county_of_origin: String = county_of_origin.trim().parse().unwrap(); 
     
    

    println!("Hello,please {:?}of id {:?} ,age{:?} from county {:?}to proceed to room no: {:?}  ",first_name + &second_name ,id, age ,county_of_origin,signed_rooms); 
    //let rooms =100;
}