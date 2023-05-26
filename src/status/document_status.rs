use crate::std::fmt;

use crate::{
    banknote::{BanknoteOrientation, NoteTableItem},
    denomination::StandardDenomination,
};

/// An accepted [NoteTableItem].
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AcceptedNoteTableItem {
    note_table_item: NoteTableItem,
    banknote_orientation: BanknoteOrientation,
}

impl AcceptedNoteTableItem {
    /// Creates a new [AcceptedNoteTableItem].
    pub const fn new(
        note_table_item: NoteTableItem,
        banknote_orientation: BanknoteOrientation,
    ) -> Self {
        Self {
            note_table_item,
            banknote_orientation,
        }
    }

    /// Creates a default [AcceptedNoteTableItem].
    pub const fn default() -> Self {
        Self {
            note_table_item: NoteTableItem::default(),
            banknote_orientation: BanknoteOrientation::default(),
        }
    }

    /// Gets the [NoteTableItem].
    pub fn note_table_item(&self) -> &NoteTableItem {
        &self.note_table_item
    }

    /// Sets the [NoteTableItem], consumes and returns the [NoteTableItem].
    pub fn with_note_table_item(mut self, note_table_item: NoteTableItem) -> Self {
        self.note_table_item = note_table_item;
        self
    }

    /// Gets the [BanknoteOrientation].
    pub fn banknote_orientation(&self) -> &BanknoteOrientation {
        &self.banknote_orientation
    }

    /// Sets the [BanknoteOrientation], consumes and returns the [BanknoteOrientation].
    pub fn with_banknote_orientation(mut self, banknote_orientation: BanknoteOrientation) -> Self {
        self.banknote_orientation = banknote_orientation;
        self
    }
}

impl fmt::Display for AcceptedNoteTableItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (item, orientation) = (self.note_table_item(), self.banknote_orientation());
        write!(
            f,
            "Note table item: {item}, Banknote orientation: {orientation}"
        )
    }
}

/// Values that represent document events.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocumentEvent {
    DispensedEvent = 0,
    EscrowedEvent = 1,
    RejectedEvent = 2,
    RetrievedEvent = 3,
    ReturnedEvent = 4,
    StackedEvent = 5,
    MissingNoteReportReadyEvent = 6,
    EscrowSessionSummaryReportReadyEvent = 7,
    NoneEvent = 8,
}

impl DocumentEvent {
    /// Creates a default [DocumentEvent].
    pub const fn default() -> Self {
        Self::NoneEvent
    }
}

impl From<DocumentEvent> for &'static str {
    fn from(d: DocumentEvent) -> Self {
        match d {
            DocumentEvent::DispensedEvent => "Dispensed event",
            DocumentEvent::EscrowedEvent => "Escrowed event",
            DocumentEvent::RejectedEvent => "Rejected event",
            DocumentEvent::RetrievedEvent => "Retrieved event",
            DocumentEvent::ReturnedEvent => "Returned event",
            DocumentEvent::StackedEvent => "Stacked event",
            DocumentEvent::MissingNoteReportReadyEvent => "Missing note report ready event",
            DocumentEvent::EscrowSessionSummaryReportReadyEvent => {
                "Escrow session summary report ready event"
            }
            DocumentEvent::NoneEvent => "None event",
        }
    }
}

impl From<&DocumentEvent> for &'static str {
    fn from(d: &DocumentEvent) -> Self {
        (*d).into()
    }
}

impl fmt::Display for DocumentEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

/// Values that represent document routing directions.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocumentRouting {
    NoRoute = 0,
    EscrowToRecycler = 1,
    RecyclerToCashbox = 2,
    RecyclerToRecycler = 3,
    RecyclerToCustomer = 4,
    EscrowToEscrowStorage = 8,
    CustomerToCashbox = 129,
    EscrowStorageToInventory = 131,
}

impl DocumentRouting {
    /// Creates a default [DocumentRouting].
    pub const fn default() -> Self {
        Self::NoRoute
    }
}

