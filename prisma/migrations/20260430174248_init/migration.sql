-- CreateEnum
CREATE TYPE "VaultStatus" AS ENUM ('active', 'paused', 'deprecated');

-- CreateEnum
CREATE TYPE "RiskLevel" AS ENUM ('low', 'medium', 'high');

-- CreateEnum
CREATE TYPE "PoolType" AS ENUM ('amm', 'clmm', 'orderbook');

-- CreateEnum
CREATE TYPE "PayloadType" AS ENUM ('rebalance_input', 'order_data', 'strategy_params');

-- CreateEnum
CREATE TYPE "PayloadStatus" AS ENUM ('pending', 'processing', 'computed', 'expired');

-- CreateEnum
CREATE TYPE "OrderStatus" AS ENUM ('pending', 'queued', 'processing', 'filled', 'cancelled');

-- CreateEnum
CREATE TYPE "OrderbookStatus" AS ENUM ('active', 'paused');

-- CreateEnum
CREATE TYPE "TxStatus" AS ENUM ('pending', 'confirmed', 'failed');

-- CreateEnum
CREATE TYPE "PositionStatus" AS ENUM ('active', 'closed');

-- CreateEnum
CREATE TYPE "CronJobType" AS ENUM ('scan', 'rebalance', 'compound');

-- CreateEnum
CREATE TYPE "CronJobStatus" AS ENUM ('active', 'paused', 'failed');

-- CreateEnum
CREATE TYPE "LogStatus" AS ENUM ('success', 'failure', 'timeout');

