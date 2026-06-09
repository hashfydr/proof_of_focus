# Proof of Focus (PoF)
tool that actually measures productivity and donot trust the user

This repo is a **work-in-progress learning project**.

I’m building Proof of Focus to understand **how trust, verification, and user intent actually work in software**, instead of just reading about them.

It started as a simple focus timer.
It slowly turned into a system that asks a harder question:

> *How do you know a record is real, and when should you stop trusting it?*

---

## What PoF currently is

* A **CLI tool** to track focus sessions
* Stores everything **locally** (no cloud, no accounts)
* Keeps a **tamper-evident history** using hash chaining
* Has the idea of **epochs** (trust periods)

This is not polished. That’s intentional.

---

## The main idea (in simple words)

PoF separates three things:

* **Recording** → sessions are always logged
* **Verification** → history is checked for tampering
* **Trust** → whether the system claims the history is valid

If you mess with the history file:

* PoF does *not* crash
* PoF does *not* auto-fix
* PoF clearly says: *this epoch is no longer trusted*

You can still keep using it.
But trust is gone unless you **explicitly reset**.

---

## Epochs (why they exist)

An **epoch** is one continuous trust period.

* When you start using PoF → Epoch 1 starts
* When you reset → Epoch 1 ends, Epoch 2 starts

Nothing is deleted.
Resets are visible.
History tells a story instead of hiding mistakes.

---

## Reset is intentional

```bash
pof reset
```

Reset does **not** mean:

* uninstalling
* deleting your data
* pretending nothing happened

It means:

> *I know trust is broken or I want a fresh start, and I’m okay with that being recorded.*

---

## Commands (current)

```bash
pof start     # start a focus session
pof stop      # stop the current session
pof status    # see if a session is running
pof history   # view epochs + sessions
pof reset     # explicitly start a new trust period
```

---

## Example output (rough)

```
📊 Proof of Focus — History

User started using PoF on: 02 Feb 2026, 19:28

🔵 Epoch 1
Started at: 02 Feb 2026, 19:28
Ended at:   02 Feb 2026, 19:40 (reset)
Sessions: 1
Status: ✅ Trusted

🟢 Epoch 2 (Current)
Started at: 02 Feb 2026, 19:48
Sessions: 0
```

---

## What this project is NOT (yet)

* Not a finished product
* Not a productivity app for everyone
* Not optimized
* Not pretty

It’s a **learning-first systems project**.



## What might come next

No promises, but possibly:

* better warnings when trust breaks
* reset confirmations
* exporting proofs
* anchoring hashes on-chain (Solana)

Only if it makes sense.

---

## Status

Actively being built.
Design may change.
Commits represent real l

