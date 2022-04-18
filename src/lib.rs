use rustler::{Encoder, Env, Error, Term};
use rustler::schedule::SchedulerFlags;
use rustler::Binary;
use rustler::types::OwnedBinary;

#[path = "salsa20.rs"] mod salsa20;

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom error;
    }
}

rustler::rustler_export_nifs! {
    "Elixir.Salsa200",
    [
        ("crypt", 2, crypt, SchedulerFlags::DirtyCpu),
    ],
    None
}

fn crypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {

    let inBuff: Binary = args[0].decode()?;
    let (k, v, n, tbl) = args[1].decode::<(Binary, Binary, u64, Binary)>()?;
    let (outBuff, newCount, restTable) = salsa20::crypt_bytes(inBuff.as_slice(), k.as_slice(), v.as_slice(), n, tbl.as_slice());

    // println!("outBuff={:?}, {:?},", outBuff.len(), outBuff);
    // println!("restTable={:?}, {:?}", restTable.len(), restTable);

    let mut out_bin: OwnedBinary = OwnedBinary::new(outBuff.len()).unwrap();
    out_bin.as_mut_slice().copy_from_slice(&(&outBuff));

    let mut tbl_bin: OwnedBinary = OwnedBinary::new(restTable.len()).unwrap();
    tbl_bin.as_mut_slice().copy_from_slice(&(&restTable));

    

    return Ok((out_bin.release(env), (k, v, newCount, tbl_bin.release(env))).encode(env));
}

