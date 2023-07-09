use ch8_collections::hash_maps;
use ch8_collections::vectors;
use ch8_collections::strings;
fn main() {
    vectors::initializtion();
    vectors::read_elements();
    vectors::mutability();
    vectors::iterating();
    vectors::enums();

    strings::creating();
    strings::updating();
    strings::concatenation();
    strings::iterating();

    hash_maps::creating_accessing();
    hash_maps::overwriting();
    hash_maps::updating_if_exists();
    hash_maps::updating_based_on_value();
}
