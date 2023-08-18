/// Creates an named boolean-like enum (set or unset enums).
///
/// Implements utility traits for converting from/to basic types.
#[macro_export]
macro_rules! bool_enum {
    ($name:ident, $doc:tt) => {
        #[doc = $doc]
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            /// The field is unset
            Unset = 0b0,
            /// The field is set
            Set = 0b1,
        }

        impl From<bool> for $name {
            fn from(b: bool) -> Self {
                match b {
                    false => Self::Unset,
                    true => Self::Set,
                }
            }
        }

        impl From<u8> for $name {
            fn from(b: u8) -> Self {
                match (b & 0b1) {
                    0b0 => Self::Unset,
                    0b1 => Self::Set,
                    _ => Self::Unset,
                }
            }
        }

        impl From<$name> for bool {
            fn from(n: $name) -> Self {
                n == $name::Set
            }
        }

        impl From<&$name> for bool {
            fn from(n: &$name) -> Self {
                (*n).into()
            }
        }

        impl From<$name> for u8 {
            fn from(n: $name) -> Self {
                (n == $name::Set) as u8
            }
        }

        impl From<&$name> for u8 {
            fn from(n: &$name) -> Self {
                (*n).into()
            }
        }

        impl From<&$name> for &'static str {
            fn from(name: &$name) -> Self {
                let set: bool = name.into();
                if set {
                    "set"
                } else {
                    "unset"
                }
            }
        }

        impl From<$name> for &'static str {
            fn from(name: $name) -> Self {
                (&name).into()
            }
        }

        impl $crate::std::fmt::Display for $name {
            fn fmt(&self, f: &mut $crate::std::fmt::Formatter<'_>) -> $crate::std::fmt::Result {
                write!(f, r#""{}""#, <&str>::from(self))
            }
        }
    };

    ($name:ident) => {
        bool_enum!($name, "");
    };
}

/// Implements the [MessageOps](crate::MessageOps) trait for a named type.
#[macro_export]
macro_rules! impl_message_ops {
    ($name:ident) => {
        impl $crate::MessageOps for $name {
            fn buf(&self) -> &[u8] {
                self.buf.as_ref()
            }

            fn buf_mut(&mut self) -> &mut [u8] {
                self.buf.as_mut()
            }
        }
    };

    ($name:ident, $full_len:ident, $enable_len:ident) => {
        impl<const $full_len: usize, const $enable_len: usize> $crate::MessageOps
            for $name<$full_len, $enable_len>
        {
            fn buf(&self) -> &[u8] {
                self.buf.as_ref()
            }

            fn buf_mut(&mut self) -> &mut [u8] {
                self.buf.as_mut()
            }
        }
    };
}

/// Implements the defaults for the [OmnibusCommandOps](crate::OmnibusCommandOps) trait for a
/// named type.
#[macro_export]
macro_rules! impl_omnibus_command_ops {
    ($name:ident) => {
        impl $crate::OmnibusCommandOps for $name {}
    };
}

/// Implements the defaults for the [OmnibusReplyOps](crate::OmnibusReplyOps) trait for a
/// named type.
#[macro_export]
macro_rules! impl_omnibus_reply_ops {
    ($name:ident) => {
        impl $crate::OmnibusReplyOps for $name {}
    };
}

/// Implements the defaults for the [ExtendedCommandOps](crate::ExtendedCommandOps) trait for a
/// named type in the Extended messages subset.
#[macro_export]
macro_rules! impl_extended_ops {
    ($name:ident) => {
        impl $crate::ExtendedCommandOps for $name {}
    };

    ($name:ident, $full_len:ident, $enable_len:ident) => {
        impl<const $full_len: usize, const $enable_len: usize> $crate::ExtendedCommandOps
            for $name<$full_len, $enable_len>
        {
        }
    };
}

/// Implements the defaults for the [ExtendedReplyOps](crate::ExtendedReplyOps) trait for a
/// named type in the Extended messages subset.
#[macro_export]
macro_rules! impl_extended_reply_ops {
    ($name:ident) => {
        impl $crate::ExtendedReplyOps for $name {}
    };

    ($name:ident, $full_len:ident, $enable_len:ident) => {
        impl<const $full_len: usize, const $enable_len: usize> $crate::ExtendedReplyOps
            for $name<$full_len, $enable_len>
        {
        }
    };
}

/// Implements the defaults for the [AuxCommandOps](crate::AuxCommandOps) trait for a
/// named type in the Auxilliary messages subset.
#[macro_export]
macro_rules! impl_aux_ops {
    ($name:ident) => {
        impl $crate::AuxCommandOps for $name {}
    };
}

