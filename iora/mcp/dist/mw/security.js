import crypto from "crypto";
import rateLimit from "express-rate-limit";
export const limiter = rateLimit({
    windowMs: 10_000,
    max: 30, // 30 req / 10s for general endpoints
    message: { ok: false, error: "rate_limit_exceeded" }
});
export const oracleLimiter = rateLimit({
    windowMs: 60_000, // 1 minute
    max: 3, // 3 oracle feeds per minute
    message: { ok: false, error: "oracle_rate_limit_exceeded" }
});
export function hmacAuth(req, res, next) {
    const secret = process.env.CORAL_SHARED_SECRET;
    if (!secret)
        return res.status(500).json({ ok: false, error: "server_not_configured" });
    const sig = req.header("x-iora-signature");
    const body = JSON.stringify(req.body || {});
    const digest = crypto.createHmac("sha256", secret).update(body).digest("hex");
    if (!sig || sig !== digest)
        return res.status(401).json({ ok: false, error: "unauthorized" });
    next();
}
export function shield(err, _req, res, _next) {
    const msg = (err?.message || "internal_error").slice(0, 300);
    res.status(400).json({ ok: false, error: msg });
}
