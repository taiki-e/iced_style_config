#[cfg(not(target_family = "wasm"))]
use std::path::PathBuf;
use std::{fmt, ops, path::Path, str::FromStr};

use futures_util::stream;
#[cfg(not(target_family = "wasm"))]
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
#[cfg(not(target_family = "wasm"))]
use tokio::sync::watch;
#[cfg(not(target_family = "wasm"))]
use tracing::debug;

use crate::{Error, Theme};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ReloadEvent;

#[derive(Debug)]
pub struct ReloadableTheme(Inner);

enum Inner {
    #[cfg(not(target_family = "wasm"))]
    Reloadable {
        theme: Theme,
        path: PathBuf,
        watcher: RecommendedWatcher,
        receiver: watch::Receiver<notify::Result<notify::Event>>,
    },
    Static {
        theme: Theme,
    },
}

impl FromStr for ReloadableTheme {
    type Err = Error;

    /// Creates a new Iced style sheets from a string of TOML text.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Inner::Static { theme: s.parse()? }))
    }
}

impl ReloadableTheme {
    /// Creates a new Iced style sheets from a TOML file.
    ///
    /// This begins watching the path and publishes events via
    /// [subscription](Self::subscription) when the path changes.
    #[cfg(not(target_family = "wasm"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))] // rustdoc displays "non-WebAssembly" for `not(wasm32)` not `not(wasm)`.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let theme = Theme::from_file(&path)?;

        let (sender, receiver) = watch::channel(Ok(notify::Event::new(notify::EventKind::Other)));

        let mut watcher: RecommendedWatcher =
            notify::recommended_watcher(move |res: notify::Result<_>| {
                let _ = sender.send(res);
            })
            .map_err(Error::new)?;
        debug!("start watching {}", path.display());
        watcher.watch(path, RecursiveMode::NonRecursive).map_err(Error::new)?;

        Ok(Self(Inner::Reloadable { theme, path: path.to_owned(), watcher, receiver }))
    }

    pub fn path(&self) -> Option<&Path> {
        match &self.0 {
            #[cfg(not(target_family = "wasm"))]
            Inner::Reloadable { path, .. } => Some(path),
            Inner::Static { .. } => None,
        }
    }

    /// Sets a new path and reloads the theme if necessary.
    ///
    /// If a new path and an old path are equal, the theme will not be reloaded.
    ///
    /// This begins watching the path and publishes events via
    /// [subscription](Self::subscription) when the path changes.
    #[cfg(not(target_family = "wasm"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))] // rustdoc displays "non-WebAssembly" for `not(wasm32)` not `not(wasm)`.
    pub fn set_path(&mut self, new_path: impl AsRef<Path>) -> Result<(), Error> {
        let new_path = new_path.as_ref();
        match &mut self.0 {
            Inner::Reloadable { theme: old_theme, path: old_path, watcher, .. } => {
                if new_path != *old_path {
                    let new_theme = Theme::from_file(&new_path)?;

                    debug!("start watching {}", new_path.display());
                    watcher.watch(new_path, RecursiveMode::NonRecursive).map_err(Error::new)?;
                    let _ = watcher.unwatch(old_path);

                    *old_path = new_path.to_owned();
                    *old_theme = new_theme;
                }
            }
            Inner::Static { .. } => *self = Self::from_file(new_path)?,
        }

        Ok(())
    }

    pub fn subscription(&self) -> iced::Subscription<ReloadEvent> {
        let recipe = match &self.0 {
            #[cfg(not(target_family = "wasm"))]
            Inner::Reloadable { receiver, .. } => Recipe { receiver: Some(receiver.clone()) },
            Inner::Static { .. } => Recipe { receiver: None },
        };
        iced::Subscription::from_recipe(recipe)
    }

    pub fn reload(&mut self) -> Result<(), Error> {
        #[cfg(not(target_family = "wasm"))]
        if let Self(Inner::Reloadable { theme, path, .. }) = self {
            debug!("reloading {}", path.display());
            let new = Theme::from_file(path)?;
            *theme = new;
        }
        Ok(())
    }
}

impl fmt::Debug for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(not(target_family = "wasm"))]
            Inner::Reloadable { theme, path, .. } => {
                f.debug_struct("Reloadable").field("theme", theme).field("path", path).finish()
            }
            Inner::Static { theme, .. } => f.debug_struct("Static").field("theme", theme).finish(),
        }
    }
}

impl ops::Deref for ReloadableTheme {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            #[cfg(not(target_family = "wasm"))]
            Inner::Reloadable { theme, .. } => theme,
            Inner::Static { theme, .. } => theme,
        }
    }
}

impl From<Theme> for ReloadableTheme {
    fn from(theme: Theme) -> Self {
        Self(Inner::Static { theme })
    }
}

impl Default for ReloadableTheme {
    fn default() -> Self {
        Self(Inner::Static { theme: Theme::default() })
    }
}

struct Recipe {
    #[cfg(not(target_family = "wasm"))]
    receiver: Option<watch::Receiver<notify::Result<notify::Event>>>,
    #[cfg(target_family = "wasm")]
    receiver: Option<()>,
}

impl<H, I> iced_futures::subscription::Recipe<H, I> for Recipe
where
    H: std::hash::Hasher,
{
    type Output = ReloadEvent;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: iced_futures::BoxStream<I>,
    ) -> iced_futures::BoxStream<Self::Output> {
        match self.receiver {
            None => Box::pin(stream::pending()),
            #[cfg(target_family = "wasm")]
            Some(()) => unreachable!(),
            #[cfg(not(target_family = "wasm"))]
            Some(mut receiver) => Box::pin(async_stream::stream! {
                while receiver.changed().await.is_ok() {
                    match &*receiver.borrow() {
                        // TODO: Which event should we detect here?
                        Ok(notify::Event { kind, .. })
                            if matches!(
                                kind,
                                notify::EventKind::Modify(..) | notify::EventKind::Create(..)
                            ) => {}
                        Ok(_) => continue,
                        Err(e) => {
                            debug!("{e}");
                            continue;
                        }
                    }
                    yield ReloadEvent;
                }
            }),
        }
    }
}
