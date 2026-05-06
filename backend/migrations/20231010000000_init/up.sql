CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR NOT NULL UNIQUE CHECK (email LIKE '%@students.uonbi.ac.ke'),
    password VARCHAR NOT NULL,
    role VARCHAR NOT NULL DEFAULT 'student',
    verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE faculties (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL
);

CREATE TABLE departments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL,
    faculty_id UUID NOT NULL REFERENCES faculties(id)
);

CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    slogan VARCHAR,
    mission TEXT,
    vision TEXT,
    faculty_id UUID REFERENCES faculties(id),
    department_id UUID REFERENCES departments(id),
    logo_url VARCHAR
);

CREATE TABLE positions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    name VARCHAR NOT NULL,
    description TEXT,
    eligibility_rules TEXT
);

CREATE TABLE candidates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    position_id UUID NOT NULL REFERENCES positions(id),
    approved BOOLEAN NOT NULL DEFAULT false,
    manifesto TEXT,
    votes_count INT NOT NULL DEFAULT 0
);

CREATE TABLE votes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    candidate_id UUID NOT NULL REFERENCES candidates(id),
    position_id UUID NOT NULL REFERENCES positions(id),
    timestamp TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, position_id)
);

-- Insert initial data
INSERT INTO faculties (id, name) VALUES ('f1f1f1f1-f1f1-f1f1-f1f1-f1f1f1f1f1f1', 'Faculty of Science and Technology');
INSERT INTO departments (id, name, faculty_id) VALUES ('d1d1d1d1-d1d1-d1d1-d1d1-d1d1d1d1d1d1', 'Department of Computer Science', 'f1f1f1f1-f1f1-f1f1-f1f1-f1f1f1f1f1f1');
INSERT INTO organizations (id, name, description, slogan, faculty_id) VALUES ('01010101-0101-0101-0101-010101010101', 'ONUSS', 'Organization of Nairobi University Science Students', 'Science for Society', 'f1f1f1f1-f1f1-f1f1-f1f1-f1f1f1f1f1f1');
