use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 600.0;
const ROAD_WIDTH: f32 = 300.0;
const LANE_WIDTH: f32 = 100.0;
const PLAYER_SPEED: f32 = 5.0;
const ENEMY_SPEED: f32 = 4.0;
const ROAD_SCROLL_SPEED: f32 = 6.0;

struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct Enemy {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

struct Fuel {
    x: f32,
    y: f32,
    size: f32,
}

struct GameState {
    score: u32,
    fuel: f32,
    game_over: bool,
    road_lines: Vec<f32>,
    enemies: Vec<Enemy>,
    fuels: Vec<Fuel>,
    last_enemy_spawn: f64,
    last_fuel_spawn: f64,
}

impl GameState {
    fn new() -> Self {
        let mut road_lines = Vec::new();
        for i in 0..10 {
            road_lines.push(-40.0 + i as f32 * 80.0);
        }
        
        GameState {
            score: 0,
            fuel: 100.0,
            game_over: false,
            road_lines,
            enemies: Vec::new(),
            fuels: Vec::new(),
            last_enemy_spawn: 0.0,
            last_fuel_spawn: 0.0,
        }
    }
}

impl Player {
    fn new() -> Self {
        Player {
            x: WINDOW_WIDTH / 2.0,
            y: WINDOW_HEIGHT - 150.0,
            width: 30.0,
            height: 50.0,
        }
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.x -= PLAYER_SPEED;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.x += PLAYER_SPEED;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.y -= PLAYER_SPEED;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.y += PLAYER_SPEED;
        }

        let road_left = (WINDOW_WIDTH - ROAD_WIDTH) / 2.0;
        let road_right = (WINDOW_WIDTH + ROAD_WIDTH) / 2.0;
        self.x = self.x.clamp(road_left + 20.0, road_right - 20.0);
        self.y = self.y.clamp(50.0, WINDOW_HEIGHT - 50.0);
    }

    fn draw(&self) {
        draw_rectangle(
            self.x - self.width / 2.0,
            self.y - self.height / 2.0,
            self.width,
            self.height,
            RED,
        );
    }

    fn collides_with(&self, other: &Enemy) -> bool {
        self.x - self.width / 2.0 < other.x + other.width / 2.0
            && self.x + self.width / 2.0 > other.x - other.width / 2.0
            && self.y - self.height / 2.0 < other.y + other.height / 2.0
            && self.y + self.height / 2.0 > other.y - other.height / 2.0
    }

    fn collides_with_fuel(&self, fuel: &Fuel) -> bool {
        let dx = self.x - fuel.x;
        let dy = self.y - fuel.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < (self.width / 2.0 + fuel.size / 2.0)
    }
}

impl Enemy {
    fn new(x: f32) -> Self {
        let color = if rand::gen_range(0, 2) == 0 { BLUE } else { SKYBLUE };
        
        Enemy {
            x,
            y: -50.0,
            width: 30.0,
            height: 50.0,
            color,
        }
    }

    fn update(&mut self) {
        self.y += ENEMY_SPEED;
    }

    fn draw(&self) {
        draw_rectangle(
            self.x - self.width / 2.0,
            self.y - self.height / 2.0,
            self.width,
            self.height,
            self.color,
        );
    }

    fn is_off_screen(&self) -> bool {
        self.y > WINDOW_HEIGHT + 50.0
    }
}

impl Fuel {
    fn new(x: f32) -> Self {
        Fuel {
            x,
            y: -50.0,
            size: 25.0,
        }
    }

    fn update(&mut self) {
        self.y += ROAD_SCROLL_SPEED;
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.size / 2.0, GREEN);
    }

    fn is_off_screen(&self) -> bool {
        self.y > WINDOW_HEIGHT + 50.0
    }
}

fn draw_road() {
    draw_rectangle(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, DARKGREEN);
    draw_rectangle((WINDOW_WIDTH - ROAD_WIDTH) / 2.0, 0.0, ROAD_WIDTH, WINDOW_HEIGHT, GRAY);
}

fn update_road_lines(state: &mut GameState) {
    for y in &mut state.road_lines {
        *y += ROAD_SCROLL_SPEED;
        if *y > WINDOW_HEIGHT + 50.0 {
            *y = -50.0;
        }
    }
}

fn draw_road_lines(state: &GameState) {
    for &y in &state.road_lines {
        draw_rectangle(WINDOW_WIDTH / 2.0 - 5.0, y - 20.0, 10.0, 40.0, YELLOW);
    }
}

fn spawn_enemies(state: &mut GameState, current_time: f64) {
    if current_time - state.last_enemy_spawn > 1.5 {
        let lane = rand::gen_range(0, 3);
        let x = WINDOW_WIDTH / 2.0 + (lane as f32 - 1.0) * LANE_WIDTH;
        
        state.enemies.push(Enemy::new(x));
        state.last_enemy_spawn = current_time;
    }
}

fn spawn_fuels(state: &mut GameState, current_time: f64) {
    if current_time - state.last_fuel_spawn > 5.0 {
        let lane = rand::gen_range(0, 3);
        let x = WINDOW_WIDTH / 2.0 + (lane as f32 - 1.0) * LANE_WIDTH;
        
        state.fuels.push(Fuel::new(x));
        state.last_fuel_spawn = current_time;
    }
}

fn update_enemies(state: &mut GameState) {
    state.enemies.retain_mut(|enemy| {
        enemy.update();
        !enemy.is_off_screen()
    });
}

fn update_fuels(state: &mut GameState) {
    state.fuels.retain_mut(|fuel| {
        fuel.update();
        !fuel.is_off_screen()
    });
}

fn check_collisions(player: &Player, state: &mut GameState) {
    for enemy in &state.enemies {
        if player.collides_with(enemy) {
            state.game_over = true;
        }
    }
    
    state.fuels.retain(|fuel| {
        if player.collides_with_fuel(fuel) {
            state.fuel = (state.fuel + 30.0).min(100.0);
            false
        } else {
            true
        }
    });
}

fn update_game_state(state: &mut GameState) {
    state.score += 1;
    state.fuel -= 0.02;
    
    if state.fuel <= 0.0 {
        state.game_over = true;
    }
}

fn draw_ui(state: &GameState) {
    draw_text(&format!("Score: {}", state.score), 10.0, 30.0, 24.0, WHITE);
    draw_text(&format!("Fuel: {:.1}", state.fuel), 10.0, 60.0, 24.0, WHITE);
    
    if state.game_over {
        draw_text("GAME OVER!", WINDOW_WIDTH / 2.0 - 100.0, WINDOW_HEIGHT / 2.0, 40.0, RED);
    }
}

#[macroquad::main("Road Fighter")]
async fn main() {
    request_new_screen_size(WINDOW_WIDTH, WINDOW_HEIGHT);
    
    let mut player = Player::new();
    let mut state = GameState::new();
    
    loop {
        clear_background(BLACK);
        
        if !state.game_over {
            player.update();
            update_road_lines(&mut state);
            spawn_enemies(&mut state, get_time());
            spawn_fuels(&mut state, get_time());
            update_enemies(&mut state);
            update_fuels(&mut state);
            check_collisions(&player, &mut state);
            update_game_state(&mut state);
        }
        
        draw_road();
        draw_road_lines(&state);
        
        for enemy in &state.enemies {
            enemy.draw();
        }
        
        for fuel in &state.fuels {
            fuel.draw();
        }
        
        player.draw();
        draw_ui(&state);
        
        next_frame().await;
    }
}
