pub fn verse(n: u32) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else if n == 1 {
        format!(
            "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"
        )
    } else if n == 2 {
        format!(
            "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"
        )
    } else {
        format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n",
            n,
            n - 1
        )
    }
}

pub fn sing(a: u32, b: u32) -> String {
    (b..(a + 1)).rev().map(verse).collect::<Vec<_>>().join("\n")
}
