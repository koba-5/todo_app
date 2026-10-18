CREATE TABLE public."task_history" (
    "task_id" BIGINT,
    "version" INT,
    "user_id" BIGINT NOT NULL,
    "title" VARCHAR(50) NOT NULL,
    "status" VARCHAR(20) NOT NULL,
    "start_date" TIMESTAMP WITH TIME ZONE NOT NULL,
    "end_date" TIMESTAMP WITH TIME ZONE,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("task_id", version),
    FOREIGN KEY ("user_id") REFERENCES public."user" ("user_id")
);
