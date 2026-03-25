//! Initial entity list for the Saturn arena.

use super::entity::Entity;

pub fn initial_entities() -> Vec<Entity> {
    vec![
        // Moons: orbit_angle, orbit_r, radius, energy
        Entity::moon(0.0, 0.0, 0.0, 10.0, 2.5, 500.0),
        Entity::moon(0.0, 0.0, 1.8, 14.0, 1.2, 200.0),
        Entity::moon(0.0, 0.0, 3.5, 17.0, 1.0, 150.0),
        Entity::moon(0.0, 0.0, 5.2, 20.0, 1.4, 300.0),
        // Debris: x, y, vx, vy, radius, energy
        Entity::debris(10.0, 20.0,  0.03, -0.01, 0.4, 10.0),
        Entity::debris(80.0, 15.0, -0.02,  0.02, 0.6, 15.0),
        Entity::debris(30.0, 85.0,  0.01, -0.03, 0.3,  8.0),
        Entity::debris(75.0, 70.0, -0.03, -0.02, 0.5, 12.0),
        Entity::debris(20.0, 60.0,  0.02,  0.01, 0.4, 10.0),
        Entity::debris(60.0, 30.0, -0.01,  0.03, 0.6, 14.0),
        Entity::debris(40.0, 10.0,  0.03,  0.02, 0.3,  7.0),
        Entity::debris(90.0, 50.0, -0.02, -0.01, 0.5, 11.0),
        Entity::debris(15.0, 40.0,  0.01, -0.03, 0.4,  9.0),
        Entity::debris(55.0, 80.0, -0.03,  0.02, 0.6, 13.0),
        Entity::debris(70.0, 45.0,  0.02, -0.02, 0.3,  8.0),
        Entity::debris(25.0, 25.0, -0.01,  0.03, 0.5, 12.0),
    ]
}
