mod a_box;
mod b_deref;
mod c_drop;
mod d_rc;
mod e_ref_cell;
mod f_ref_cycle;

fn main() {
    a_box::boxed();
    b_deref::deref();
    c_drop::dropped();
    d_rc::reference_counted();
    e_ref_cell::interior_mutability();
    f_ref_cycle::reference_cycle();
}
