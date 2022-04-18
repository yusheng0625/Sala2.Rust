pub struct Context {
    pub k: Vec<u8>,
    pub v: Vec<u8>,
    pub n: u64,
    pub hashTbl: Vec<u8>,
}


pub fn rotl(x: u32, r: u32) -> u32{
    return (x << r) | (x >> (32 - r));
}
pub fn sum(x: u32, y: u32) -> u32{
    let xx: u64 = x as u64 + y as u64;
    return xx as u32;
}

pub fn quarterround([y0, y1, y2, y3]: [u32; 4]) -> [u32; 4]
{
    let z1 = y1 ^ rotl(sum(y0, y3), 7);
    let z2 = y2 ^ rotl(sum(z1, y0), 9);
    let z3 = y3 ^ rotl(sum(z2, z1), 13);
    let z0 = y0 ^ rotl(sum(z3, z2), 18);
    return [z0, z1, z2, z3];
}

pub fn rowround([y0, y1, y2, y3, y4, y5, y6, y7, y8, y9, y10, y11, y12, y13, y14, y15]: [u32; 16]) -> [u32; 16]
{
    let [z0, z1, z2, z3] = quarterround([y0, y1, y2, y3]);
    let [z5, z6, z7, z4] = quarterround([y5, y6, y7, y4]);
    let [z10, z11, z8, z9] = quarterround([y10, y11, y8, y9]);
    let [z15, z12, z13, z14] = quarterround([y15, y12, y13, y14]);
    return [z0, z1, z2, z3, z4, z5, z6, z7, z8, z9, z10, z11, z12, z13, z14, z15];    
}

pub fn columnround([x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15]: [u32; 16]) -> [u32; 16]
{
    let [y0, y4, y8, y12] = quarterround([x0, x4, x8, x12]);
    let [y5, y9, y13, y1] = quarterround([x5, x9, x13, x1]);
    let [y10, y14, y2, y6] = quarterround([x10, x14, x2, x6]);
    let [y15, y3, y7, y11] = quarterround([x15, x3, x7, x11]);
    return [y0, y1, y2, y3, y4, y5, y6, y7, y8, y9, y10, y11, y12, y13, y14, y15];
}

pub fn doubleround(x: [u32; 16]) -> [u32; 16]
{
    return rowround(columnround(x));
}

pub fn doublerounds(x: [u32; 16], n: u32) -> [u32; 16]
{
    if n==0 {
       return x; 
    }
    return doublerounds(doubleround(x), n-1);
}

pub fn littleendian_inv32(i: u32) -> [u8; 4]
{
    return i.to_le_bytes();
    // let mut bs = [0u8; mem::size_of::<u32>()];
    // bs.as_mut().write_u32::<LittleEndian>(i).expect("Unable to write");
}

pub fn littleendian_inv64(i: u64) -> [u8; 8]
{
    return i.to_le_bytes();
}

pub fn pick_elements(zs: [u32; 16]) ->[[u8; 4]; 8]
{
    return [
        littleendian_inv32(zs[0]), 
        littleendian_inv32(zs[5]), 
        littleendian_inv32(zs[10]), 
        littleendian_inv32(zs[15]), 
        littleendian_inv32(zs[6]), 
        littleendian_inv32(zs[7]), 
        littleendian_inv32(zs[8]), 
        littleendian_inv32(zs[9]) 
        ];
}

pub fn s20_hash_rounds(xs :[u32; 16], n: u32) ->[u8; 64]
{
    let mut res: [u8; 64] = [0; 64];

    if n == 0 {
        for i in 0..xs.len() {
            let xx = littleendian_inv32(xs[i]);
            res[i * 4] = xx[0];
            res[i * 4 + 1] = xx[1];
            res[i * 4 + 2] = xx[2];
            res[i * 4 + 3] = xx[3];
        } 
        return res;    
    }

    let mut round = doublerounds(xs, 4);
    for i in 0..round.len(){
        round[i] = sum(round[i], xs[i]);
    }
    return s20_hash_rounds(round, n-1);
}


pub fn words_as_ints(b: [u8; 64]) -> [u32; 16]
{
    let mut arr: [u32; 16] = [0; 16];
    for i in 0..16 {
        let idx = i * 4;
        arr[i] = u32::from_le_bytes([b[idx], b[idx+1], b[idx+2], b[idx+3]]);
    }
    return arr;
}

pub fn s20_hash(b :[u8; 64], rounds: u32) -> [u8; 64]
{
    return s20_hash_rounds(words_as_ints(b), rounds);
}

