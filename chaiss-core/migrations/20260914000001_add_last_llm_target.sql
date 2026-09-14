-- The provider/model that served the game's most recent LLM request
-- (e.g. 'openai/gpt-6-astra'); NULL until the first request. Restored
-- on resume when that target is still configured.
ALTER TABLE games ADD COLUMN last_llm_target TEXT;
