mod binaries;
mod custom_commands;
mod publish_crate;
mod release_builds;
mod workspaces;

fn main() {
    release_builds::release_builds();
    publish_crate::publish_crate();
    workspaces::workspaces();
    binaries::binaries();
    custom_commands::custom_commands();
}
