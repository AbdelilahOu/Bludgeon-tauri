migrationup:
	cd src-tauri && sea-orm-cli migrate up

migrationlast:
	cd src-tauri && sea-orm-cli migrate down

migrationdown:
	cd src-tauri && sea-orm-cli migrate fresh

entity:
	cd src-tauri && sea-orm-cli generate entity --lib -u sqlite://stocker.sqlite?mode=rwc -o entity/src

dev:
	npm run tauri dev

build: 
	npm run tauri build

check:
	cd src-tauri && cargo check

lint:
	npx eslint src/**/** --fix
	

migration: 
	cd src-tauri && sea-orm-cli migrate generate $(name)