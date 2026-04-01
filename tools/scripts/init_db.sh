#!/bin/bash
set -e

# Change into the project root
cd "$(dirname "$0")/../.."

echo "Initializing SQLite Database 'chaiss.db' locally..."

sqlite3 chaiss.db <<EOF
CREATE TABLE IF NOT EXISTS players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL DEFAULT 'Untitled Game',
    white_player_id INTEGER NOT NULL REFERENCES players(id),
    black_player_id INTEGER NOT NULL REFERENCES players(id),
    current_fen TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS moves (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL REFERENCES games(id),
    move_number INTEGER NOT NULL,
    fen_snapshot TEXT NOT NULL,
    notation TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS chat_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL REFERENCES games(id),
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
EOF

echo "Database schemas successfully instantiated natively without relying on heavy cargo cli installs!"
