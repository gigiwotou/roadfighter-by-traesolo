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
        let x = self.x - self.width / 2.0;
        let y = self.y - self.height / 2.0;
        
        // 赛车主体 - 红色
        draw_rectangle(x + 2.0, y + 5.0, self.width - 4.0, self.height - 10.0, Color::new(1.0, 0.2, 0.2, 1.0));
        
        // 赛车顶部 - 深红色
        draw_rectangle(x + 4.0, y + 8.0, self.width - 8.0, 15.0, Color::new(0.8, 0.0, 0.0, 1.0));
        
        // 车窗
        draw_rectangle(x + 6.0, y + 10.0, self.width - 12.0, 10.0, Color::new(0.6, 0.8, 1.0, 0.8));
        
        // 车轮 - 黑色
        draw_rectangle(x, y + 8.0, 4.0, 10.0, BLACK);
        draw_rectangle(x + self.width - 4.0, y + 8.0, 4.0, 10.0, BLACK);
        draw_rectangle(x, y + self.height - 18.0, 4.0, 10.0, BLACK);
        draw_rectangle(x + self.width - 4.0, y + self.height - 18.0, 4.0, 10.0, BLACK);
        
        // 尾灯 - 红色
        draw_rectangle(x + 4.0, y + self.height - 8.0, 6.0, 4.0, Color::new(1.0, 0.4, 0.4, 1.0));
        draw_rectangle(x + self.width - 10.0, y + self.height - 8.0, 6.0, 4.0, Color::new(1.0, 0.4, 0.4, 1.0));
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
        let color = if rand::gen_range(0, 2) == 0 { 
            Color::new(0.2, 0.4, 1.0, 1.0) 
        } else { 
            Color::new(0.4, 0.8, 1.0, 1.0) 
        };
        
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
        let x = self.x - self.width / 2.0;
        let y = self.y - self.height / 2.0;
        
        // 赛车主体
        draw_rectangle(x + 2.0, y + 5.0, self.width - 4.0, self.height - 10.0, self.color);
        
        // 赛车顶部
        let darker_color = Color::new(self.color.r * 0.7, self.color.g * 0.7, self.color.b * 0.7, 1.0);
        draw_rectangle(x + 4.0, y + 8.0, self.width - 8.0, 15.0, darker_color);
        
        // 车窗
        draw_rectangle(x + 6.0, y + 10.0, self.width - 12.0, 10.0, Color::new(0.6, 0.8, 1.0, 0.8));
        
        // 车轮 - 黑色
        draw_rectangle(x, y + 8.0, 4.0, 10.0, BLACK);
        draw_rectangle(x + self.width - 4.0, y + 8.0, 4.0, 10.0, BLACK);
        draw_rectangle(x, y + self.height - 18.0, 4.0, 10.0, BLACK);
        draw_rectangle(x + self.width - 4.0, y + self.height - 18.0, 4.0, 10.0, BLACK);
        
        // 尾灯
        draw_rectangle(x + 4.0, y + self.height - 8.0, 6.0, 4.0, Color::new(1.0, 0.4, 0.4, 1.0));
        draw_rectangle(x + self.width - 10.0, y + self.height - 8.0, 6.0, 4.0, Color::new(1.0, 0.4, 0.4, 1.0));
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
        // 燃料包外圈 - 深绿色
        draw_circle(self.x, self.y, self.size / 2.0, Color::new(0.0, 0.5, 0.0, 1.0));
        
        // 燃料包内圈 - 亮绿色
        draw_circle(self.x, self.y, self.size / 2.0 - 3.0, Color::new(0.0, 1.0, 0.3, 1.0));
        
        // 燃料包中心 - 白色高光
        draw_circle(self.x - 2.0, self.y - 2.0, self.size / 4.0, Color::new(1.0, 1.0, 0.8, 0.6));
        
        // "F" 字母
        draw_text("F", self.x - 5.0, self.y + 3.0, 16.0, WHITE);
    }

    fn is_off_screen(&self) -> bool {
        self.y > WINDOW_HEIGHT + 50.0
    }
}

fn draw_road() {
    // 草地背景 - 带深色条纹
    for i in 0..(WINDOW_HEIGHT / 20.0) as i32 {
        let y = i as f32 * 20.0;
        let color = if i % 2 == 0 {
            Color::new(0.1, 0.5, 0.1, 1.0)
        } else {
            Color::new(0.15, 0.55, 0.15, 1.0)
        };
        draw_rectangle(0.0, y, WINDOW_WIDTH, 20.0, color);
    }
    
    // 道路
    let road_left = (WINDOW_WIDTH - ROAD_WIDTH) / 2.0;
    draw_rectangle(road_left, 0.0, ROAD_WIDTH, WINDOW_HEIGHT, Color::new(0.3, 0.3, 0.35, 1.0));
    
    // 道路边缘线
    draw_rectangle(road_left, 0.0, 8.0, WINDOW_HEIGHT, WHITE);
    draw_rectangle(road_left + ROAD_WIDTH - 8.0, 0.0, 8.0, WINDOW_HEIGHT, WHITE);
    
    // 车道分隔线虚线
    let lane1_x = road_left + LANE_WIDTH;
    let lane2_x = road_left + LANE_WIDTH * 2.0;
    for i in 0..15 {
        let y = (i as f32 * 50.0) - 25.0;
        draw_rectangle(lane1_x - 2.0, y, 4.0, 30.0, Color::new(1.0, 1.0, 1.0, 0.6));
        draw_rectangle(lane2_x - 2.0, y, 4.0, 30.0, Color::new(1.0, 1.0, 1.0, 0.6));
    }
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
        // 中心黄色线
        draw_rectangle(WINDOW_WIDTH / 2.0 - 5.0, y - 20.0, 10.0, 40.0, Color::new(1.0, 0.9, 0.0, 1.0));
        // 中心白色高光
        draw_rectangle(WINDOW_WIDTH / 2.0 - 2.0, y - 15.0, 4.0, 30.0, Color::new(1.0, 1.0, 0.6, 0.4));
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
    // 分数背景
    draw_rectangle(5.0, 5.0, 130.0, 70.0, Color::new(0.0, 0.0, 0.0, 0.6));
    
    // 分数文本
    draw_text(&format!("Score: {}", state.score), 15.0, 30.0, 24.0, WHITE);
    
    // 燃料条背景
    draw_rectangle(15.0, 38.0, 110.0, 20.0, Color::new(0.2, 0.2, 0.2, 1.0));
    
    // 燃料条
    let fuel_width = (state.fuel / 100.0) * 106.0;
    let fuel_color = if state.fuel > 50.0 {
        Color::new(0.0, 1.0, 0.3, 1.0)
    } else if state.fuel > 25.0 {
        Color::new(1.0, 0.7, 0.0, 1.0)
    } else {
        Color::new(1.0, 0.2, 0.2, 1.0)
    };
    draw_rectangle(17.0, 40.0, fuel_width, 16.0, fuel_color);
    
    // 燃料文本
    draw_text(&format!("Fuel: {:.0}", state.fuel), 15.0, 85.0, 20.0, WHITE);
    
    if state.game_over {
        // 游戏结束背景
        draw_rectangle(WINDOW_WIDTH / 2.0 - 120.0, WINDOW_HEIGHT / 2.0 - 40.0, 240.0, 80.0, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_text("GAME OVER!", WINDOW_WIDTH / 2.0 - 90.0, WINDOW_HEIGHT / 2.0 + 10.0, 40.0, RED);
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
