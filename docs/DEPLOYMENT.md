# 🚀 Deployment Guide - Sonic SVM + Pyth GDP Dashboard

> **Production-Ready Deployment Instructions**  
> Deploy your enterprise-grade economic data platform with confidence

## 📋 **Overview**

This guide covers deploying the Sonic SVM + Pyth GDP Dashboard in various environments, from local development to production cloud deployments.

---

## 🏠 **Local Development**

### **Prerequisites**
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git
- 4GB+ RAM available

### **Quick Start**
```bash
# Clone repository
git clone https://github.com/yourusername/sonic-svm-gdp-dashboard.git
cd sonic-svm-gdp-dashboard

# Build and run
cargo build --release
cargo run --release -- --port 3000

# Access dashboard
open http://localhost:3000
```

### **Environment Variables**
```bash
export RUST_LOG=info
export RUST_BACKTRACE=1
export PORT=3000
```

---

## 🐳 **Docker Deployment**

### **Single Container**
```bash
# Build image
docker build -t sonic-gdp-dashboard .

# Run container
docker run -d \
  --name sonic-gdp-dashboard \
  -p 3000:3000 \
  -e RUST_LOG=info \
  sonic-gdp-dashboard

# Verify deployment
curl http://localhost:3000/health
```

### **Docker Compose (Recommended)**
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f sonic-gdp-dashboard

# Scale if needed
docker-compose up -d --scale sonic-gdp-dashboard=3
```

### **Production Docker Compose**
```bash
# Use production configuration
docker-compose -f docker-compose.prod.yml up -d

# With custom environment
docker-compose -f docker-compose.prod.yml \
  -e POSTGRES_PASSWORD=your_secure_password \
  -e GRAFANA_PASSWORD=your_admin_password \
  up -d
```

---

## ☁️ **Cloud Deployment**

### **AWS EC2 Deployment**

#### **1. Launch EC2 Instance**
```bash
# Launch Ubuntu 22.04 LTS instance
# Instance type: t3.medium (2 vCPU, 4GB RAM)
# Security group: Allow ports 22 (SSH), 80 (HTTP), 443 (HTTPS), 3000 (API)
```

#### **2. Install Dependencies**
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

#### **3. Deploy Application**
```bash
# Clone repository
git clone https://github.com/yourusername/sonic-svm-gdp-dashboard.git
cd sonic-svm-gdp-dashboard

# Start services
docker-compose up -d

# Verify deployment
curl http://localhost:3000/health
```

#### **4. Configure Nginx (Optional)**
```bash
# Install Nginx
sudo apt install nginx -y

# Create Nginx configuration
sudo tee /etc/nginx/sites-available/sonic-gdp <<EOF
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
EOF

# Enable site
sudo ln -s /etc/nginx/sites-available/sonic-gdp /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

### **Google Cloud Platform (GCP)**

#### **1. Create Compute Engine Instance**
```bash
# Create instance
gcloud compute instances create sonic-gdp-dashboard \
  --zone=us-central1-a \
  --machine-type=e2-medium \
  --image-family=ubuntu-2204-lts \
  --image-project=ubuntu-os-cloud \
  --tags=http-server,https-server

# Allow HTTP/HTTPS traffic
gcloud compute firewall-rules create allow-http \
  --allow tcp:80,tcp:443,tcp:3000 \
  --target-tags=http-server \
  --source-ranges=0.0.0.0/0
```

#### **2. Deploy Application**
```bash
# SSH into instance
gcloud compute ssh sonic-gdp-dashboard --zone=us-central1-a

# Follow same deployment steps as AWS EC2
```

### **DigitalOcean Droplet**

#### **1. Create Droplet**
```bash
# Create droplet via DigitalOcean console
# Image: Ubuntu 22.04 LTS
# Size: Basic (2GB RAM, 1 vCPU)
# Region: Choose closest to your users
```

#### **2. Deploy Application**
```bash
# SSH into droplet
ssh root@your-droplet-ip

# Follow same deployment steps as AWS EC2
```

---

## 🚀 **Production Deployment**

### **System Requirements**
- **CPU:** 2+ vCPUs
- **RAM:** 4GB+ (8GB recommended)
- **Storage:** 20GB+ SSD
- **Network:** 100Mbps+ bandwidth

### **Security Configuration**

#### **1. Firewall Setup**
```bash
# UFW firewall configuration
sudo ufw allow ssh
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 3000/tcp
sudo ufw enable
```

#### **2. SSL/TLS Configuration**
```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx -y

# Obtain SSL certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

#### **3. Environment Security**
```bash
# Create secure environment file
sudo tee /opt/sonic-gdp/.env <<EOF
RUST_LOG=info
POSTGRES_PASSWORD=your_very_secure_password_here
GRAFANA_PASSWORD=your_admin_password_here
EOF

# Set proper permissions
sudo chmod 600 /opt/sonic-gdp/.env
sudo chown root:root /opt/sonic-gdp/.env
```

### **Monitoring & Observability**

#### **1. Prometheus Configuration**
```yaml
# monitoring/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'sonic-gdp-dashboard'
    static_configs:
      - targets: ['localhost:3000']
    metrics_path: '/metrics'
    scrape_interval: 5s
