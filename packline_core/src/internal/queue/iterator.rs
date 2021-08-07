use async_trait::async_trait;

#[async_trait]
pub trait AsyncIterator {
    type Item: Copy + Sized;

    async fn next(&mut self) -> Self::Item {
        *self.next_count(1).await.get(0).unwrap()
    }
    async fn next_count(&mut self, count: usize) -> Vec<Self::Item>;
}
