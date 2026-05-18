use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::intent_definition::Shortcut;
use crate::private::{self, RawObject, RetainedObject};

/// Placeholder marker for relevant-shortcut shape data used alongside `INRelevantShortcut`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct RelevantShape;

/// Mirrors `INRelevantShortcutRole`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RelevantShortcutRole {
    /// Corresponds to the `Action` case of `INRelevantShortcutRole`.
    Action,
    /// Corresponds to the `Information` case of `INRelevantShortcutRole`.
    Information,
    /// Stores an unknown raw value from `INRelevantShortcutRole`.
    Unknown(i64),
}

impl RelevantShortcutRole {
    pub(crate) const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Action,
            1 => Self::Information,
            other => Self::Unknown(other),
        }
    }

    const fn raw_value(self) -> i64 {
        match self {
            Self::Action => 0,
            Self::Information => 1,
            Self::Unknown(raw) => raw,
        }
    }
}

/// Mirrors `INDailyRoutineRelevanceProvider`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DailyRoutineSituation {
    /// Corresponds to the `Morning` case of `INDailyRoutineRelevanceProvider`.
    Morning,
    /// Corresponds to the `Evening` case of `INDailyRoutineRelevanceProvider`.
    Evening,
    /// Corresponds to the `Home` case of `INDailyRoutineRelevanceProvider`.
    Home,
    /// Corresponds to the `Work` case of `INDailyRoutineRelevanceProvider`.
    Work,
    /// Corresponds to the `School` case of `INDailyRoutineRelevanceProvider`.
    School,
    /// Corresponds to the `Gym` case of `INDailyRoutineRelevanceProvider`.
    Gym,
    /// Corresponds to the `Commute` case of `INDailyRoutineRelevanceProvider`.
    Commute,
    /// Corresponds to the `HeadphonesConnected` case of `INDailyRoutineRelevanceProvider`.
    HeadphonesConnected,
    /// Corresponds to the `ActiveWorkout` case of `INDailyRoutineRelevanceProvider`.
    ActiveWorkout,
    /// Corresponds to the `PhysicalActivityIncomplete` case of `INDailyRoutineRelevanceProvider`.
    PhysicalActivityIncomplete,
    /// Stores an unknown raw value from `INDailyRoutineRelevanceProvider`.
    Unknown(i64),
}

impl DailyRoutineSituation {
    const fn raw_value(self) -> i64 {
        match self {
            Self::Morning => 0,
            Self::Evening => 1,
            Self::Home => 2,
            Self::Work => 3,
            Self::School => 4,
            Self::Gym => 5,
            Self::Commute => 6,
            Self::HeadphonesConnected => 7,
            Self::ActiveWorkout => 8,
            Self::PhysicalActivityIncomplete => 9,
            Self::Unknown(raw) => raw,
        }
    }
}

/// Wraps `INRelevanceProvider`.
#[derive(Debug)]
pub struct RelevanceProvider {
    raw: RetainedObject,
}

impl RelevanceProvider {
    /// Creates a `INRelevanceProvider` wrapper using the corresponding initializer.
    pub fn date(start: f64, end: Option<f64>) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_date_relevance_provider_create(
                start,
                end.unwrap_or_default(),
                end.is_some(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating date relevance provider") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Creates a `INRelevanceProvider` wrapper using the corresponding initializer.
    pub fn daily_routine(situation: DailyRoutineSituation) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_daily_routine_relevance_provider_create(situation.raw_value(), &mut error)
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating daily routine provider") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "relevance provider") }?,
        })
    }

    /// Returns the Objective-C class name for this `INRelevanceProvider` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }
}

impl RawObject for RelevanceProvider {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

/// Wraps `INRelevantShortcut`.
#[derive(Debug)]
pub struct RelevantShortcut {
    raw: RetainedObject,
}

impl RelevantShortcut {
    /// Creates a `INRelevantShortcut` wrapper.
    pub fn new(shortcut: &Shortcut) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_relevant_shortcut_create(shortcut.as_ptr(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating relevant shortcut") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "relevant shortcut") }?,
        })
    }

    /// Returns the corresponding value from `INRelevantShortcut`.
    pub fn shortcut_role(&self) -> RelevantShortcutRole {
        private::integer_property(self, "shortcutRole")
            .map_or(RelevantShortcutRole::Action, RelevantShortcutRole::from_raw)
    }

    /// Sets the corresponding `shortcut_role` value on `INRelevantShortcut`.
    pub fn set_shortcut_role(&mut self, role: RelevantShortcutRole) -> Result<(), IntentsError> {
        private::set_integer_property(self, "shortcutRole", role.raw_value())
    }

    /// Returns the corresponding value from `INRelevantShortcut`.
    pub fn widget_kind(&self) -> Option<String> {
        private::string_property(self, "widgetKind")
    }

    /// Sets the corresponding `widget_kind` value on `INRelevantShortcut`.
    pub fn set_widget_kind(&mut self, widget_kind: &str) -> Result<(), IntentsError> {
        private::set_string_property(self, "widgetKind", widget_kind)
    }

    /// Returns the corresponding value from `INRelevantShortcut`.
    pub fn shortcut(&self) -> Option<Shortcut> {
        private::object_property(self, "shortcut").map(Shortcut::from_retained)
    }

    /// Returns the corresponding value from `INRelevantShortcut`.
    pub fn watch_template_class_name(&self) -> Option<String> {
        private::object_property(self, "watchTemplate")
            .map(|template| private::class_name(&template))
    }

    /// Returns the number of corresponding values exposed by `INRelevantShortcut`.
    pub fn relevance_provider_count(&self) -> usize {
        private::array_count_property(self, "relevanceProviders").unwrap_or_default()
    }

    /// Sets the corresponding `relevance_providers` value on `INRelevantShortcut`.
    pub fn set_relevance_providers(
        &mut self,
        providers: &[&RelevanceProvider],
    ) -> Result<(), IntentsError> {
        let values = providers
            .iter()
            .map(|provider| provider.as_ptr())
            .collect::<Vec<_>>();
        private::set_object_array_property(self, "relevanceProviders", &values)
    }
}

impl RawObject for RelevantShortcut {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
