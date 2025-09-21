 fractal-accounts/README.md 

 
# fractal-accounts
Cloud-native micro-services for user & account management.

## quick start
```bash
docker compose up --build
 

auth-svc :8080
user-svc :8081
grafana  :3000


## stack
- Rust + Tokio + gRPC / REST
- Postgres (RDS)
- ECS Fargate / k8s
- Grafana + Loki + Prometheus
- CI/CD via GitHub Actions

## quick start
```bash
git clone <repo>
cd fractal-accounts
docker compose up --build

 

-------------------------------------------------
Next message: `terraform/`, `helm/`, `.github/workflows/`, and the three service crates (auth, user, ledger) with full proto definitions, tests, and Grafana dashboards.
 
