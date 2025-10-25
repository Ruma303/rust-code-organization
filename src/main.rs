mod network;

fn main() {
  network::tcp::using_tcp();
  network::udp::using_udp();

  network::print_network();
}