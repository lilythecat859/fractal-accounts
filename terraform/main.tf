terraform {
  required_version = ">= 1.5"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.region
}

variable "region" {
  default = "us-east-1"
}

variable "app_name" {
  default = "fractal-accounts"
}

data "aws_availability_zones" "available" {}

locals {
  azs = slice(data.aws_availability_zones.available.names, 0, 3)
}

module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "5.0.0"

  name                 = var.app_name
  cidr                 = "10.0.0.0/16"
  azs                  = local.azs
  private_subnets      = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets       = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]
  enable_nat_gateway   = true
  single_nat_gateway   = true
  enable_dns_hostnames = true
  enable_dns_support   = true
}

module "rds" {
  source  = "terraform-aws-modules/rds/aws"
  version = "6.0.0"

  identifier     = var.app_name
  engine         = "postgres"
  engine_version = "16"
  family         = "postgres16"
  instance_class = "db.t4g.micro"
  allocated_storage = 20
  db_name  = "fractal"
  username = "fractal"
  port     = 5432
  subnet_ids = module.vpc.private_subnets
  vpc_security_group_ids = [aws_security_group.rds.id]
  skip_final_snapshot = true
}

resource "aws_security_group" "rds" {
  name_prefix = "${var.app_name}-rds"
  vpc_id      = module.vpc.vpc_id
  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = [module.vpc.vpc_cidr_block]
  }
}

module "ecs_cluster" {
  source  = "terraform-aws-modules/ecs/aws"
  version = "5.0.0"

  cluster_name = var.app_name
  fargate_capacity_providers = ["FARGATE", "FARGATE_SPOT"]
}

output "vpc_id" { value = module.vpc.vpc_id }
output "subnet_ids" { value = module.vpc.private_subnets }
output "cluster_id" { value = module.ecs_cluster.cluster_id }
output "rds_endpoint" { value = module.rds.db_instance_endpoint }
