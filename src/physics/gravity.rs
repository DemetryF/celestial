use crate::cosmos_object::CosmosObject;

#[inline(always)]
pub fn gravity(a: &mut CosmosObject, b: &CosmosObject) {
    let rect = b.position - a.position;
    let dist = rect.length();

    let dir = rect.normalized();

    let force = a.mass * b.mass / dist.powi(2);

    a.acceleration += dir * force / a.mass;
}
