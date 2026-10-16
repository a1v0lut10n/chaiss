# CHAI-0048 — move the Google default to gemini-3.8-flash

- **Ticket:** CHAI-0048
- **Branch:** `chore/CHAI-0048-gemini-3-8-flash`
- **Status:** implemented (PR pending review)

## Premise

Google has deprecated `gemini-3.7-flash`, the default Google model since
0.3.0. A game turn surfaced "The AI backend gemini-3.7-flash (Google) is
rate-limiting requests or your quota is exhausted", which was read as a
side effect of the deprecation.

Probing the API directly (2026-10-16) showed otherwise: `gemini-3.7-flash`
and `gemini-3.8-flash` both still appear in the models listing, and both
answered the same way, HTTP 402 with the body "Your prepayment credits are
depleted" and status `RESOURCE_EXHAUSTED`. `classify_backend_error` keys
the rate-limit message on that status string, so a billing problem was
reported as throttling. The model bump is still due; the misleading
message is the bug.

## Action items

1. `DEFAULT_GOOGLE_MODEL` becomes `gemini-3.8-flash`; tests, `.env.example`,
   `.chaiss.cola.example` and the user guide follow. The config overrides
   are unchanged, so a `model` pinned in `.chaiss.cola` still wins.
2. `classify_backend_error` recognises HTTP 402 / "credits are depleted"
   ahead of the rate-limit check and names the billing cause; a unit test
   carries Google's verbatim 402 body.
3. CHANGELOG under `[Unreleased]`.

## Out of scope

- Pointing the not-found message at `.chaiss.cola` as well as `.env`.
- Live verification of a reply from `gemini-3.8-flash`: blocked on the
  depleted balance; the models listing confirms the id exists.