```

#### **2. Grafana Dashboard**
```json
{
  "dashboard": {
    "title": "Sonic GDP Dashboard Metrics",
    "panels": [
      {
        "title": "API Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_request_duration_seconds_sum[5m])"
          }
        ]
      }
    ]
  }
}
```

#### **3. Log Aggregation**
```bash
# Configure log rotation
sudo tee /etc/logrotate.d/sonic-gdp <<EOF
/opt/sonic-gdp/logs/*.log {
    daily
    missingok
    rotate 52
    compress
    delaycompress
    notifempty
    create 644 appuser appuser
}
EOF
```

---

## 🔄 **CI/CD Pipeline**

### **GitHub Actions Workflow**
```yaml
# .github/workflows/deploy.yml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build Docker image
        run: |
          docker build -t sonic-gdp-dashboard .
          docker tag sonic-gdp-dashboard:latest ${{ secrets.REGISTRY }}/sonic-gdp-dashboard:latest
      
      - name: Deploy to server
        run: |
          ssh ${{ secrets.SSH_USER }}@${{ secrets.SSH_HOST }} << 'EOF'
            cd /opt/sonic-gdp
            docker-compose pull
            docker-compose up -d
          EOF
```

### **Automated Testing**
```bash
# Run tests before deployment
cargo test
cargo clippy
cargo fmt --check

# Integration tests
cargo test --test integration_tests
```

---

## 📊 **Performance Optimization**

### **1. Database Optimization**
```sql
-- PostgreSQL performance tuning
ALTER SYSTEM SET shared_buffers = '1GB';
ALTER SYSTEM SET effective_cache_size = '3GB';
ALTER SYSTEM SET maintenance_work_mem = '256MB';
ALTER SYSTEM SET checkpoint_completion_target = 0.9;
ALTER SYSTEM SET wal_buffers = '16MB';
ALTER SYSTEM SET default_statistics_target = 100;
```

### **2. Caching Strategy**
```rust
// Redis caching implementation
use redis::{Client, Commands};

pub async fn get_cached_gdp_data(&self, symbol: &str) -> Option<PriceData> {
    let mut conn = self.redis_client.get_async_connection().await.ok()?;
    let cached: Option<String> = conn.get(&format!("gdp:{}", symbol)).await.ok()?;
    
    cached.and_then(|data| serde_json::from_str(&data).ok())
}
```

### **3. Load Balancing**
```nginx
# Nginx load balancer configuration
upstream sonic_gdp_backend {
    least_conn;
    server 127.0.0.1:3000 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:3001 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:3002 max_fails=3 fail_timeout=30s;
}

server {
    listen 80;
    location / {
        proxy_pass http://sonic_gdp_backend;
    }
}
```

---

## 🚨 **Troubleshooting**

### **Common Issues**

#### **1. Application Won't Start**
```bash
# Check logs
docker-compose logs sonic-gdp-dashboard

# Verify dependencies
cargo check

# Check port availability
sudo netstat -tlnp | grep :3000
```

#### **2. Performance Issues**
```bash
# Monitor resource usage
htop
iotop
nethogs

# Check API response times
curl -w "@curl-format.txt" -o /dev/null -s "http://localhost:3000/gdp"
```

#### **3. Blockchain Connection Issues**
```bash
# Test Sonic SVM connectivity
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getVersion"}' \
  https://rpc.mainnet-alpha.sonic.game

# Test Solana RPC
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getVersion"}' \
  https://mainnet.helius-rpc.com
```

### **Debug Commands**
```bash
# Enable debug logging
export RUST_LOG=debug
export RUST_BACKTRACE=1

# Run with verbose output
cargo run -- --port 3000 --verbose

# Check system resources
free -h
df -h
systemctl status sonic-gdp-dashboard
```

---

## 📈 **Scaling Strategies**

### **Horizontal Scaling**
```yaml
# docker-compose.scale.yml
services:
  sonic-gdp-dashboard:
    deploy:
      replicas: 5
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
        reservations:
          cpus: '0.25'
          memory: 256M
```

### **Vertical Scaling**
```bash
# Upgrade instance type (AWS)
aws ec2 modify-instance-attribute \
  --instance-id i-1234567890abcdef0 \
  --instance-type "{\"Value\": \"t3.large\"}"

# Scale Docker resources
docker update --cpus 2 --memory 4g sonic-gdp-dashboard
```

---

## 🔒 **Security Best Practices**

### **1. Network Security**
- Use VPC/private subnets
- Implement security groups/firewall rules
- Enable DDoS protection
- Use VPN for administrative access

### **2. Application Security**
- Regular security updates
- Input validation and sanitization
- Rate limiting implementation
- API key authentication

### **3. Data Security**
- Encrypt data at rest
- Secure API communications (HTTPS)
- Regular backup procedures
- Access control and logging

---

## 📞 **Support & Maintenance**

### **Monitoring Checklist**
- [ ] API response times < 100ms
- [ ] Error rate < 1%
- [ ] Uptime > 99.9%
- [ ] Memory usage < 80%
- [ ] CPU usage < 70%

### **Maintenance Schedule**
- **Daily:** Log review and error monitoring
- **Weekly:** Performance metrics analysis
- **Monthly:** Security updates and dependency updates
- **Quarterly:** Capacity planning and scaling review

### **Emergency Procedures**
1. **Service Down:** Check logs, restart service, scale if needed
2. **Performance Degradation:** Enable caching, optimize queries
3. **Security Breach:** Isolate affected systems, audit logs, update credentials

---

## 🎯 **Next Steps**

1. **Deploy to staging environment** for testing
2. **Set up monitoring and alerting**
3. **Configure backup and disaster recovery**
4. **Implement CI/CD pipeline**
5. **Plan for production scaling**

---

**Need Help?**  
- **Documentation:** [Project Wiki](https://github.com/yourusername/sonic-svm-gdp-dashboard/wiki)
- **Issues:** [GitHub Issues](https://github.com/yourusername/sonic-svm-gdp-dashboard/issues)
- **Community:** [Discord Server](https://discord.gg/your-invite)

---

**Built with ❤️ by the Sonic SVM + Pyth Network Team**

*Empowering the future of decentralized economic data infrastructure*
