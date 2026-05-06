import express from "express";
import cors from "cors";
import helmet from "helmet";
import { env } from "./env";
import { healthRouter } from "./routes/health";
import { authRouter } from "./routes/auth";
import { vaultsRouter } from "./routes/vaults";
import { strategiesRouter } from "./routes/strategies";
import { positionsRouter } from "./routes/positions";
import { dashboardRouter } from "./routes/dashboard";
import { depositsRouter } from "./routes/deposits";
import { withdrawalsRouter } from "./routes/withdrawals";

const app = express();

app.use(helmet());
app.use(cors({ origin: env.corsOrigin, credentials: true }));
app.use(express.json({ limit: "1mb" }));

app.use("/health", healthRouter);
app.use("/auth", authRouter);
app.use("/vaults", vaultsRouter);
app.use("/strategies", strategiesRouter);
app.use("/positions", positionsRouter);
app.use("/dashboard", dashboardRouter);
app.use("/deposits", depositsRouter);
app.use("/withdrawals", withdrawalsRouter);

app.use((_req, res) => res.status(404).json({ error: "not_found" }));

app.use(
  (
    err: unknown,
    _req: express.Request,
    res: express.Response,
    _next: express.NextFunction,
  ) => {
    console.error(err);
    res.status(500).json({
      error: "internal_error",
      message: err instanceof Error ? err.message : String(err),
    });
  },
);

app.listen(env.port, () => {
  console.log(`yieldveil-server listening on http://localhost:${env.port}`);
});