/// Implements the defaults for the [OmnibusCommandOps](crate::OmnibusCommandOps) trait for a
/// named type that is in the subset of Extended Commands.
#[macro_export]
macro_rules! impl_omnibus_extended_command {
    ($name:ident) => {
        impl $crate::OmnibusCommandOps for $name {
            fn denomination(&self) -> $crate::StandardDenomination {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf()[index::DENOMINATION + 1].into()
            }

            fn set_denomination(&mut self, denomination: $crate::StandardDenomination) {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf_mut()[index::DENOMINATION + 1] = denomination.into();
            }

            fn operational_mode(&self) -> $crate::OperationalMode {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf()[index::OPERATIONAL_MODE + 1].into()
            }

            fn set_operational_mode(&mut self, operational_mode: $crate::OperationalMode) {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf_mut()[index::OPERATIONAL_MODE + 1] = operational_mode.into();
            }

            fn configuration(&self) -> $crate::Configuration {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf()[index::CONFIGURATION + 1].into()
            }

            fn set_configuration(&mut self, configuration: $crate::Configuration) {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf_mut()[index::CONFIGURATION + 1] = configuration.into();
            }
        }
    };

    ($name:ident, $full_len:ident, $enable_len:ident) => {
        impl<const $full_len: usize, const $enable_len: usize> $crate::OmnibusCommandOps
            for $name<$full_len, $enable_len>
        {
            fn denomination(&self) -> $crate::StandardDenomination {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf()[index::DENOMINATION + 1].into()
            }

            fn set_denomination(&mut self, denomination: $crate::StandardDenomination) {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf_mut()[index::DENOMINATION + 1] = denomination.into();
            }

            fn operational_mode(&self) -> $crate::OperationalMode {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf()[index::OPERATIONAL_MODE + 1].into()
            }

            fn set_operational_mode(&mut self, operational_mode: $crate::OperationalMode) {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf_mut()[index::OPERATIONAL_MODE + 1] = operational_mode.into();
            }

            fn configuration(&self) -> $crate::Configuration {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf()[index::CONFIGURATION + 1].into()
            }

            fn set_configuration(&mut self, configuration: $crate::Configuration) {
                use $crate::{omnibus::command::index, MessageOps};
                self.buf_mut()[index::CONFIGURATION + 1] = configuration.into();
            }
        }
    };
}

/// Implements the defaults for the [OmnibusReplyOps](crate::OmnibusReplyOps) trait for a
/// named type that is in the subset of Extended Replies.
#[macro_export]
macro_rules! impl_omnibus_extended_reply {
    ($name:ident) => {
        impl $crate::OmnibusReplyOps for $name {
            fn device_state(&self) -> $crate::DeviceState {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf()[index::DEVICE_STATE + 1].into()
            }

            fn set_device_state(&mut self, device_state: $crate::DeviceState) {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf_mut()[index::DEVICE_STATE + 1] = device_state.into();
            }

            fn device_status(&self) -> $crate::DeviceStatus {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf()[index::DEVICE_STATUS + 1].into()
            }

            fn set_device_status(&mut self, device_status: $crate::DeviceStatus) {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf_mut()[index::DEVICE_STATUS + 1] = device_status.into();
            }

            fn exception_status(&self) -> $crate::ExceptionStatus {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf()[index::EXCEPTION_STATUS + 1].into()
            }

            fn set_exception_status(&mut self, exception_status: $crate::ExceptionStatus) {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf_mut()[index::EXCEPTION_STATUS + 1] = exception_status.into();
            }

            fn misc_device_state(&self) -> $crate::MiscDeviceState {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf()[index::MISC_DEVICE_STATE + 1].into()
            }

            fn set_misc_device_state(&mut self, misc_device_state: $crate::MiscDeviceState) {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf_mut()[index::MISC_DEVICE_STATE + 1] = misc_device_state.into();
            }

            fn model_number(&self) -> $crate::ModelNumber {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf()[index::MODEL_NUMBER + 1].into()
            }

            fn set_model_number(&mut self, model_number: $crate::ModelNumber) {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf_mut()[index::MODEL_NUMBER + 1] = model_number.into();
            }

            fn code_revision(&self) -> $crate::CodeRevision {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf()[index::CODE_REVISION + 1].into()
            }

            fn set_code_revision(&mut self, code_revision: $crate::CodeRevision) {
                use $crate::{omnibus::reply::index, MessageOps};
                self.buf_mut()[index::CODE_REVISION + 1] = code_revision.into();
            }
        }
    };
}

