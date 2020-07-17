mod a_box;
mod b_deref;
mod c_drop;
mod d_ref_smart_pointer;
mod e_ref_cell;

fn main() {
    a_box::box_func();
    b_deref::_deref();
    c_drop::_drop_main();
    d_ref_smart_pointer::_ref_smart_pointers_main();
    e_ref_cell::ref_cell_main();
}
