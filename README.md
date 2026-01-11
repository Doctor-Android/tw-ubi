# TW-UBI

**Time-Weighted Universal Basic Income**

**Not an investment.**
**Not a DAO.**
**Not a stablecoin.**

**Just income — enforced by rules, not permission.**

---

## What This Is

TW-UBI is a reference implementation of a Universal Basic Income system designed as **income infrastructure**, not a financial product.

It guarantees every verified human the same recurring income, while structurally discouraging hoarding and preventing discretionary control.

This repository contains the **base system**: intentionally minimal, boring, and forkable.

---

## One-Sentence Explanation

**Everyone receives the same monthly income, but time makes hoarding it irrational.**

If that sentence makes sense to you, the rest is details.

---

## The Core Idea (Plain Language)

TW-UBI separates income from wealth.

### 🟢 UE — Universal Entitlement

- Monthly income
- Same amount for everyone
- Issued every 30 days
- Designed to be spent
- Hoarding is allowed, but discouraged by math

### 🟣 BU — Base Unit

- Scarce reserve asset
- Fixed supply
- Does not decay
- Used for long-term saving

**You never receive BU directly.**

You may convert UE → BU, but the conversion rate decays over time.

---

## Why Hoarding Doesn't Work

Your UE balance never disappears.

Instead, the **conversion power** of UE declines with time.

| Time | 1 UE converts to |
|------|------------------|
| Now | 1.00 BU |
| Later | 0.90 BU |
| Much later | 0.70 BU |

Income is preserved.

Speculation is not rewarded.

**Income is for living.**
**Wealth is for saving.**

TW-UBI enforces this by incentives, not restrictions.

---

## How Payments Work

- Time is divided into fixed 30-day periods ("epochs")
- Each epoch, every verified person may claim income once
- Claims are automatic, predictable, and equal
- No applications.
- No means testing.
- No discretion.

---

## How Much Income?

In the base system:

- **696 UE per epoch**
- Same for everyone
- Frozen at deployment
- Changing this requires a hard fork.

---

## Identity & Security

- Identity is bound to a `personId`, not a wallet
- Wallets can be rotated if lost
- Rotation requires multi-factor authentication
- National identity documents are verified off-chain
- The system never stores real-world identifiers

**A leaked database or document dump alone is not enough to steal funds.**

---

## Countries & Regions

TW-UBI is country-agnostic.

Countries are implemented as **identity adapters**, not protocol rules.

Any country can be added without changing the system:

- Spain
- USA
- Mexico
- Any other

If a country wants different economics:

**they fork**

---

## Is This Decentralized?

- **Execution**: centralized (for now)
- **Rules**: immutable
- **Exit**: fully decentralized

Anyone can:

- export the full event history
- re-run the system
- fork the rules
- invite others to migrate

**No one can change the rules here.**

---

## Forking Is a Feature

Disagreement is expected.

Instead of governance battles, TW-UBI offers **exit**.

If you want:

- different income levels
- different decay rules
- different identity assumptions
- different philosophy

**Please fork this project.**

This repository is a reference, not an authority.

---

## What This Is NOT

TW-UBI is not:

- an investment
- a DAO
- a governance token
- a stablecoin
- a welfare program
- a promise of returns

There is:

- no voting
- no emissions tuning
- no discretionary minting
- no "just this once" changes

---

## Design Principles

- Time, not politics, controls outcomes
- Exit is healthier than governance
- Boring systems last longer
- Infrastructure should be auditable

---

## License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

### Why AGPL?

TW-UBI is income infrastructure.

People who rely on it should be able to inspect how it works.

AGPL-3.0 ensures that:

- anyone may run or modify the system
- anyone may deploy it as a service
- any deployed version remains auditable by its users

### About forks

Forking is encouraged.

If you can improve this system, simplify it, adapt it, or prove parts of it wrong — that is a success.

This project does not claim to have solved UBI.

It is one attempt to encode a small set of rules worth testing.

If this is useful, I hope it gets copied.

If it's flawed, I hope better forks make that obvious.

---

## Status

This repository contains:

- a centralized reference implementation
- with decentralized guarantees
- intended to be forked, audited, and re-implemented

Full infrastructure decentralization is a separate step, not a prerequisite.

---

## Final Note

If you're looking for:

- price appreciation
- governance power
- yield
- influence

**This is not for you.**

If you're looking for:

- predictable income
- hard guarantees
- minimal trust
- the right to leave

**Welcome.**

---

**Not an investment.**
**Not a DAO.**
**Just income — by rule, not permission.**
