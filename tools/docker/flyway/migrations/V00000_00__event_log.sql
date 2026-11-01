CREATE TABLE public."event_log" (
    "event_log_id" BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    "session_unique_id" VARCHAR(36) NOT NULL,
    "user_id" BIGINT NOT NULL,
    "event_type" VARCHAR(50) NOT NULL,
    "event_name" VARCHAR(50) NOT NULL,
    "event_timestamp" TIMESTAMP WITH TIME ZONE NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
