struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x: x, y: y }
    }
    fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn sub(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    fn mul(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
    fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    fn norm(&self) -> Vector2D {
        let mag = self.mag();
        Vector2D {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}

struct Body {
    pos: Vector2D,
    vel: Vector2D,
    mass: f64,
}

impl Body {
    fn new(pos: Vector2D, vel: Vector2D, mass: f64) -> Self {
        Body {
            pos: pos,
            vel: vel,
            mass: mass,
        }
    }
    fn verlet_update(&mut self, force: &Vector2D, dt: f64) {
        // acceleration a = F/m
        let acc = force.mul(&Vector2D::new(1.0 / self.mass, 1.0 / self.mass));

        // position r(t + dt) = r(t) + v(t)dt + (1/2)a(t)dt^2
        let pos_delta = self
            .vel
            .mul(&Vector2D::new(dt, dt))
            .add(&acc.mul(&Vector2D::new(0.5 * dt * dt, 0.5 * dt * dt)));
        self.pos = self.pos.add(&pos_delta);

        // velocity v(t + dt) = v(t) + a(t)dt
        let vel_delta = acc.mul(&Vector2D::new(dt, dt));
        self.vel = self.vel.add(&vel_delta);
    }
}

fn gravitational_force(body1: &Body, body2: &Body, g: f64) -> Vector2D {
    let r = body1.pos.sub(&body2.pos);
    let r_mag = r.mag();
    let force_mag = g * body1.mass * body2.mass / (r_mag * r_mag);
    r.norm().mul(&Vector2D::new(force_mag, force_mag))
}

pub fn simulate_moon_earth(dt: f64, steps: i32) -> Vec<Vec<f64>> {
    let g = 6.67430e-11;

    let mut body1 = Body::new(Vector2D::new(0.0, 0.0), Vector2D::new(0.0, 0.0), 5.972e24); // earth
    let mut body2 = Body::new(
        Vector2D::new(3.844e8, 0.0),
        Vector2D::new(0.0, 1022.0),
        7.342e22,
    ); // moon

    let dt = dt;
    let steps = steps;
    let mut body1_x = Vec::new();
    let mut body1_y = Vec::new();
    let mut body2_x = Vec::new();
    let mut body2_y = Vec::new();
    for _ in 0..steps {
        let force = gravitational_force(&body1, &body2, g);
        body1.verlet_update(&force, dt);
        body2.verlet_update(&force, dt);

        body1_x.push(body1.pos.x);
        body1_y.push(body1.pos.y);
        body2_x.push(body2.pos.x);
        body2_y.push(body2.pos.y);
    }
    vec![body1_x, body1_y, body2_x, body2_y]
}
