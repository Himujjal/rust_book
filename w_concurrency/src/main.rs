mod a_threads;
mod b_message_pass;
mod c_shared_state;
mod d_sync_send;

fn main() {
    a_threads::main();
    b_message_pass::main();
    c_shared_state::main();
    d_sync_send::main();
}
