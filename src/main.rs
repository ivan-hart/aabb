use macroquad::prelude::*;

const PLAYER_W: f32 = 100.0;
const PLAYER_H: f32 = 100.0;
const PLAYER_SPEED: f32 = 100.0;

#[derive(Copy, Clone)]
struct Player {
    rect: Rect,
    direction: Vec2,
}

impl Player {
    fn new(rect: Rect, direction: Vec2) -> Self {
        Self { rect, direction }
    }

    fn check_collision(this: Rect, other: Rect) -> bool {
        return this.overlaps(&other);
    }

    fn update(
        &mut self,
        left: KeyCode,
        up: KeyCode,
        right: KeyCode,
        down: KeyCode,
        other: Player,
        delta: f32,
    ) {
        if is_key_down(left) {
            self.direction.x = -1.0;
        } else if is_key_down(right) {
            self.direction.x = 1.0;
        } else {
            self.direction.x = 0.0;
        }
        if is_key_down(up) {
            self.direction.y = -1.0;
        } else if is_key_down(down) {
            self.direction.y = 1.0;
        } else {
            self.direction.y = 0.0;
        }

        let old_pos_rect: Rect = self.rect;

        if self.direction != Vec2::ZERO {

            self.rect.x += (self.direction.normalize().x * PLAYER_SPEED) * delta;
            self.rect.y += (self.direction.normalize().y * PLAYER_SPEED) * delta;

            if Player::check_collision(self.rect, other.rect) {
                self.rect = old_pos_rect;
            }
        }
    }

    fn draw(&mut self) {
        let r = self.rect;
        draw_rectangle(r.x, r.y, r.w, r.h, RED);
    }
}

#[macroquad::main("aabb")]
async fn main() {
    let mut player1 = Player::new(Rect::new(0.0, 0.0, PLAYER_W, PLAYER_H), Vec2::ZERO);
    let mut player2 = Player::new(Rect::new(200.0, 200.0, PLAYER_W, PLAYER_H), Vec2::ZERO);

    let mut delta: f32 = 0.0;
    loop {
        player1.update(
            KeyCode::A,
            KeyCode::W,
            KeyCode::D,
            KeyCode::S,
            player2,
            delta,
        );
        player2.update(
            KeyCode::Left,
            KeyCode::Up,
            KeyCode::Right,
            KeyCode::Down,
            player1,
            delta,
        );

        if is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(Color::from_hex(0x111133));

        player1.draw();
        player2.draw();

        delta = get_frame_time();
        next_frame().await;
    }
}
