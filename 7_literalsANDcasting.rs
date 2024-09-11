fn main(){
    let one = 1i32 ;
    // let two:i8 =12;
    // let three = 20_i8; // or 20i8 will also work
    // let four = 12.2f32;
    // println!("{}",three);
    // println!("{}",four);

    println!("{} bytes",std::mem::size_of_val(&one));
    let six=2i16;
    println!("{} bytes",std::mem::size_of_val(&six));

    //Now to Typecasts
    let seven = one as i16;
    println!("{} bytes",std::mem::size_of_val(&seven));
    



}


//This is literals in RUST. Literals means various ways by which we can represent how we are storing the data in any variable.


//Type-casting : RUST does not support implicit typecasting, i.e it does not change data type automatically.
//But it does support explicit, i.e manual type-casting.
//It is done as follows: