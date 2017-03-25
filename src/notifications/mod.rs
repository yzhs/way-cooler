mod notification;
mod notification_draw;

pub use self::notification::Notification;
pub use self::notification_draw::NotificationDraw;

use std::sync::{Mutex, MutexGuard, TryLockError, PoisonError};

type Notifications = Vec<Notification>;
type NotificationsGuard = MutexGuard<'static, Notifications>;
type NotificationsErr = TryLockError<NotificationsGuard>;

lazy_static! {
    static ref NOTIFICATIONS: Mutex<Notifications> = {
        Mutex::new(Vec::with_capacity(16))
    };
}

pub fn try_lock_notifications() -> Result<NotificationsGuard, NotificationsErr> {
    NOTIFICATIONS.try_lock()
}

pub fn lock_notifications() -> Result<NotificationsGuard, NotificationsErr> {
    Ok(NOTIFICATIONS.lock()?)
}
