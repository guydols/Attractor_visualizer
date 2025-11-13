use macroquad::prelude::*;

struct State {
    linestrip: Vec<Vec3>,
    attractor: for<'a> fn(&'a Vec3) -> Vec3,
    center: Vec3,
    max_distance: f32,
    max: Vec3,
    min: Vec3,
}

impl State {
    fn new(func: for<'a> fn(&'a Vec3) -> Vec3) -> Self {
        let mut state = Self::default_state();
        state.attractor = func;
        state
    }

    fn reset(&mut self) {
        *self = Self::default_state();
        // Preserve the attractor function
        self.attractor = std::mem::replace(&mut self.attractor, |_| Vec3::default());
    }

    fn default_state() -> Self {
        let default_vec = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        State {
            linestrip: vec![default_vec],
            center: default_vec,
            max_distance: 0.0,
            max: default_vec,
            min: default_vec,
            attractor: |_| Vec3::default(), // placeholder
        }
    }
}

    fn change_attractor(&mut self, func: for<'a> fn(&'a Vec3) -> Vec3) {
        self.attractor = func;
        self.reset();
    }

    fn step(&self) -> Vec3 {
        (self.attractor)(self.linestrip.last().unwrap())
    }
}

// Defining a type for a value that moves from a max to a min
// the step function changes the value by the step size
struct BouncingVariable {
    pub value: f32,
    min: f32,
    max: f32,
    step: f32,
    direction: f32,
}

impl BouncingVariable {
    fn new(initial_value: f32, min: f32, max: f32, step: f32) -> Self {
        BouncingVariable {
            value: initial_value,
            min,
            max,
            step,
            direction: 1.0,
        }
    }

    fn update(&mut self, min: f32, max: f32) {
        self.value = self.value.clamp(min, max);
        self.min = min;
        self.max = max;
    }

    fn step(&mut self) {
        self.value += self.direction * self.step;
        if self.value > self.max {
            self.value = 2.0 * self.max - self.value;
            self.direction = -1.0;
        } else if self.value < self.min {
            self.value = 2.0 * self.min - self.value;
            self.direction = 1.0;
        }
    }
}

fn orbit_camera(camera: &mut Camera3D, distance: f32, angle_around_target: f32, pitch_angle: &f32) {
    // Calculate the position of the camera based on the distance, angle and pitch
    let x = distance * angle_around_target.to_radians().cos() * pitch_angle.to_radians().cos();
    let y = distance * pitch_angle.to_radians().sin();
    let z = distance * angle_around_target.to_radians().sin() * pitch_angle.to_radians().cos();
    let camera_pos = Vec3::new(x, y, z) + camera.target;

    // Set the camera position and up vector
    camera.position = camera_pos;
    camera.up = Vec3::new(0.0, 1.0, 0.0); // Assuming y-axis is up
}

