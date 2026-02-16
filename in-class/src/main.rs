
   // let mut result : f32 = 0.0; // int -> int8, int16, int32, int64, int128
    //let x:i32 = 5; //float
    //result = result + x as f32; // no implicit conversion

    //println!("{}",result);

fn main() {
    let mut x:i32 = 5;
    x = 1.012; // you can't

    let x:f32 = x as f32 + 1.012;
    println!("{}",x)


}
