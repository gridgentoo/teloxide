use std::{
    fmt::Debug,
    marker::{Send, Sync},
    sync::Arc,
};

use futures::future::BoxFuture;

use crate::dispatching::dialogue::Storage;

/// A dialogue storage wrapper which logs all actions performed on an underlying
/// storage.
///
/// Reports about any dialogue action via [`log::Level::Trace`].
pub struct TraceStorage<S> {
    inner: Arc<S>,
}

impl<S> TraceStorage<S> {
    #[must_use]
    pub fn new(inner: Arc<S>) -> Arc<Self> {
        Arc::new(Self { inner })
    }

    pub fn into_inner(self) -> Arc<S> {
        self.inner
    }
}

impl<S, D> Storage<D> for TraceStorage<S>
where
    D: Debug,
    S: Storage<D> + Send + Sync + 'static,
{
    type Error = <S as Storage<D>>::Error;

    fn remove_dialogue(self: Arc<Self>, chat_id: i64) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        log::trace!("Removing dialogue #{}", chat_id);
        <S as Storage<D>>::remove_dialogue(self.inner.clone(), chat_id)
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            let to = format!("{:#?}", dialogue);
            <S as Storage<D>>::update_dialogue(self.inner.clone(), chat_id, dialogue).await?;
            log::trace!("Updated a dialogue #{}: {:#?}", chat_id, to);
            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        log::trace!("Requested a dialogue #{}", chat_id);
        <S as Storage<D>>::get_dialogue(self.inner.clone(), chat_id)
    }
}