/// Sets all [OmnibusReplyOps](crate::OmnibusReplyOps) functions to `unimplemented` for an [AuxCommand](crate::AuxCommand) reply type.
///
/// Intended to allow generalization over AuxCommand reply types as [OmnibusReplyOps](crate::OmnibusReplyOps) in contexts
/// where calling the trait functions is not intended. For example, in [ReplyVariant](crate::ReplyVariant) where each
/// variant needs to implement the [OmnibusReplyOps](crate::OmnibusReplyOps) trait, but it is not necessary to actually
/// call the trait functions.
#[macro_export]
macro_rules! impl_omnibus_nop_reply {
    ($name:ident) => {
        impl $crate::OmnibusReplyOps for $name {
            fn device_state(&self) -> $crate::DeviceState {
                0u8.into()
            }

            fn set_device_state(&mut self, _device_state: $crate::DeviceState) {}

            fn idling(&self) -> $crate::Idling {
                0u8.into()
            }

            fn set_idling(&mut self, _idling: $crate::Idling) {}

            fn accepting(&self) -> $crate::Accepting {
                0u8.into()
            }

            fn set_accepting(&mut self, _accepting: $crate::Accepting) {}

            fn escrowed_state(&self) -> $crate::EscrowedState {
                0u8.into()
            }

            fn set_escrowed_state(&mut self, _escrowed_state: $crate::EscrowedState) {}

            fn stacking(&self) -> $crate::Stacking {
                0u8.into()
            }

            fn set_stacking(&mut self, _stacking: $crate::Stacking) {}

            fn stacked_event(&self) -> $crate::StackedEvent {
                0u8.into()
            }

            fn set_stacked_event(&mut self, _stacked_event: $crate::StackedEvent) {}

            fn returning(&self) -> $crate::Returning {
                0u8.into()
            }

            fn set_returning(&mut self, _returning: $crate::Returning) {}

            fn returned_event(&self) -> $crate::ReturnedEvent {
                0u8.into()
            }

            fn set_returned_event(&mut self, _returned_event: $crate::ReturnedEvent) {}

            fn device_status(&self) -> $crate::DeviceStatus {
                0u8.into()
            }

            fn set_device_status(&mut self, _device_status: $crate::DeviceStatus) {}

            fn cheated(&self) -> $crate::Cheated {
                0u8.into()
            }

            fn set_cheated(&mut self, _cheated: $crate::Cheated) {}

            fn rejected(&self) -> $crate::Rejected {
                0u8.into()
            }

            fn set_rejected(&mut self, _rejected: $crate::Rejected) {}

            fn jammed(&self) -> $crate::Jammed {
                0u8.into()
            }

            fn set_jammed(&mut self, _jammed: $crate::Jammed) {}

            fn stacker_full(&self) -> $crate::StackerFull {
                0u8.into()
            }

            fn set_stacker_full(&mut self, _stacker_full: $crate::StackerFull) {}

            fn cassette_attached(&self) -> $crate::CassetteAttached {
                0u8.into()
            }

            fn set_cassette_attached(&mut self, _cassette_attached: $crate::CassetteAttached) {}

            fn cash_box_status(&self) -> $crate::CashBoxStatus {
                0u8.into()
            }

            fn paused(&self) -> $crate::Paused {
                0u8.into()
            }

            fn set_paused(&mut self, _paused: $crate::Paused) {}

            fn calibration(&self) -> $crate::Calibration {
                0u8.into()
            }

            fn set_calibration(&mut self, _calibration: $crate::Calibration) {}

            fn exception_status(&self) -> $crate::ExceptionStatus {
                0u8.into()
            }

            fn set_exception_status(&mut self, _exception_status: $crate::ExceptionStatus) {}

            fn power_up(&self) -> $crate::PowerUpStatus {
                0u8.into()
            }

            fn set_power_up(&mut self, _power_up: $crate::PowerUpStatus) {}

            fn invalid_command(&self) -> $crate::InvalidCommand {
                0u8.into()
            }

            fn set_invalid_command(&mut self, _invalid_command: $crate::InvalidCommand) {}

            fn failure(&self) -> $crate::Failure {
                0u8.into()
            }

            fn set_failure(&mut self, _failure: $crate::Failure) {}

            fn note_value(&self) -> $crate::StandardDenomination {
                0u8.into()
            }

            fn set_note_value(&mut self, _note_value: $crate::StandardDenomination) {}

            fn transport_open(&self) -> $crate::TransportOpen {
                0u8.into()
            }

            fn set_transport_open(&mut self, _transport_open: $crate::TransportOpen) {}

            fn misc_device_state(&self) -> $crate::MiscDeviceState {
                0u8.into()
            }

            fn set_misc_device_state(&mut self, _misc_device_state: $crate::MiscDeviceState) {}

            fn stalled(&self) -> $crate::Stalled {
                0u8.into()
            }

            fn set_stalled(&mut self, _stalled: $crate::Stalled) {}

            fn flash_download(&self) -> $crate::FlashDownload {
                0u8.into()
            }

            fn set_flash_download(&mut self, _flash_download: $crate::FlashDownload) {}

            fn pre_stack(&self) -> $crate::PreStack {
                0u8.into()
            }

            fn set_pre_stack(&mut self, _pre_stack: $crate::PreStack) {}

            fn raw_barcode(&self) -> $crate::RawBarcode {
                0u8.into()
            }

            fn set_raw_barcode(&mut self, _raw_barcode: $crate::RawBarcode) {}

            fn device_capabilities(&self) -> $crate::DeviceCapabilities {
                0u8.into()
            }

            fn set_device_capabilities(
                &mut self,
                _device_capabilities: $crate::DeviceCapabilities,
            ) {
            }

            fn disabled(&self) -> $crate::Disabled {
                0u8.into()
            }

            fn set_disabled(&mut self, _disabled: $crate::Disabled) {}

            fn model_number(&self) -> $crate::ModelNumber {
                0u8.into()
            }

            fn set_model_number(&mut self, _model_number: $crate::ModelNumber) {}

            fn code_revision(&self) -> $crate::CodeRevision {
                0u8.into()
            }

            fn set_code_revision(&mut self, _code_revision: $crate::CodeRevision) {}
        }
    };
}

