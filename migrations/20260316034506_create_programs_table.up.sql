-- Add up migration script here

CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);




CREATE TABLE programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    duration SMALLINT NOT NULL,
    difficulty VARCHAR(255) NOT NULL,
    category_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- Added missing comma here

    CONSTRAINT fk_category
        FOREIGN KEY(category_id)
        REFERENCES categories(id) -- Fixed spelling of "REFERENCES"
        ON DELETE CASCADE
);

CREATE INDEX idx_programs_title ON programs(title);
CREATE INDEX idx_programs_category_id ON programs(category_id);


-- Add up migration script here
CREATE TABLE users(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);

-- Add up migration script here

CREATE TABLE chapters(
id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
title VARCHAR(255) NOT NULL,
description    TEXT,    
program_id UUID NOT NULL,

created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),


CONSTRAINT fk_program
        FOREIGN KEY(program_id) 
        REFERENCES programs(id)
        ON DELETE CASCADE 
);


CREATE INDEX idx_chapters_title ON chapters(title);
CREATE INDEX idx_chapters_program_id ON chapters(program_id);


-- Add up migration script here
CREATE TABLE lessons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    is_project BOOLEAN NOT NULL DEFAULT FALSE,
    chapter_id UUID NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Foreign Key Constraint
    CONSTRAINT fk_chapter
        FOREIGN KEY(chapter_id) 
        REFERENCES chapters(id)
        ON DELETE CASCADE
);

-- Index for performance when fetching a chapter's lessons
CREATE INDEX idx_lessons_chapter_id ON lessons(chapter_id);

