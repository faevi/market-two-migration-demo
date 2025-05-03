# MarketTwo Migration Demo

A concise test assignment showing how to add a `flash_swap_fee_bps: u32` field to the `MarketTwo` account in Anchor v0.31 on Solana. Two approaches are provided to handle existing and updated account layouts.

---

## 🔧 Setup

Install and build the custom fork of [`anchor-bankrun`](https://github.com/IaroslavMazur/anchor-bankrun/tree/iaro/update-anchor):

```bash
yarn build:anchor-bankrun
```

---

## 🛠 Migration Approaches

Two methods to extend `MarketTwo` with a new `flash_swap_fee_bps` field:

### 1. Standard `Account<T>` Migration

* Uses Anchor’s `realloc` on `Account<'info, MarketTwo>`.
* **Pros**: minimal code changes.
* **Cons**: instructions expecting the new field will fail until each account is migrated.

### 2. Custom Deserialization Migration

* Works with raw `AccountInfo`, detects old vs. new layout at runtime.
* On first access: reads the old layout, appends the new field, calls `realloc`, and writes the updated data.
* **Pros**: backward-compatible; existing instructions continue to work.
* **Cons**: requires manual (de)serialization in each instruction.

---

## ⚙️ Usage

Add these `package.json` scripts:

```jsonc
{
  "scripts": {
    "build-simple-test": "anchor build",
    "build-migration":   "anchor build -- --features migration"
  }
}
```

* `build-simple-test`: builds the program without the migration feature.
* `build-migration`: builds with the `migration` feature enabled.

---

*This repository is part of a technical exercise and demonstrates careful handling of on-chain account changes.*
