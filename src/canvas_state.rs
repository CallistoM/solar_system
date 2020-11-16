use iced::{canvas, window, Color, Point, Size, Vector};

use std::time::Instant;

#[derive(Debug)]
pub struct State {
    start: Instant,
    current: Instant,
    stars: Vec<(Point, f32)>,
}

impl State {
    const SUN_RADIUS: f32 = 70.0;
    const ORBIT_RADIUS: f32 = 150.0;
    const MARS_ORBIT_RADIUS: f32 = 225.0;
    const EARTH_RADIUS: f32 = 12.0;
    const MOON_RADIUS: f32 = 4.0;
    const MOON_DISTANCE: f32 = 28.0;
    const MARS_RADIUS: f32 = 15.0;
    pub fn new() -> State {
        let now = Instant::now();
        let (width, height) = window::Settings::default().size;

        State {
            start: now,
            current: now,
            stars: {
                use rand::Rng;

                let mut rng = rand::thread_rng();

                (0..100)
                    .map(|_| {
                        (
                            Point::new(
                                rng.gen_range(0.0, width as f32),
                                rng.gen_range(0.0, height as f32),
                            ),
                            rng.gen_range(0.5, 1.0),
                        )
                    })
                    .collect()
            },
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.current = now;
    }
}

impl canvas::Drawable for State {
    fn draw(&self, frame: &mut canvas::Frame) {
        use canvas::{Path, Stroke};
        use std::f32::consts::PI;

        let center = frame.center();

        let space = Path::rectangle(Point::new(0.0, 0.0), frame.size());

        let stars = Path::new(|path| {
            for (p, size) in &self.stars {
                path.rectangle(*p, Size::new(*size, *size));
            }
        });

        let sun = Path::circle(center, Self::SUN_RADIUS);
        let orbit = Path::circle(center, Self::ORBIT_RADIUS);
        let orbit_mars = Path::circle(center, Self::MARS_ORBIT_RADIUS);

        frame.fill(&space, Color::BLACK);
        frame.fill(&stars, Color::WHITE);
        frame.fill(&sun, Color::from_rgb8(0xF9, 0xD7, 0x1C));
        frame.stroke(
            &orbit,
            Stroke {
                width: 1.0,
                color: Color::from_rgba8(0, 153, 255, 0.1),
                ..Stroke::default()
            },
        );
        
        frame.stroke(
            &orbit_mars,
            Stroke {
                width: 1.0,
                color: Color::from_rgba8(193, 68, 14, 0.1),
                ..Stroke::default()
            },
        );

        let elapsed = self.current - self.start;
        let elapsed_seconds = elapsed.as_secs() as f32 - 44.0;
        let elapsed_millis = elapsed.subsec_millis() as f32 - 44.0;

        frame.with_save(|frame| {
            frame.translate(Vector::new(center.x, center.y));
            frame.rotate(
                (2.0 * PI / 60.0) * elapsed_seconds + (2.0 * PI / 60_000.0) * elapsed_millis,
            );
            frame.translate(Vector::new(Self::ORBIT_RADIUS, 0.0));

            let earth = Path::circle(Point::ORIGIN, Self::EARTH_RADIUS);
            let shadow = Path::rectangle(
                Point::new(0.0, -Self::EARTH_RADIUS),
                Size::new(Self::EARTH_RADIUS * 4.0, Self::EARTH_RADIUS * 2.0),
            );

            frame.fill(&earth, Color::from_rgb8(0x6B, 0x93, 0xD6));

            frame.with_save(|frame| {
                frame.rotate(
                    ((2.0 * PI) / 6.0) * elapsed_seconds + ((2.0 * PI) / 6_000.0) * elapsed_millis,
                );
                frame.translate(Vector::new(0.0, Self::MOON_DISTANCE));

                let moon = Path::circle(Point::ORIGIN, Self::MOON_RADIUS);
                frame.fill(&moon, Color::WHITE);
            });

            frame.fill(
                &shadow,
                Color {
                    a: 0.7,
                    ..Color::BLACK
                },
            );
        });

        let test = self.current - self.start;
        let elapsed_seconds_ = test.as_secs() as f32;
        let elapsed_millis_ = test.subsec_millis() as f32;

        frame.with_save(|frame| {
            frame.translate(Vector::new(center.x, center.y));
            frame.rotate(
                (2.0 * PI / 60.0) * elapsed_seconds_ + (2.0 * PI / 60_000.0) * elapsed_millis_,
            );
            frame.translate(Vector::new(Self::MARS_ORBIT_RADIUS, 0.0));

            let earth = Path::circle(Point::ORIGIN, Self::MARS_RADIUS);
            let shadow = Path::rectangle(
                Point::new(0.0, -Self::MARS_RADIUS),
                Size::new(Self::MARS_RADIUS * 4.0, Self::MARS_RADIUS * 2.0),
            );

            frame.fill(&earth, Color::from_rgb8(193, 68, 14));

            frame.with_save(|frame| {
                frame.rotate(
                    ((2.0 * PI) / 6.0) * elapsed_seconds_
                        + ((2.0 * PI) / 6_000.0) * elapsed_millis_,
                );
            });

            frame.fill(
                &shadow,
                Color {
                    a: 0.7,
                    ..Color::BLACK
                },
            );
        });
    }
}
