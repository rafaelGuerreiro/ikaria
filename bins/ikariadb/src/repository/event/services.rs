use std::{ops::Deref, time::Duration};

use crate::{
    error::ServiceResult,
    repository::event::{
        OneshotDeferredEventV1, oneshot_deferred_event_v1,
        types::{DeferredEventV1, EventV1},
    },
};
use spacetimedb::{ReducerContext, Table};

pub trait EventReducerContext {
    fn event_services(&self) -> EventServices<'_>;

    fn publish(&self) -> EventPublisher<'_>;
}

impl EventReducerContext for ReducerContext {
    fn event_services(&self) -> EventServices<'_> {
        EventServices { ctx: self }
    }

    fn publish(&self) -> EventPublisher<'_> {
        EventPublisher { ctx: self }
    }
}

pub struct EventServices<'a> {
    ctx: &'a ReducerContext,
}

impl Deref for EventServices<'_> {
    type Target = ReducerContext;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl EventServices<'_> {
    fn handle_sync_event(&self, event: EventV1, _rethrow: bool) -> ServiceResult<()> {
        match event {
            EventV1::SystemInit => {}
            EventV1::UserCreated { .. } => {}
            EventV1::UserSignedIn { .. } => {}
            EventV1::UserSignedOut { .. } => {}
        }

        Ok(())
    }

    pub fn handle_deferred_event(&self, timer: OneshotDeferredEventV1) {
        match timer.event {
            DeferredEventV1::SignedOut { .. } => {}
        }
    }

    pub fn fire(&self, event: EventV1) -> ServiceResult<()> {
        if let Some(deferred) = event.into_deferred() {
            self.enqueue_deferred_event(deferred);
        }

        self.handle_sync_event(event, true)?;
        Ok(())
    }

    pub fn fire_and_forget(&self, event: EventV1) {
        let _ = self.handle_sync_event(event, false);
    }

    fn enqueue_deferred_event(&self, event: DeferredEventV1) {
        let scheduled_at = self.timestamp + Duration::from_millis(12);

        self.db
            .oneshot_deferred_event_v1()
            .insert(OneshotDeferredEventV1 {
                job_id: 0,
                scheduled_at: scheduled_at.into(),
                event,
                sender: self.sender,
                created_at: self.timestamp,
            });
    }
}

pub struct EventPublisher<'a> {
    ctx: &'a ReducerContext,
}

impl Deref for EventPublisher<'_> {
    type Target = ReducerContext;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}
