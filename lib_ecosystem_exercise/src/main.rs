mod _1_arrayref;
mod _2_config;

fn main() {
    println!("---------> 1. arrayref");
    _1_arrayref::_1_simple_case();
    _1_arrayref::_2_array_refs();
    _1_arrayref::_3_array_refs_with_const();

    // config lib
    println!("---------> 2. config");
    _2_config::main();
}
