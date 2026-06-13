//! Meta Signal contract for the domain-criome component.
//!
//! This crate carries meta policy domain registry and projection-policy records.

#[cfg(not(feature = "nota-text"))]
use nota_next::{Block, NotaDecodeError};
use nota_next::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

pub mod schema;

pub use signal_domain_criome::{
    DelegationName, DelegationTarget, DomainName, DomainNameSystemRecord, ProjectionScope,
    RecordKind, RecordValue, RedirectRule,
};

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Registration {
    pub domain: DomainName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Delegation {
    pub name: DelegationName,
    pub domain: DomainName,
    pub target: DelegationTarget,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Retirement {
    pub domain: DomainName,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ProjectionDirective {
    Enable,
    Disable,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ProjectionPolicy {
    pub domain: DomainName,
    pub scope: ProjectionScope,
    pub directive: ProjectionDirective,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Policy {
    pub projections: Vec<ProjectionPolicy>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ProjectionDeclaration {
    pub domain: DomainName,
    pub records: Vec<DomainNameSystemRecord>,
    pub redirects: Vec<RedirectRule>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DomainRegistered {
    pub domain: DomainName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DomainDelegated {
    pub name: DelegationName,
    pub domain: DomainName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DomainRetired {
    pub domain: DomainName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PolicySet {
    pub projection_policy_count: u64,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ProjectionSet {
    pub domain: DomainName,
    pub record_count: u64,
    pub redirect_count: u64,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RejectionReason {
    DomainAlreadyRegistered,
    DomainUnknown,
    DelegationAlreadyExists,
    DelegationUnknown,
    ProjectionUnavailable,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestRejected {
    pub operation: OperationKind,
    pub reason: RejectionReason,
}

signal_channel! {
    channel Meta {
        operation RegisterDomain(Registration),
        operation Delegate(Delegation),
        operation RetireDomain(Retirement),
        operation SetPolicy(Policy),
        operation SetProjection(ProjectionDeclaration),
    }
    reply Reply {
        DomainRegistered(DomainRegistered),
        DomainDelegated(DomainDelegated),
        DomainRetired(DomainRetired),
        PolicySet(PolicySet),
        ProjectionSet(ProjectionSet),
        RequestRejected(RequestRejected),
    }
}

#[cfg(not(feature = "nota-text"))]
impl OperationKind {
    const fn as_nota_atom(self) -> &'static str {
        match self {
            Self::RegisterDomain => "RegisterDomain",
            Self::Delegate => "Delegate",
            Self::RetireDomain => "RetireDomain",
            Self::SetPolicy => "SetPolicy",
            Self::SetProjection => "SetProjection",
        }
    }

    fn from_nota_atom(atom: &str) -> Result<Self, NotaDecodeError> {
        match atom {
            "RegisterDomain" => Ok(Self::RegisterDomain),
            "Delegate" => Ok(Self::Delegate),
            "RetireDomain" => Ok(Self::RetireDomain),
            "SetPolicy" => Ok(Self::SetPolicy),
            "SetProjection" => Ok(Self::SetProjection),
            variant => Err(NotaDecodeError::UnknownVariant {
                enum_name: "OperationKind",
                variant: variant.to_owned(),
            }),
        }
    }
}

#[cfg(not(feature = "nota-text"))]
impl NotaEncode for OperationKind {
    fn to_nota(&self) -> String {
        self.as_nota_atom().to_owned()
    }
}

#[cfg(not(feature = "nota-text"))]
impl NotaDecode for OperationKind {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let atom = block
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom {
                type_name: "OperationKind",
            })?;
        Self::from_nota_atom(atom)
    }
}

pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<Reply>;

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
