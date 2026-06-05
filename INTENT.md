# INTENT — meta-signal-domain-criome

*The meta (owner-only policy) wire contract for the `domain-criome` component.
Defines the typed request/reply channel that the domain owner uses to register,
delegate, and retire domains, set projection policy, and declare provider-neutral
projection state.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `meta-signal-domain-criome`
contract. Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `domain-criome/INTENT.md`. Ordinary domain
resolution and projection stays in `signal-domain-criome/INTENT.md`.

## Why this repo exists

`meta-signal-domain-criome` is the **owner-only meta policy contract** for the
`domain-criome` component. It controls domain registration, delegation,
retirement, projection policy, and provider-neutral projection declarations. The
ordinary `signal-domain-criome` contract resolves and projects domain meaning;
this meta contract mutates the registry that gives that meaning authority.

Provider-specific plan application stays outside this contract: the domain
registry decides what should exist, while `cloud` decides how a provider applies
it.

## The channel shape

The meta channel carries:

- **`RegisterDomain(Registration)`** — register a domain root.
- **`Delegate(Delegation)`** — delegate a named branch.
- **`RetireDomain(Retirement)`** — retire a registered domain.
- **`SetPolicy(Policy)`** — change projection policy.
- **`SetProjection(ProjectionDeclaration)`** — record the provider-neutral DNS and
  redirect state a registered domain should project.

Replies carry accepted mutations and typed owner rejections.

## Constraints

- Owner-only registry-mutating authority enters through this crate; ordinary
  resolution and projection stay in `signal-domain-criome`.
- Provider names stay out of this contract — the registry is provider-neutral;
  `cloud` owns provider vocabulary.
- Depend on `signal-frame`, not deprecated `signal-core`. Reuse public domain
  types from `signal-domain-criome`.
- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses — no daemon actor tree or database.
- When this contract moves from `signal_channel!` to schema-derived generation,
  its schema lives in this repository and carries only meta Signal wire vocabulary
  (Input roots for the registry mutations, Output roots for accepted mutations and
  typed owner rejections, and the payload types that cross the meta wire). Nexus
  decisions, SEMA state, registry tables, and projection runtime stay in
  `domain-criome`.

## Non-ownership

This crate does not own:

- Cloudflare, Google, Hetzner, or other provider vocabulary;
- provider credentials or external API mutation;
- the runtime daemon's actor tree or database;
- ordinary domain resolution/projection traffic (lives in `signal-domain-criome`).

## See also

- `ARCHITECTURE.md` — public operations, the registry boundary, and the schema track.
- `../domain-criome/INTENT.md` — daemon-side intent (registry, delegation, projection runtime).
- `../signal-domain-criome/INTENT.md` — ordinary resolution/projection contract.
- `../meta-signal-cloud/INTENT.md` — sibling provider-application meta contract.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
