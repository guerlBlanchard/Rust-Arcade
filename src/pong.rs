//
// EPITECH PROJECT, 2022
// Untitled (Workspace)
// File description:
// pong
//

use coffee::graphics::{
    Color, Font, Frame, Mesh, Point, Rectangle, Shape, Text, Window, WindowSettings,
};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, Input};

use coffee::load::Task;
use coffee::{Game, Timer};

use std::collections::HashSet;

pub fn Pong_player(){
    Pong::run(WindowSettings{
        title: String::from("Arcade"),
        size: (1280, 1024),
        resizable: false,
        maximized: false,
        fullscreen: false
    })
    .expect("an error has occured while loading the window");
}

struct Vector2f(f32, f32);

struct Bar {
    position: Vector2f,
    size: Vector2f
}

impl Bar {
    fn new(barposition: Vector2f) -> Bar {
        Bar {
            position: barposition,
            size: Vector2f(20.0, 100.0)
        }
    }
    
    fn draw_bar(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.position.0,
                y: self.position.1,
                width: self.size.0,
                height: self.size.1,
            }),
            Color::WHITE,
        );
        mesh.draw(&mut frame.as_target());
    }

    fn go_up(&mut self) {
        self.position.1 = self.position.1 - 10.0;
    }

    fn go_down(&mut self) {
        self.position.1 = self.position.1 + 10.0;
    }
}

struct Ball {
    position: Vector2f,
    size: Vector2f,
    velocity: Vector2f
}

impl Ball {
    fn new(ballposition: Vector2f) -> Ball {
        Ball {
            position: ballposition,
            size: Vector2f(10.0, 10.0),
            velocity: Vector2f(1.0, 1.0)
        }
    }
    
    fn draw_ball(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.position.0,
                y: self.position.1,
                width: self.size.0,
                height: self.size.1,
            }),
            Color::WHITE,
        );
        mesh.draw(&mut frame.as_target());
    }

    fn move_ball(&mut self, player1_info: &Bar, player2_info: &Bar) {
        self.position.0 += 2.0 * self.velocity.0;
        self.position.1 += 1.0 * self.velocity.1;
        if self.position.1 > 1014.0 || self.position.1 < 0.0 {
            self.velocity.1 *= -1.0
        }
        if self.position.0 + self.size.0 >= player2_info.position.0 && (self.position.1 > player2_info.position.1 && self.position.1 < player2_info.position.1 + player2_info.size.1) {
            self.velocity.0 *= -1.0;
        }
        if self.position.0 + self.size.0 <= player1_info.position.0 + player1_info.size.0 && (self.position.1 > player1_info.position.1 && self.position.1 < player1_info.position.1 + player1_info.size.1) {
            self.velocity.0 *= -1.0;
        }
    }
}

struct Pong {
    player1: Bar,
    player2: Bar,
    ball: Ball,
    score: u16
}

impl Game for Pong {
    type Input = CustomInput; // No input data
    type LoadingScreen = (); // No loading screen

    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
        for key in input.keys_pressed.iter() {
            match key {
                KeyCode::W => {
                    self.player1.go_up();
                }
                KeyCode::S => {
                    self.player1.go_down();
                }
                KeyCode::Up => {
                    self.player2.go_up();
                }
                KeyCode::Down => {
                    self.player2.go_down();
                }
                _ => (),
            }
        }
    }

    fn load(_window: &Window) -> Task<Pong> {
        let character_1 = Bar::new(Vector2f(10.0, 5.0));
        let character_2 = Bar::new(Vector2f(1250.0, 5.0));
        let ball = Ball::new(Vector2f(500.0, 500.0));
        Task::succeed(|| Pong {
            player1: character_1,
            player2: character_2,
            ball: ball,
            score: 0
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);
        self.player1.draw_bar(frame);
        self.player2.draw_bar(frame);
        self.ball.draw_ball(frame);
        self.ball.move_ball(&self.player1, &self.player2)

    }
}

struct CustomInput {
    cursor_position: Point,
    mouse_wheel: Point,
    keys_pressed: HashSet<keyboard::KeyCode>,
    text_buffer: String,
}

impl Input for CustomInput {
    fn new() -> CustomInput {
        CustomInput {
            cursor_position: Point::new(0.0, 0.0),
            mouse_wheel: Point::new(0.0, 0.0),
            keys_pressed: HashSet::new(),
            text_buffer: String::new(),
        }
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::TextEntered { character } => {
                    self.text_buffer.push(character);
                }
                keyboard::Event::Input { key_code, state } => match state {
                    input::ButtonState::Pressed => {
                        self.keys_pressed.insert(key_code);
                    }
                    input::ButtonState::Released => {
                        self.keys_pressed.remove(&key_code);
                    }
                },
            },
            _ => {}
        }
    }

    fn clear(&mut self) {
        self.text_buffer.clear();
    }
}