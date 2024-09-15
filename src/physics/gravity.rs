use crate::cosmos_object::CosmosObject;

#[inline(always)]
pub fn gravity(a: &mut CosmosObject, b: &CosmosObject) {
    let rect = b.position - a.position;
    let dist = rect.length();

    let dir = rect.normalized();

    a.acceleration += dir * b.mass / dist.powi(2);
}
