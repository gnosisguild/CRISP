mod index;
mod rounds;
mod ciphernode;
mod voting;
mod state;

use router::Router;

pub fn setup_routes(router: &mut Router) {
    index::setup_routes(router);
    rounds::setup_routes(router);
    ciphernode::setup_routes(router);
    voting::setup_routes(router);
    state::setup_routes(router);
}