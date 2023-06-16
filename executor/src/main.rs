//! Executor with your game connected to it as a plugin.
use bleeping_fyrox::GameConstructor;
use fyrox::engine::executor::Executor;

fn main() {
    let mut executor = Executor::new();
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}
