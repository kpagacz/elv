use aoc_elf::aoc_api;
use aoc_elf::configuration;

fn main() {
    let configuration = configuration::get_config();
    let _client = aoc_api::prepare_http_client(&configuration);
}