-- CreateTable
CREATE TABLE "users" (
    "id" TEXT NOT NULL,
    "wallet_address" TEXT NOT NULL,
    "display_name" TEXT,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tokens" (
    "id" TEXT NOT NULL,
    "mint_address" TEXT NOT NULL,
    "symbol" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "decimals" INTEGER NOT NULL,
    "logo_url" TEXT,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "tokens_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "protocols" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "chain" TEXT NOT NULL DEFAULT 'solana',
    "contract_address" TEXT NOT NULL,
    "website_url" TEXT,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "protocols_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pools" (
    "id" TEXT NOT NULL,
    "protocol_id" TEXT NOT NULL,
    "pool_address" TEXT NOT NULL,
    "pool_type" "PoolType" NOT NULL,
    "token_a_id" TEXT NOT NULL,
    "token_b_id" TEXT NOT NULL,
    "fee_tier" DECIMAL(65,30),
    "current_apy" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "tvl" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "risk_score" INTEGER,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "pools_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "vaults" (
    "id" TEXT NOT NULL,
    "vault_address" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "token_id" TEXT NOT NULL,
    "tvl" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "current_apy" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "total_shares" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "status" "VaultStatus" NOT NULL DEFAULT 'active',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "vaults_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "strategies" (
    "id" TEXT NOT NULL,
    "vault_id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "strategy_type" TEXT NOT NULL,
    "risk_level" "RiskLevel" NOT NULL,
    "allocation_weight" DECIMAL(65,30) NOT NULL,
    "is_encrypted" BOOLEAN NOT NULL DEFAULT false,
    "program_id" TEXT NOT NULL,
    "pool_id" TEXT,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "strategies_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "encrypted_payloads" (
    "id" TEXT NOT NULL,
    "strategy_id" TEXT,
    "vault_id" TEXT NOT NULL,
    "payload_type" "PayloadType" NOT NULL,
    "encrypted_data" BYTEA NOT NULL,
    "encryption_pubkey" TEXT NOT NULL,
    "nonce" BIGINT NOT NULL,
    "commitment_hash" TEXT NOT NULL,
    "status" "PayloadStatus" NOT NULL DEFAULT 'pending',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expires_at" TIMESTAMP(3),

    CONSTRAINT "encrypted_payloads_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "orderbooks" (
    "id" TEXT NOT NULL,
    "orderbook_address" TEXT NOT NULL,
    "admin" TEXT NOT NULL,
    "token_id" TEXT NOT NULL,
    "quote_token_id" TEXT NOT NULL,
    "arcium_artifact_id" TEXT NOT NULL,
    "arcium_verification_key" TEXT NOT NULL,
    "arcium_mxe_public_key" TEXT NOT NULL,
    "escrow_authority_bump" INTEGER NOT NULL,
    "order_count" BIGINT NOT NULL DEFAULT 0,
    "status" "OrderbookStatus" NOT NULL DEFAULT 'active',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "orderbooks_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "orders" (
    "id" TEXT NOT NULL,
    "orderbook_id" TEXT NOT NULL,
    "owner" TEXT NOT NULL,
    "order_id" BIGINT NOT NULL,
    "encryption_pubkey" TEXT NOT NULL,
    "nonce" BIGINT NOT NULL,
    "escrow_amount" BIGINT NOT NULL,
    "status" "OrderStatus" NOT NULL DEFAULT 'pending',
    "order_encrypted" TEXT NOT NULL,
    "price_encrypted" TEXT NOT NULL,
    "size_encrypted" TEXT NOT NULL,
    "is_buy_encrypted" TEXT NOT NULL,
    "tx_signature" TEXT,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "orders_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "settlements" (
    "id" TEXT NOT NULL,
    "orderbook_id" TEXT NOT NULL,
    "order_a_id" TEXT NOT NULL,
    "order_b_id" TEXT NOT NULL,
    "settlement_hash" TEXT NOT NULL,
    "execution_price" BIGINT,
    "execution_size" BIGINT,
    "arcium_signature" TEXT NOT NULL,
    "relayer" TEXT NOT NULL,
    "tx_signature" TEXT,
    "settled_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "settlements_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "deposits" (
    "id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    "vault_id" TEXT NOT NULL,
    "token_id" TEXT NOT NULL,
    "amount" DECIMAL(65,30) NOT NULL,
    "shares_received" DECIMAL(65,30) NOT NULL,
    "tx_signature" TEXT NOT NULL,
    "status" "TxStatus" NOT NULL DEFAULT 'pending',
    "deposited_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "deposits_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "withdrawals" (
    "id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    "vault_id" TEXT NOT NULL,
    "token_id" TEXT NOT NULL,
    "amount" DECIMAL(65,30) NOT NULL,
    "shares_burned" DECIMAL(65,30) NOT NULL,
    "tx_signature" TEXT NOT NULL,
    "status" "TxStatus" NOT NULL DEFAULT 'pending',
    "withdrawn_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "withdrawals_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "positions" (
    "id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    "vault_id" TEXT NOT NULL,
    "shares" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "deposit_value" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "current_value" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "unrealized_pnl" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "avg_entry_price" DECIMAL(65,30) NOT NULL DEFAULT 0,
    "status" "PositionStatus" NOT NULL DEFAULT 'active',
    "last_rebalanced_at" TIMESTAMP(3),
    "opened_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "closed_at" TIMESTAMP(3),
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "positions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "rebalance_events" (
    "id" TEXT NOT NULL,
    "vault_id" TEXT NOT NULL,
    "strategy_changes" JSONB NOT NULL,
    "tx_signature" TEXT NOT NULL,
    "is_stealth" BOOLEAN NOT NULL DEFAULT false,
    "encrypted_payload_id" TEXT,
    "gas_used" BIGINT,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "rebalance_events_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "strategy_nfts" (
    "id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    "strategy_id" TEXT NOT NULL,
    "cnft_mint_address" TEXT NOT NULL,
    "metadata_uri" TEXT NOT NULL,
    "tx_signature" TEXT NOT NULL,
    "minted_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "strategy_nfts_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "apy_snapshots" (
    "id" TEXT NOT NULL,
    "vault_id" TEXT,
    "pool_id" TEXT,
    "apy_value" DECIMAL(65,30) NOT NULL,
    "source" TEXT NOT NULL,
    "recorded_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "apy_snapshots_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "cron_jobs" (
    "id" TEXT NOT NULL,
    "job_type" "CronJobType" NOT NULL,
    "target_entity_type" TEXT,
    "target_entity_id" TEXT,
    "frequency" TEXT NOT NULL,
    "status" "CronJobStatus" NOT NULL DEFAULT 'active',
    "last_run_at" TIMESTAMP(3),
    "next_run_at" TIMESTAMP(3),
    "last_error" TEXT,
    "run_count" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "cron_jobs_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "integration_logs" (
    "id" TEXT NOT NULL,
    "step_name" TEXT NOT NULL,
    "status" "LogStatus" NOT NULL,
    "error_message" TEXT,
    "latency_ms" INTEGER NOT NULL,
    "related_entity_type" TEXT,
    "related_entity_id" TEXT,
    "metadata" JSONB,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "integration_logs_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "users_wallet_address_key" ON "users"("wallet_address");

-- CreateIndex
CREATE UNIQUE INDEX "tokens_mint_address_key" ON "tokens"("mint_address");

-- CreateIndex
CREATE UNIQUE INDEX "pools_pool_address_key" ON "pools"("pool_address");

-- CreateIndex
CREATE UNIQUE INDEX "vaults_vault_address_key" ON "vaults"("vault_address");

-- CreateIndex
CREATE UNIQUE INDEX "orderbooks_orderbook_address_key" ON "orderbooks"("orderbook_address");

-- CreateIndex
CREATE INDEX "orders_orderbook_id_status_idx" ON "orders"("orderbook_id", "status");

-- CreateIndex
CREATE UNIQUE INDEX "settlements_settlement_hash_key" ON "settlements"("settlement_hash");

-- CreateIndex
CREATE INDEX "deposits_user_id_status_idx" ON "deposits"("user_id", "status");

-- CreateIndex
CREATE INDEX "withdrawals_user_id_status_idx" ON "withdrawals"("user_id", "status");

-- CreateIndex
CREATE UNIQUE INDEX "positions_user_id_vault_id_key" ON "positions"("user_id", "vault_id");

-- CreateIndex
CREATE UNIQUE INDEX "strategy_nfts_cnft_mint_address_key" ON "strategy_nfts"("cnft_mint_address");

-- CreateIndex
CREATE INDEX "apy_snapshots_vault_id_recorded_at_idx" ON "apy_snapshots"("vault_id", "recorded_at");

-- CreateIndex
CREATE INDEX "cron_jobs_job_type_status_idx" ON "cron_jobs"("job_type", "status");

-- CreateIndex
CREATE INDEX "integration_logs_step_name_status_created_at_idx" ON "integration_logs"("step_name", "status", "created_at");

-- AddForeignKey
ALTER TABLE "pools" ADD CONSTRAINT "pools_protocol_id_fkey" FOREIGN KEY ("protocol_id") REFERENCES "protocols"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pools" ADD CONSTRAINT "pools_token_a_id_fkey" FOREIGN KEY ("token_a_id") REFERENCES "tokens"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pools" ADD CONSTRAINT "pools_token_b_id_fkey" FOREIGN KEY ("token_b_id") REFERENCES "tokens"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "vaults" ADD CONSTRAINT "vaults_token_id_fkey" FOREIGN KEY ("token_id") REFERENCES "tokens"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "strategies" ADD CONSTRAINT "strategies_vault_id_fkey" FOREIGN KEY ("vault_id") REFERENCES "vaults"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "strategies" ADD CONSTRAINT "strategies_pool_id_fkey" FOREIGN KEY ("pool_id") REFERENCES "pools"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "encrypted_payloads" ADD CONSTRAINT "encrypted_payloads_strategy_id_fkey" FOREIGN KEY ("strategy_id") REFERENCES "strategies"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "encrypted_payloads" ADD CONSTRAINT "encrypted_payloads_vault_id_fkey" FOREIGN KEY ("vault_id") REFERENCES "vaults"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "orderbooks" ADD CONSTRAINT "orderbooks_token_id_fkey" FOREIGN KEY ("token_id") REFERENCES "tokens"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "orderbooks" ADD CONSTRAINT "orderbooks_quote_token_id_fkey" FOREIGN KEY ("quote_token_id") REFERENCES "tokens"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "orders" ADD CONSTRAINT "orders_orderbook_id_fkey" FOREIGN KEY ("orderbook_id") REFERENCES "orderbooks"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "settlements" ADD CONSTRAINT "settlements_orderbook_id_fkey" FOREIGN KEY ("orderbook_id") REFERENCES "orderbooks"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "settlements" ADD CONSTRAINT "settlements_order_a_id_fkey" FOREIGN KEY ("order_a_id") REFERENCES "orders"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "settlements" ADD CONSTRAINT "settlements_order_b_id_fkey" FOREIGN KEY ("order_b_id") REFERENCES "orders"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "deposits" ADD CONSTRAINT "deposits_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "deposits" ADD CONSTRAINT "deposits_vault_id_fkey" FOREIGN KEY ("vault_id") REFERENCES "vaults"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "deposits" ADD CONSTRAINT "deposits_token_id_fkey" FOREIGN KEY ("token_id") REFERENCES "tokens"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "withdrawals" ADD CONSTRAINT "withdrawals_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "withdrawals" ADD CONSTRAINT "withdrawals_vault_id_fkey" FOREIGN KEY ("vault_id") REFERENCES "vaults"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "withdrawals" ADD CONSTRAINT "withdrawals_token_id_fkey" FOREIGN KEY ("token_id") REFERENCES "tokens"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "positions" ADD CONSTRAINT "positions_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "positions" ADD CONSTRAINT "positions_vault_id_fkey" FOREIGN KEY ("vault_id") REFERENCES "vaults"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "rebalance_events" ADD CONSTRAINT "rebalance_events_vault_id_fkey" FOREIGN KEY ("vault_id") REFERENCES "vaults"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "rebalance_events" ADD CONSTRAINT "rebalance_events_encrypted_payload_id_fkey" FOREIGN KEY ("encrypted_payload_id") REFERENCES "encrypted_payloads"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "strategy_nfts" ADD CONSTRAINT "strategy_nfts_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "strategy_nfts" ADD CONSTRAINT "strategy_nfts_strategy_id_fkey" FOREIGN KEY ("strategy_id") REFERENCES "strategies"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "apy_snapshots" ADD CONSTRAINT "apy_snapshots_vault_id_fkey" FOREIGN KEY ("vault_id") REFERENCES "vaults"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "apy_snapshots" ADD CONSTRAINT "apy_snapshots_pool_id_fkey" FOREIGN KEY ("pool_id") REFERENCES "pools"("id") ON DELETE SET NULL ON UPDATE CASCADE;
