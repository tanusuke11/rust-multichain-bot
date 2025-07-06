import { drizzle } from "drizzle-orm/better-sqlite3";
import { migrate } from "drizzle-orm/better-sqlite3/migrator";
import Database from "better-sqlite3";
import * as schema from "./schema";

let db: ReturnType<typeof drizzle>;

export function initializeDatabase(databaseUrl: string) {
  const sqlite = new Database(databaseUrl);
  db = drizzle(sqlite, { schema });

  // Run migrations
  migrate(db, { migrationsFolder: "./db/migrations" });

  return db;
}

export function getDatabase() {
  if (!db) {
    throw new Error("Database not initialized. Call initializeDatabase first.");
  }
  return db;
}

export * from "./schema";
