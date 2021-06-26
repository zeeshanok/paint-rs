use macroquad::prelude::*;

struct Dot {
    radius: f32,
    x: f32,
    y: f32,
    color: Color,
}

#[macroquad::main("Test")]
async fn main() {
    let mut dots: Vec<Dot> = vec![];
    let mut radius = 20f32;
    let mut color = BLUE;
    let mut colors = vec![WHITE, RED, BLUE, YELLOW, GREEN].into_iter().cycle();

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Q) {
            break;
        }

        // clear
        if is_key_down(KeyCode::C) {
            dots.clear();
        }

        // color
        if is_mouse_button_pressed(MouseButton::Right) {
            color = colors.next().unwrap();
        }

        let (x, y) = mouse_position();

        // click
        if is_mouse_button_down(MouseButton::Left) {
            dots.push(Dot {
                color,
                x,
                y,
                radius,
            });
        }
        draw_dots(&dots).await;

        // hover
        let shift_held = is_key_down(KeyCode::LeftShift);
        let (_, scroll) = mouse_wheel();
        radius += ((scroll as i32).signum() * if shift_held { 6 } else { 2 }) as f32;
        radius = radius.clamp(1.0, 200.0);
        draw_hover(x, y, WHITE, color, radius, 0.2 * radius).await;

        // text
        draw_text(
            format!("Size: {} {}\nDots: {}", radius / 2.0, if shift_held { "(3x)" } else { "" }, dots.len()).as_str(),
            20.0,
            30.0,
            40.0,
            color,
        );

        next_frame().await
    }
}

async fn draw_dots(dots: &Vec<Dot>) {
    for dot in dots.iter() {
        let Dot {
            x,
            y,
            radius,
            color,
        } = *dot;
        draw_circle(x, y, radius, color);
    }
}

async fn draw_hover(
    x: f32,
    y: f32,
    fill_color: Color,
    border_color: Color,
    radius: f32,
    thickness: f32,
) {
    draw_circle(x, y, radius, border_color);
    draw_circle(x, y, radius - thickness, fill_color);
}
