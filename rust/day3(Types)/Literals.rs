///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// Numeric literals can be type annotated by adding the type as a suffix. As an example, to specify that the literal 42 should have the type i32, write 42i32.
/// The type of unsuffixed numeric literals will depend on how they are used. If no constraint exists, the compiler will use i32 for integers, and f64 for floating-point numbers.
/// 
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


fn main() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
