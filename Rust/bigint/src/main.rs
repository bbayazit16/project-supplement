use bigint::math::*;

fn main() {

    let start = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();

    let x: Z = "2420312732137280740252082039239213219382903182100382193192421102".try_into().unwrap();

    let y: Z = "2412104204720487120487102470128741082740182140128412074122".try_into().unwrap();

    let end = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();

    println!("{}", x * y);
    println!("Took {:?}", end - start);
}
