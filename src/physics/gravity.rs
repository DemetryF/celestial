use crate::cosmos_object::CosmosObject;

#[inline(always)]
pub fn gravity(a: &mut CosmosObject, b: &mut CosmosObject, delta_time: f32) {
    let rect = b.position - a.position;
    let dist = rect.length_sq().sqrt();

    let direction = rect / dist;

    let force = a.mass * b.mass / dist.powi(2);

    let a_acceleration = force / a.mass;
    let b_acceleration = force / b.mass;

    a.velocity += direction * a_acceleration * delta_time;
    b.velocity += -direction * b_acceleration * delta_time;
}
