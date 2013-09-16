extern mod ruby;

fn main() {
    println("Starting");
    ruby::start_ruby();
    ruby::run_ruby();
    ruby::stop_ruby();
}