pub fn expand(k: &[u8], n: &[u8]) -> [u8; 64]
{
    let mut res: [u8; 64] = [0; 64];
    let mut idx = 0;
    
    if k.len() == 16 && n.len() == 16 
    {
        let t0: [u8; 4] = [101, 120, 112, 97];
        let t1: [u8; 4] = [110, 100, 32, 49];
        let t2: [u8; 4] = [54, 45, 98, 121];
        let t3: [u8; 4] = [116, 101, 32, 107];

        res[idx] = t0[0];   res[idx+1] = t0[1]; res[idx+2] = t0[2];   res[idx+3] = t0[3];  idx += 4;
        for i in 0..16 {
            res[idx] = k[i];
            idx += 1;
        }
        res[idx] = t1[0];   res[idx+1] = t1[1]; res[idx+2] = t1[2];   res[idx+3] = t1[3];  idx += 4;
        for i in 0..16 {
            res[idx] = n[i];
            idx += 1;
        }
        res[idx] = t2[0];   res[idx+1] = t2[1]; res[idx+2] = t2[2];   res[idx+3] = t2[3];  idx += 4;
        for i in 0..16 {
            res[idx] = k[i];
            idx += 1;
        }
        res[idx] = t3[0];   res[idx+1] = t3[1]; res[idx+2] = t3[2];   res[idx+3] = t3[3];  idx += 4;        
    }
    else
    {
        let t0: [u8; 4] = [101, 120, 112, 97];
        let t1: [u8; 4] = [110, 100, 32, 51];
        let t2: [u8; 4] = [50, 45, 98, 121];
        let t3: [u8; 4] = [116, 101, 32, 107];
        res[idx] = t0[0];   res[idx+1] = t0[1]; res[idx+2] = t0[2];   res[idx+3] = t0[3];  idx += 4;
        for i in 0..16 {
            res[idx] = k[i];
            idx += 1;
        }
        res[idx] = t1[0];   res[idx+1] = t1[1]; res[idx+2] = t1[2];   res[idx+3] = t1[3];  idx += 4;
        for i in 0..16 {
            res[idx] = n[i];
            idx += 1;
        }
        res[idx] = t2[0];   res[idx+1] = t2[1]; res[idx+2] = t2[2];   res[idx+3] = t2[3];  idx += 4;
        for i in 16..32 {
            res[idx] = k[i];
            idx += 1;
        }
        res[idx] = t3[0];   res[idx+1] = t3[1]; res[idx+2] = t3[2];   res[idx+3] = t3[3];  idx += 4;
    }
    return res;
}

pub fn block(k: &[u8], v: &[u8], n: u64) -> [u8; 64]
{
    let mut vv: [u8; 16] = [0; 16];
    let xx = littleendian_inv64(n);
    for i in 0..8 {
        vv[i] = v[i];
        vv[i+8] = xx[i];        
    }

    let expand = expand(k, &vv);
    return s20_hash(expand, 1);
}


pub fn crypt_bytes(inBuff: &[u8], k: &[u8], v: &[u8], n: u64,  hTbl: &[u8]) -> (Vec<u8>, u64, Vec<u8>)
{
    let mut count = n;
    let mut tblIdx = 0;
    let mut outBuff: Vec<u8> = vec![0; inBuff.len()];
    let mut tbl: [u8; 64] = [0; 64];

    let mut tblLen = hTbl.len();
    for i in 0..tblLen{
        tbl[i] = hTbl[i];
    }

    for i in 0..inBuff.len(){        
        if tblIdx >= tblLen {            
            let table = block(&k, &v, count);
            for i in 0..table.len(){
                tbl[i] = table[i];                
            }  
            count = count + 1;
            tblIdx = 0;
            tblLen = table.len();
        }
        outBuff[i] = inBuff[i] ^ tbl[tblIdx];
        tblIdx = tblIdx + 1;
    }

    let mut restTable: Vec<u8> = vec![];
    for i in tblIdx..tblLen {
        restTable.push(tbl[i]);
    }
    return (outBuff, count, restTable);
}


// pub fn crypt_bytes(inBuff: &Vec<u8>, k: &Vec<u8>, v: &Vec<u8>, n: u64,  hTbl: &Vec<u8>) -> (Vec<u8>, u64, Vec<u8>)
// {
//     let mut count = n;
//     let mut tblIdx = 0;
//     let mut outBuff: Vec<u8> = vec![0; inBuff.len()];
//     let mut tbl: [u8; 64] = [0; 64];

//     let mut tblLen = hTbl.len();
//     for i in 0..tblLen{
//         tbl[i] = hTbl[i];
//     }

//     for i in 0..inBuff.len(){        
//         if tblIdx >= tblLen {
//             let table = block(&k, &v, count);
//             for i in 0..table.len(){
//                 tbl[i] = table[i];                
//             }        
//             count = count + 1;
//             tblIdx = 0;
//             tblLen = table.len();
//         }
//         outBuff[i] = inBuff[i] ^ tbl[tblIdx];
//         tblIdx = tblIdx + 1;
//     }

//     let mut restTable: Vec<u8> = vec![0; tblLen - tblIdx];
//     for i in tblIdx..tblLen {
//         restTable.push(tbl[i]);
//     }
//     return (outBuff, count, restTable);
// }

