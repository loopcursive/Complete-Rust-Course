mod library;


use library::math::rec;
use library::server::database::connect as db_connect;
use library::server::network::connect as net_connect;

fn main(){
    library::math::rec();
    rec();
    db_connect();
    net_connect();
}