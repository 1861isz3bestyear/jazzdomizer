create table if not exists songs (
    id serial primary key,
    song_name text not null,
    song_version text
);

create table if not exists harmony (
    id serial primary key,
    song_id integer not null unique references songs(id) on delete cascade,
    chords text not null,
    key text not null
);

create table if not exists list_data (
    id serial primary key,
    song_id integer not null unique references songs(id) on delete cascade,
    key_alteration integer not null default 0,
    complexity integer not null default 0,
    global_playlist boolean not null default false,
    local_playlist boolean not null default false,
    song_for_study boolean not null default false
);

create index if not exists idx_songs_song_name on songs (song_name);
create index if not exists idx_list_data_song_id on list_data (song_id);
