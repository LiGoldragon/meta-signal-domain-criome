# meta-signal-domain-criome Architecture

`meta-signal-domain-criome` is the meta policy Signal contract for the
`domain-criome` component. It controls domain registration, delegation,
retirement, projection policy, and provider-neutral projection declarations.

## Boundary

The ordinary `signal-domain-criome` contract resolves and projects domain
meaning. This meta contract mutates the registry that gives that meaning
authority.

Provider-specific plan application remains outside this contract. The domain
registry can decide what should exist; `cloud` decides how a provider applies
it.

## Public Operations

- `RegisterDomain(Registration)` registers a domain root.
- `Delegate(Delegation)` delegates a named branch.
- `RetireDomain(Retirement)` retires a registered domain.
- `SetPolicy(Policy)` changes projection policy.
- `SetProjection(ProjectionDeclaration)` records the provider-neutral DNS and
  redirect state a registered domain should project.

## Owns

- Domain-registration authority.
- Delegation authority.
- Projection-policy directives.
- Provider-neutral projection declarations.
- Typed meta-policy rejections.

## Does Not Own

- Cloudflare, Google, Hetzner, or other provider vocabulary.
- Provider credentials.
- External API mutation.
- The runtime daemon's actor tree or database.

## Constraints

- Depend on `signal-frame`, not deprecated `signal-core`.
- Reuse public domain types from `signal-domain-criome`.
- Keep provider names out of this contract.

## Schema-engine upgrade track

When this contract moves from `signal_channel!` to schema-derived generation,
its schema lives in this repository and carries only meta Signal wire
vocabulary:

- `Input` roots for domain registration, delegation, retirement, projection
  policy, and projection declaration mutations.
- `Output` roots for accepted mutations and typed meta-policy rejections.
- Domain, delegation, policy, and projection-declaration payload types that
  cross the meta Signal wire.

Nexus decisions, SEMA state, registry tables, projection runtime, and daemon
storage schemas live in `domain-criome`, not here. Ordinary resolution and
projection messages live in `signal-domain-criome`, not in this meta contract.
