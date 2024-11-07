struct Vektor2D {
    pub x: f64,
    pub y: f64,
}

impl Vektor2D {
    fn new(x: f64, y: f64) -> Self {
        Vektor2D { x: x, y: y }
    }
    fn add(&self, other: &Vektor2D) -> Vektor2D {
        Vektor2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn sub(&self, other: &Vektor2D) -> Vektor2D {
        Vektor2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    fn mul(&self, other: &Vektor2D) -> Vektor2D {
        Vektor2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
    fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    fn norm(&self) -> Vektor2D {
        let mag = self.mag();
        Vektor2D {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}

struct Body {
    pos: Vektor2D,
    vel: Vektor2D,
    mass: f64,
}

impl Body {
    fn new(pos: Vektor2D, vel: Vektor2D, mass: f64) -> Self {
        Body {
            pos: pos,
            vel: vel,
            mass: mass,
        }
    }
    fn verlet_update(&mut self, force: &Vektor2D, dt: f64) {
        // acceleration a = F/m
        let acc = force.mul(&Vektor2D::new(1.0 / self.mass, 1.0 / self.mass));

        // position r(t + dt) = r(t) + v(t)dt + (1/2)a(t)dt^2
        let pos_delta = self
            .vel
            .mul(&Vektor2D::new(dt, dt))
            .add(&acc.mul(&Vektor2D::new(0.5 * dt * dt, 0.5 * dt * dt)));
        self.pos = self.pos.add(&pos_delta);

        // velocity v(t + dt) = v(t) + a(t)dt
        let vel_delta = acc.mul(&Vektor2D::new(dt, dt));
        self.vel = self.vel.add(&vel_delta);
    }
}

fn gravitational_force(body1: &Body, body2: &Body, g: f64) -> Vektor2D {
    let r = body1.pos.sub(&body2.pos);
    let r_mag = r.mag();
    let force_mag = g * body1.mass * body2.mass / (r_mag * r_mag);
    r.norm().mul(&Vektor2D::new(force_mag, force_mag))
}

pub fn simulate_moon_earth() -> Vec<Vec<f64>> {
    let g = 6.67430e-11;

    let mut body1 = Body::new(Vektor2D::new(0.0, 0.0), Vektor2D::new(0.0, 0.0), 5.972e24); // earth
    let mut body2 = Body::new(
        Vektor2D::new(3.844e8, 0.0),
        Vektor2D::new(0.0, 1022.0),
        7.342e22,
    ); // moon

    let dt = 100.0;
    let steps = 30000;
    let mut body1_pos = Vec::new();
    let mut body2_pos = Vec::new();
    for _ in 0..steps {
        let force = gravitational_force(&body1, &body2, g);
        body1.verlet_update(&force, dt);
        body2.verlet_update(&force, dt);

        body1_pos.push(body1.pos.x);
        body1_pos.push(body1.pos.y);
        body2_pos.push(body2.pos.x);
        body2_pos.push(body2.pos.y);
    }
    vec![body1_pos, body2_pos]
}