impl From<DocumentRouting> for &'static str {
    fn from(d: DocumentRouting) -> Self {
        match d {
            DocumentRouting::NoRoute => "No route",
            DocumentRouting::EscrowToRecycler => "Escrow to recycler",
            DocumentRouting::RecyclerToCashbox => "Recycler to cashbox",
            DocumentRouting::RecyclerToRecycler => "Recycler to recycler",
            DocumentRouting::RecyclerToCustomer => "Recycler to customer",
            DocumentRouting::EscrowToEscrowStorage => "Escrow to escrow storage",
            DocumentRouting::CustomerToCashbox => "Customer to cashbox",
            DocumentRouting::EscrowStorageToInventory => "Escrow storage to inventory",
        }
    }
}

impl From<&DocumentRouting> for &'static str {
    fn from(d: &DocumentRouting) -> Self {
        (*d).into()
    }
}

impl fmt::Display for DocumentRouting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

/// A document status.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DocumentStatus {
    /// The [DocumentEvent].
    document_event: DocumentEvent,
    /// The [DocumentRouting].
    document_routing: DocumentRouting,
    /// The [AcceptedNoteTableItem].
    accepted_note_table_item: AcceptedNoteTableItem,
    /// The [StandardDenomination].
    standard_denomination: StandardDenomination,
}

impl fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (event, routing, item, denom) = (
            &self.document_event,
            &self.document_routing,
            &self.accepted_note_table_item,
            &self.standard_denomination,
        );

        write!(f, "Document event: {event}, Document routing: {routing}, Accepted note table item: {item}, Standard denomination: {denom}")
    }
}

impl DocumentStatus {
    /// Creates a new [DocumentStatus].
    pub const fn new(
        document_event: DocumentEvent,
        document_routing: DocumentRouting,
        accepted_note_table_item: AcceptedNoteTableItem,
        standard_denomination: StandardDenomination,
    ) -> Self {
        Self {
            document_event,
            document_routing,
            accepted_note_table_item,
            standard_denomination,
        }
    }

    /// Creates a default [DocumentStatus].
    pub const fn default() -> Self {
        Self {
            document_event: DocumentEvent::default(),
            document_routing: DocumentRouting::default(),
            accepted_note_table_item: AcceptedNoteTableItem::default(),
            standard_denomination: StandardDenomination::none(),
        }
    }

    /// Gets the [DocumentEvent].
    pub fn document_event(&self) -> &DocumentEvent {
        &self.document_event
    }

    /// Sets the [DocumentEvent].
    pub fn set_document_event(&mut self, document_event: DocumentEvent) {
        self.document_event = document_event;
    }

    /// Sets the [DocumentEvent], consumes and returns the [DocumentStatus].
    pub fn with_document_event(mut self, document_event: DocumentEvent) -> Self {
        self.document_event = document_event;
        self
    }

    /// Gets the [DocumentRouting].
    pub fn document_routing(&self) -> &DocumentRouting {
        &self.document_routing
    }

    /// Sets the [DocumentRouting].
    pub fn set_document_routing(&mut self, document_routing: DocumentRouting) {
        self.document_routing = document_routing;
    }

    /// Sets the [DocumentRouting], consumes and returns the [DocumentStatus].
    pub fn with_document_routing(mut self, document_routing: DocumentRouting) -> Self {
        self.document_routing = document_routing;
        self
    }

    /// Gets the [AcceptedNoteTableItem].
    pub fn accepted_note_table_item(&self) -> &AcceptedNoteTableItem {
        &self.accepted_note_table_item
    }

    /// Sets the [AcceptedNoteTableItem].
    pub fn set_accepted_note_table_item(
        &mut self,
        accepted_note_table_item: AcceptedNoteTableItem,
    ) {
        self.accepted_note_table_item = accepted_note_table_item;
    }

    /// Sets the [AcceptedNoteTableItem], consumes and returns the [DocumentStatus].
    pub fn with_accepted_note_table_item(
        mut self,
        accepted_note_table_item: AcceptedNoteTableItem,
    ) -> Self {
        self.accepted_note_table_item = accepted_note_table_item;
        self
    }

    /// Gets the [StandardDenomination].
    pub fn standard_denomination(&self) -> StandardDenomination {
        self.standard_denomination
    }

    /// Sets the [StandardDenomination].
    pub fn set_standard_denomination(&mut self, standard_denomination: StandardDenomination) {
        self.standard_denomination = standard_denomination;
    }

    /// Sets the [StandardDenomination], consumes and returns the [DocumentStatus].
    pub fn with_standard_denomination(
        mut self,
        standard_denomination: StandardDenomination,
    ) -> Self {
        self.standard_denomination = standard_denomination;
        self
    }
}
