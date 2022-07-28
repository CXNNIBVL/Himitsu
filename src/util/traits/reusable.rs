
pub trait Reusable {
    type Props;
    fn reset(&mut self, props: Self::Props);
}