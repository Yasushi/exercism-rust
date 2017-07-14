/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| encode(v)).collect()
}

fn encode(value: &u32) -> Vec<u8> {
    let mut v = *value;
    let mut r = Vec::new();
    while v > 0 {
        r.push((v & 0x7f) as u8);
        v = v >> 7;
    }
    if r.is_empty() {
        r.push(0u8);
    }
    println!("XXX {:?}", r);
    r.iter()
        .enumerate()
        .map(|(i, &b)| if i == 0 { b } else { b | 0x80 })
        .rev()
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut acc = 0u32;
    let mut count = 0;
    let mut result = Vec::new();
    for b in bytes.iter() {
        let v = (b & 0x7f) as u32;
        if count == 4 && acc >> 25 > 0 {
            // println!(
            //     "OVF {:x} {:x} {} {:?}",
            //     b,
            //     acc,
            //     count,
            //     result
            //         .iter()
            //         .map(|r| format!("{:x}", r))
            //         .collect::<Vec<_>>()
            // );
            return Err("overflow");
        }
        acc = acc << 7 | v;
        count += 1;
        if b & 0x80 == 0 {
            result.push(acc);
            acc = 0;
            count = 0;
        }
        // println!(
        //     "XXX {:x} {:x} {} {:?}",
        //     b,
        //     acc,
        //     count,
        //     result
        //         .iter()
        //         .map(|r| format!("{:x}", r))
        //         .collect::<Vec<_>>()
        // );
    }
    if count == 0 {
        Ok(result)
    } else {
        Err("incomplete")
    }
}
