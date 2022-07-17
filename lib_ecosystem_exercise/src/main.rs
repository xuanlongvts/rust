mod _1_arrayref;
mod _2_config;
mod _3_hex;
mod _4_hmac;

fn main() {
    println!("---------> 1. arrayref");
    _1_arrayref::_1_simple_case();
    _1_arrayref::_2_array_refs();
    _1_arrayref::_3_array_refs_with_const();

    println!("---------> 2. config");
    _2_config::_1_simple();
    _2_config::_2_env_list();

    println!("---------> 3. hex");
    _3_hex::encode_fn();
    _3_hex::decode_fn();
    _3_hex::encode_upper_fn();

    println!("---------> 4. hmac");
    _4_hmac::_1_anatomy_fun();
    _4_hmac::_2_verify_slice_fun();
}
