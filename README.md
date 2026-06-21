# Sentinel

A global honeypot service.

Check out the approach at [APPROACH.md](./APPROACH.md)

## Architecture:

```text
                API Gateway
                      |
             Threat Intelligence
                      |
    ---------------------------------------
    |                |                    |
 Query Service   Reputation Engine   Alert Engine
    |                |                    |
    ---------------------------------------
                      |
                    Kafka
                      |
    ---------------------------------------
    |          |          |              |
 Collector  Aggregator  Detection   Analytics
                      |
                  PostgreSQL
                      |
                Read Replicas
```