#[macro_export]
macro_rules! impl_from_for_omnibus_reply {
    ($name:ident) => {
        impl From<&$name> for $crate::OmnibusReply {
            fn from(reply: &$name) -> Self {
                use $crate::OmnibusReplyOps;

                let mut msg = Self::new();

                msg.set_device_state(reply.device_state());
                msg.set_device_status(reply.device_status());
                msg.set_exception_status(reply.exception_status());
                msg.set_misc_device_state(reply.misc_device_state());
                msg.set_model_number(reply.model_number());
                msg.set_code_revision(reply.code_revision());

                msg
            }
        }

        impl From<$name> for $crate::OmnibusReply {
            fn from(reply: $name) -> Self {
                (&reply).into()
            }
        }
    };
}

/// Implements the Default trait for a named type with a `Self::new()` function.
#[macro_export]
macro_rules! impl_default {
    ($name:ident) => {
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };

    ($name:ident, $full_len:ident, $enable_len:ident) => {
        impl<const $full_len: usize, const $enable_len: usize> Default
            for $name<$full_len, $enable_len>
        {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

/// Provides convenience functions to deconstruct an enum with new-type variants.
#[macro_export]
macro_rules! inner_enum {
    // macro variant for when the enum variant and its type are the same ident
    ($ty:ident, $var:ident) => {
        inner_enum!($ty, $var, $var);
    };

    // macro variant for when the enum variant and its type are potentially different
    ($ty:ident, $var:ident, $var_ty:ident) => {
        impl $ty {
            ::paste::paste! {
                #[doc = "Gets whether `" $ty "` is the variant `" $var "`."]
                pub fn [<is_ $var:snake>](&self) -> bool {
                    matches!(self, $ty::$var(_))
                }

                #[doc = "Gets a reference to `" $ty "` as the variant `" $var "`'s inner type `" $var_ty "`."]
                pub fn [<as_ $var:snake>](&self) -> $crate::Result<&$var_ty> {
                    use $crate::Error;

                    match self {
                        $ty::$var(ty) => Ok(ty),
                        _ => Err(Error::failure(format!("have variant: {self}, expected: {}", $crate::std::any::type_name::<$var>()))),
                    }
                }

                #[doc = "Converts `" $ty "` into the variant `" $var "`'s inner type `" $var_ty "`."]
                pub fn [<into_ $var:snake>](self) -> $crate::Result<$var_ty> {
                    use $crate::Error;

                    match self {
                        $ty::$var(ty) => Ok(ty),
                        _ => Err(Error::failure(format!("have variant: {self}, expected: {}", $crate::std::any::type_name::<$var>()))),
                    }
                }
            }
        }
    };
}