#[macroquad::main("3D")]
async fn main() {
    let attractors: Vec<for<'a> fn(&'a Vec3) -> Vec3> = vec![
        lorenz,
        aizawa,
        chen,
        rossler,
        sprott,
        four_wing,
        // thomas, halvorsen,
        // dadras,
        burke_shaw,
        nose_hoover,
        rabinovich_fabrikant,
        three_scroll_unified,
        yu_wang,
    ];
    let mut current_attractor: usize = 0;

    // Define 3d camera type
    let mut camera = Camera3D {
        position: Vec3::new(0.0, 10.0, 10.0),
        target: Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        fovy: 90.0,
        aspect: None,
        projection: Projection::Perspective,
        render_target: None,
        viewport: None,
    };

    // Define 3d camera behauviour vars
    let mut angle_around_target = BouncingVariable::new(0.0, 0.0, 360.0, 0.5);
    let mut pitch_angle = BouncingVariable::new(0.0, -89.4, 89.4, 0.5);
    let mut cam_distance = BouncingVariable::new(1.0, 0.5, 1.5, 0.005);

    let mut state = State::new(attractors[current_attractor]);

    loop {
        if state.linestrip.len() > 50000 {
            if current_attractor == attractors.len() - 1 {
                current_attractor = 0;
            } else {
                current_attractor += 1;
            }
            state.change_attractor(attractors[current_attractor]);
            camera.target = Vec3::new(0.0, 0.0, 0.0);
        }

        // Calculate the next 30 steps in the Lorenz equation
        for _ in 0..50 {
            let new_val = state.step();

            if state.max.x < new_val.x {
                state.max.x = new_val.x.clone();
            }
            if state.max.y < new_val.y {
                state.max.y = new_val.y.clone();
            }
            if state.max.z < new_val.z {
                state.max.z = new_val.z.clone();
            }
            if state.min.x > new_val.x {
                state.min.x = new_val.x.clone();
            }
            if state.min.y > new_val.y {
                state.min.y = new_val.y.clone();
            }
            if state.min.z > new_val.z {
                state.min.z = new_val.z.clone();
            }
            let dist = Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            .distance(new_val);
            if dist > state.max_distance {
                state.max_distance = dist;
            }
            state.linestrip.push(new_val);
        }

        clear_background(BLACK);

        // change angle of the camera on the y axis
        pitch_angle.step();

        cam_distance.update(state.max_distance * 0.4, state.max_distance);
        cam_distance.step();

        angle_around_target.step();

        // Find the center of all the points in linestrip to use as target to look at
        let new_target = (state.max + state.min)
            / Vec3 {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            };
        camera.target -= (camera.target - new_target) * 0.01;

        orbit_camera(
            &mut camera,
            cam_distance.value,
            angle_around_target.value,
            &pitch_angle.value,
        );
        set_camera(&camera);

        // Loop through the linestrip and draw a line between each point in the vector
        for (i, _) in state.linestrip.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Calced distance for color
            let dist = state.linestrip[i].distance(state.center);
            let normalized_distance = dist / state.max_distance;
            let color = get_turbo_color(normalized_distance);

            draw_line_3d(
                state.linestrip[i - 1],
                state.linestrip[i],
                Color::new(color.0, color.1, color.2, 1.0),
            );
        }

        // render next frame
        next_frame().await
    }
}

fn get_turbo_color(t: f32) -> (f32, f32, f32) {
    // let t = t.clamp(0.0, 1.0);

    let r = (34.61
        + t * (1172.33 - t * (10793.56 - t * (33300.12 - t * (38394.49 - t * 14825.05)))))
        .clamp(0.0, 255.0);
    let g = (23.31 + t * (557.33 + t * (1225.33 - t * (3574.96 - t * (1073.77 + t * 707.56)))))
        .clamp(0.0, 255.0);
    let b = (27.2 + t * (3211.1 - t * (15327.97 - t * (27814.0 - t * (22569.18 - t * 6838.66)))))
        .clamp(0.0, 255.0);

    (r / 255.0, g / 255.0, b / 255.0)
}

// Lorenz attractor equation
fn lorenz(v: &Vec3) -> Vec3 {
    let sigma = 10.0;
    let rho = 28.0;
    let beta = 8.0 / 3.0;
    let dt = 0.001;

    Vec3 {
        x: ((sigma * (v.y - v.x)) * dt) + v.x,
        y: ((v.x * (rho - v.z) - v.y) * dt) + v.y,
        z: ((v.x * v.y - beta * v.z) * dt) + v.z,
    }
}

fn aizawa(v: &Vec3) -> Vec3 {
    let a = 0.95;
    let b = 0.7;
    let c = 0.6;
    let d = 3.5;
    let e = 0.25;
    let f = 0.1;
    let dt = 0.01;
    Vec3 {
        x: ((v.z - b) * v.x - d * v.y) * dt + v.x,
        y: (d * v.x + (v.z - b) * v.y) * dt + v.y,
        z: (c + a * v.z - v.z.powi(3) / 3.0 - (v.x.powi(2) + v.y.powi(2)) * (1.0 + e * v.z)
            + f * v.z * v.x.powi(3))
            * dt
            + v.z,
    }
}

fn rossler(v: &Vec3) -> Vec3 {
    let a = 0.2;
    let b = 0.2;
    let c = 5.7;
    let dt = 0.01;
    Vec3 {
        x: ((-v.y - v.z) * dt) + v.x,
        y: ((v.x + a * v.y) * dt) + v.y,
        z: ((b + v.z * (v.x - c)) * dt) + v.z,
    }
}

fn chen(v: &Vec3) -> Vec3 {
    let a = 40.0;
    let b = 3.0;
    let c = 28.0;
    let dt = 0.001;
    Vec3 {
        x: ((a * (v.y - v.x)) * dt) + v.x,
        y: (((c - a) * v.x - v.x * v.z + c * v.y) * dt) + v.y,
        z: ((v.x * v.y - b * v.z) * dt) + v.z,
    }
}

