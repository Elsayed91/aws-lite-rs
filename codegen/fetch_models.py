"""Download botocore service-2.json models for AWS codegen.

Usage:
    python3 codegen/fetch_models.py monitoring
    python3 codegen/fetch_models.py logs
    python3 codegen/fetch_models.py --all
"""

from __future__ import annotations

import json
import sys
import urllib.request
from pathlib import Path

BOTOCORE_BASE = (
    "https://raw.githubusercontent.com/boto/botocore/develop/botocore/data"
)

# Map from our API names to botocore service directories + API versions.
SERVICE_MAP: dict[str, tuple[str, str]] = {
    "cloudwatch": ("cloudwatch", "2010-08-01"),
    "config": ("config", "2014-11-12"),
    "iam": ("iam", "2010-05-08"),
    "logs": ("logs", "2014-03-28"),
    "s3": ("s3", "2006-03-01"),
    "sts": ("sts", "2011-06-15"),
    "autoscaling": ("autoscaling", "2011-01-01"),
    "ec2": ("ec2", "2016-11-15"),
    "cloudfront": ("cloudfront", "2020-05-31"),
    "rds": ("rds", "2014-10-31"),
    "elasticache": ("elasticache", "2015-02-02"),
    "redshift": ("redshift", "2012-12-01"),
    "elbv2": ("elbv2", "2015-12-01"),
    "dynamodb": ("dynamodb", "2012-08-10"),
    "ecr": ("ecr", "2015-09-21"),
    "ecs": ("ecs", "2014-11-13"),
    "cloudtrail": ("cloudtrail", "2013-11-01"),
    "kms": ("kms", "2014-11-01"),
    "secretsmanager": ("secretsmanager", "2017-10-17"),
    "ce": ("ce", "2017-10-25"),
    "kinesis": ("kinesis", "2013-12-02"),
    "sagemaker": ("sagemaker", "2017-07-24"),
    "emr": ("emr", "2009-03-31"),
    "lambda": ("lambda", "2015-03-31"),
    "eks": ("eks", "2017-11-01"),
    "apigateway": ("apigateway", "2015-07-09"),
    "opensearch": ("opensearch", "2021-01-01"),
    "route53": ("route53", "2013-04-01"),
    "organizations": ("organizations", "2016-11-28"),
    "accessanalyzer": ("accessanalyzer", "2019-11-01"),
    "securityhub": ("securityhub", "2018-10-26"),
    "efs": ("efs", "2015-02-01"),
}

CACHE_DIR = Path(__file__).parent / "model_cache"


def fetch_model(service_name: str) -> Path:
    """Download a botocore service-2.json model to the cache directory."""
    if service_name not in SERVICE_MAP:
        print(f"ERROR: Unknown service '{service_name}'. Known: {list(SERVICE_MAP.keys())}")
        sys.exit(1)

    botocore_dir, api_version = SERVICE_MAP[service_name]
    url = f"{BOTOCORE_BASE}/{botocore_dir}/{api_version}/service-2.json"
    cache_file = CACHE_DIR / f"{service_name}.service-2.json"

    if cache_file.exists():
        print(f"  Using cached model: {cache_file}")
        return cache_file

    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    print(f"  Downloading {url} ...")
    urllib.request.urlretrieve(url, str(cache_file))

    # Verify it's valid JSON
    with open(cache_file) as f:
        json.load(f)
    print(f"  Cached to: {cache_file}")
    return cache_file


def main() -> None:
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    if sys.argv[1] == "--all":
        for name in SERVICE_MAP:
            fetch_model(name)
    else:
        for name in sys.argv[1:]:
            fetch_model(name)


if __name__ == "__main__":
    main()
