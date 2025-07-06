import { sqliteTable, text, integer, real } from "drizzle-orm/sqlite-core";
import { createInsertSchema, createSelectSchema } from "drizzle-zod";
import { z } from "zod";

// Strategies table
export const strategies = sqliteTable("strategies", {
  id: text("id").primaryKey(),
  name: text("name").notNull(),
  type: text("type").notNull(), // 'atomic_arb', 'liquidator', etc.
  chainId: text("chain_id").notNull(),
  isActive: integer("is_active", { mode: "boolean" }).default(false),
  parameters: text("parameters", { mode: "json" }).notNull(), // JSON string
  createdAt: integer("created_at", { mode: "timestamp" }).notNull(),
  updatedAt: integer("updated_at", { mode: "timestamp" }).notNull(),
});

// Strategy execution history
export const executions = sqliteTable("executions", {
  id: text("id").primaryKey(),
  strategyId: text("strategy_id")
    .references(() => strategies.id)
    .notNull(),
  status: text("status").notNull(), // 'pending', 'running', 'completed', 'failed'
  startedAt: integer("started_at", { mode: "timestamp" }).notNull(),
  completedAt: integer("completed_at", { mode: "timestamp" }),
  result: text("result", { mode: "json" }), // JSON result data
  errorMessage: text("error_message"),
  profit: real("profit").default(0),
  gasUsed: real("gas_used").default(0),
});

// Performance metrics
export const metrics = sqliteTable("metrics", {
  id: text("id").primaryKey(),
  strategyId: text("strategy_id")
    .references(() => strategies.id)
    .notNull(),
  executionId: text("execution_id").references(() => executions.id),
  metricType: text("metric_type").notNull(), // 'profit', 'gas', 'slippage', etc.
  value: real("value").notNull(),
  timestamp: integer("timestamp", { mode: "timestamp" }).notNull(),
});

// Zod schemas for validation
export const insertStrategySchema = createInsertSchema(strategies, {
  parameters: z.record(z.any()),
});

export const selectStrategySchema = createSelectSchema(strategies);

export const insertExecutionSchema = createInsertSchema(executions, {
  result: z.record(z.any()).optional(),
});

export const selectExecutionSchema = createSelectSchema(executions);

export const insertMetricSchema = createInsertSchema(metrics);
export const selectMetricSchema = createSelectSchema(metrics);

export type Strategy = z.infer<typeof selectStrategySchema>;
export type NewStrategy = z.infer<typeof insertStrategySchema>;
export type Execution = z.infer<typeof selectExecutionSchema>;
export type NewExecution = z.infer<typeof insertExecutionSchema>;
export type Metric = z.infer<typeof selectMetricSchema>;
export type NewMetric = z.infer<typeof insertMetricSchema>;