fn four_wing(v: &Vec3) -> Vec3 {
    let a = 0.2;
    let b = 0.01;
    let c = -0.4;
    let dt = 0.01;
    Vec3 {
        x: ((a * v.x + v.y * v.z) * dt) + v.x,
        y: ((b * v.x + c * v.y - v.x * v.z) * dt) + v.y,
        z: ((-v.z - v.x * v.y) * dt) + v.z,
    }
}

fn thomas(v: &Vec3) -> Vec3 {
    let b = 0.208186;
    let dt = 0.01;
    Vec3 {
        x: ((-b * v.x + v.y.sin()) * dt) + v.x,
        y: ((-b * v.y + v.z.sin()) * dt) + v.y,
        z: ((-b * v.z + v.x.sin()) * dt) + v.z,
    }
}

fn halvorsen(v: &Vec3) -> Vec3 {
    let a = 1.4;
    let dt = 0.01;
    Vec3 {
        x: ((-a * v.x - 4.0 * v.y - 4.0 * v.z - v.y * v.y) * dt) + v.x,
        y: ((-a * v.y - 4.0 * v.z - 4.0 * v.x - v.z * v.z) * dt) + v.y,
        z: ((-a * v.z - 4.0 * v.x - 4.0 * v.y - v.x * v.x) * dt) + v.z,
    }
}

fn dadras(v: &Vec3) -> Vec3 {
    let a = 3.0;
    let b = 2.7;
    let c = 1.7;
    let d = 2.0;
    let e = 9.0;
    let dt = 0.01;
    Vec3 {
        x: ((v.y - a * v.x + b * v.y * v.z) * dt) + v.x,
        y: ((c * v.y - v.x * v.z + v.z) * dt) + v.y,
        z: ((d * v.x * v.y - e * v.z) * dt) + v.z,
    }
}

fn sprott(v: &Vec3) -> Vec3 {
    let a = 2.07;
    let b = 1.79;
    let dt = 0.01;
    Vec3 {
        x: ((v.y + a * v.x * v.y + v.x * v.z) * dt) + v.x,
        y: ((1.0 - b * v.x * v.x + v.y * v.z) * dt) + v.y,
        z: ((v.x - v.x * v.x - v.y * v.y) * dt) + v.z,
    }
}

fn burke_shaw(v: &Vec3) -> Vec3 {
    let s = 10.0;
    let w = 4.272;
    let dt = 0.01;
    Vec3 {
        x: ((-s * (v.x + v.y)) * dt) + v.x,
        y: ((-v.y - s * v.x * v.z) * dt) + v.y,
        z: ((s * v.x * v.y + w) * dt) + v.z,
    }
}

fn nose_hoover(v: &Vec3) -> Vec3 {
    let a = 1.5;
    let dt = 0.01;
    Vec3 {
        x: (v.y * dt) + v.x,
        y: ((-v.x + v.y * v.z) * dt) + v.y,
        z: ((a - v.y * v.y) * dt) + v.z,
    }
}

fn rabinovich_fabrikant(v: &Vec3) -> Vec3 {
    let a = 0.14;
    let b = 0.10;
    let dt = 0.01;
    Vec3 {
        x: ((v.y * (v.z - 1.0 + v.x * v.x) + b * v.x) * dt) + v.x,
        y: ((v.x * (3.0 * v.z + 1.0 - v.x * v.x) + b * v.y) * dt) + v.y,
        z: ((-2.0 * v.z * (a + v.x * v.y)) * dt) + v.z,
    }
}

fn three_scroll_unified(v: &Vec3) -> Vec3 {
    let a = 40.0;
    let b = 0.5;
    let c = 20.0;
    let d = 0.833;
    let e = 0.65;
    let dt = 0.001;
    Vec3 {
        x: ((a * (v.y - v.x) + d * v.x * v.z) * dt) + v.x,
        y: ((c * v.y - v.x * v.z) * dt) + v.y,
        z: ((b * v.z + v.x * v.y - e * v.x * v.x) * dt) + v.z,
    }
}

fn yu_wang(v: &Vec3) -> Vec3 {
    let a = 10.0;
    let b = 40.0;
    let c = 2.0;
    let d = 2.5;
    let dt = 0.001;
    Vec3 {
        x: ((a * (v.y - v.x)) * dt) + v.x,
        y: ((b * v.x - c * v.x * v.z) * dt) + v.y,
        z: ((v.x.powi(2) - d * v.z) * dt) + v.z,
    }
}
