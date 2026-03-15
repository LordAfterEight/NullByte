use NullByte;

fn main() {
    let mut game = NullByte::core::Game::init();
    while game.window.window.is_open() {
        game.window.update();
    }
}
