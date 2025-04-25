pub fn first_fifty_even_square() -> Vec<i32> {
    let mut store : Vec<i32> = Vec::new();
    for i in 2 ..=100{
        if i %2 == 0 {
            store.push(i*i);
        }
    }
    store
}