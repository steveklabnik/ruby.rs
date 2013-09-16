extern mod ruby;

fn main() {
    println("Starting...");
    ruby::start_ruby();
    println("Running...");
    ruby::run_ruby();
    println("Stopping...");
    ruby::stop_ruby();
}
