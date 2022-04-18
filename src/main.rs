// extern crate rust_salsa20;
// use rust_salsa20::{Salsa20, Key::Key32};
// pub mod salsa20;

#[path = "salsa20.rs"] mod salsa20;

fn main() {
    // let xx = salsa20::rotl(1, 3);
    // let yy = salsa20::quarterround([1,2,3,4]);
    // println!("{:?}, {:?}", xx, yy);

    // let rowround = salsa20::rowround([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
    // println!("rowround={:?}", rowround);
    
    // let columnround = salsa20::columnround([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
    // println!("columnround={:?}", columnround);
    
    // let doublerounds = salsa20::doublerounds([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16], 3);
    // println!("doublerounds={:?}", doublerounds);

    // let littleendian_inv32 = salsa20::littleendian_inv32(200);
    // println!("littleendian_inv32={:?}", littleendian_inv32);

    // let littleendian_inv64 = salsa20::littleendian_inv64(200);
    // println!("littleendian_inv64={:?}", littleendian_inv64);
    // let key: [u8; 32] = [
    //     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    //     11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    //     21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    // let v: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    // let ctx = salsa20::Context{
    //     k: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    //     v: vec![1, 2, 3, 4, 5, 6, 7, 8],
    //     n: 0,
    //     hashTbl: vec![0; 64],
    //     hashTblIdx: 64
    // };

    // let block = salsa20::block(&(&ctx.k), &(&ctx.v), 2);
    // println!("block");  
    // for i in 0..64 {
    //     println!("{:?}", block[i]);          
    // }

    // let ctx = salsa20::Context{
    //     k: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    //     v: vec![1, 2, 3, 4, 5, 6, 7, 8],
    //     n: 0,
    //     hashTbl: vec![]
    // };


    // let data: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10];
    // let (_, ctx1) = salsa20::crypt_bytes(&(&data), ctx);

    // let data1: Vec<u8> = vec![0; 1000];
    // let (_, ctx2) = salsa20::crypt_bytes(&(&data1),ctx1);

    // let (crypt_bytes, _) = salsa20::crypt_bytes(&(&data), ctx2);
    // println!("crypt_bytes= {:?}", crypt_bytes);  
 
}