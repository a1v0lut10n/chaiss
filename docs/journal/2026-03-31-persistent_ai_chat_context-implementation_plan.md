# Persistent AI Chat Context Architecture

Because switching matches correctly purged UI session leakage, our active logic permanently flushed Gemini's dialogue into the void! Since we aren't enforcing rigid migrations yet, we can natively blow away the local `.db` file and systematically wire the Chat logging pipeline identically to how we wired the Mechanical Move logging!

## Proposed Changes

### Database Layer (`tools/scripts/init_db.sh`)
#### [MODIFY] [init_db.sh](file:///home/hansvw/Projects/Aivolution/chaiss/tools/scripts/init_db.sh)
- Inject a brand new `chat_messages` table securely referencing the generic `games(id)` constraint.
- Table design: `id`, `game_id`, `role` (User/Agent), `content` (Markdown string), `created_at`.

### Core Data Services (`core/src/db.rs`)
#### [MODIFY] [db.rs](file:///home/hansvw/Projects/Aivolution/chaiss/core/src/db.rs)
- Implement `pub async fn log_chat_message(game_id, role, content)`.
- Implement `pub async fn load_chat_history(game_id) -> Vec<(String, String)>`.

### Application Routing Pipeline (`desktop/src/app.rs`)
#### [MODIFY] [app.rs](file:///home/hansvw/Projects/Aivolution/chaiss/desktop/src/app.rs)
- Modify the `DbEvent::GameResumed` payload to explicitly carry a populated `chat_history: Vec<(String, String)>` instead of clearing natively!
- Inside the Event loop processing `LlmEvent::InferenceRequested` (where User text is captured) $\rightarrow$ mathematically trigger the `db.log_chat_message()` pipeline asynchronously via Flume channels natively.
- Inside the Event loop processing `LlmEvent::InferenceFinished` (where the final Markdown stream completes) $\rightarrow$ trigger the `db.log_chat_message()` explicitly!

## User Review Required
> [!CAUTION]
> This pipeline mandates physically destroying `chaiss.db` to reconstruct the raw Table geometries smoothly. Your active games will be lost. Give me the green light and I'll formally tear down the DB, inject the Tables, and link the UI organically!
